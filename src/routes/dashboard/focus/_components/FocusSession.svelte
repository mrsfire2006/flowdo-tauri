<script lang="ts">
  import { onMount, untrack } from "svelte";
  import { fade } from "svelte/transition";
  import {
    Play,
    Pause,
    Check,
    X,
    Minus,
    Plus,
    Timer,
    Flag,
    LoaderCircle,
  } from "@lucide/svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import type { TaskCard, UpdateTaskRequest } from "$lib/shared-types/task";
  import { getTasksContext } from "$lib/components/contexts/tasks/tasks.context.svelte";
  import { notifyTaskCompleted } from "$lib/components/contexts/tasks/tasks.store.svelte";
  import { prepareCompletionSound } from "$lib/completion-sound";
  import AlertMessage from "$lib/components/shared/AlertMessage.svelte";

  let {
    task,
    onexit,
  }: {
    task: TaskCard;
    /** Called by "Exit focus" and after the task is completed. */
    onexit: () => void;
  } = $props();

  const store = getTasksContext();

  type Status = "ready" | "running" | "paused" | "done";

  const DEFAULT_MINUTES = 25;
  const MIN_MINUTES = 5;
  const MAX_MINUTES = 180;
  const OVERTIME_BLOCK = 5 * 60; // "Add 5 minutes" adds one block

  /* ---- restore saved progress (focusStartedAt / focusElapsedSeconds) ---- */

  const restored = untrack(() => {
    const planned = (task.durationMinutes || DEFAULT_MINUTES) * 60;
    const saved = task.focusElapsedSeconds ?? 0;
    const startedMs = task.focusStartedAt
      ? new Date(task.focusStartedAt).getTime()
      : NaN;
    const wasRunning = Number.isFinite(startedMs);

    // If the timer was running, add the time that passed since it started.
    // Before the planned time ends it is capped at the planned time; during an
    // "Add 5 minutes" block it is capped at one more block.
    let elapsed = saved;
    if (wasRunning) {
      const passed = Math.max(0, Math.floor((Date.now() - startedMs) / 1000));
      const limit = saved < planned ? planned : saved + OVERTIME_BLOCK;
      elapsed = Math.min(saved + passed, limit);
    }

    // Time beyond the plan was added in 5-minute blocks.
    const overtime =
      elapsed > planned
        ? Math.ceil((elapsed - planned) / OVERTIME_BLOCK) * OVERTIME_BLOCK
        : 0;

    let status: Status = "ready";
    if (elapsed >= planned + overtime) status = "done";
    else if (wasRunning) status = "running";
    else if (elapsed > 0) status = "paused";

    return {
      status,
      elapsed,
      saved,
      overtime,
      wasRunning,
      startedMs: wasRunning ? startedMs : 0,
    };
  });

  let status = $state<Status>(restored.status);
  let plannedMinutes = $state(
    untrack(() => task.durationMinutes || DEFAULT_MINUTES),
  );
  let overtimeSec = $state(restored.overtime); // not saved as the estimate
  let elapsed = $state(restored.elapsed);
  let startedAt = restored.startedMs; // when a saved running session started
  let elapsedAtStart = restored.saved;
  let isStarting = $state(false);
  let isExiting = $state(false);
  let errorMsg = $state("");
  let root = $state<HTMLDivElement>();

  const title = $derived(task.title || "Untitled task");
  const totalSec = $derived(plannedMinutes * 60 + overtimeSec);
  const remaining = $derived(Math.max(0, totalSec - elapsed));
  const fraction = $derived(totalSec > 0 ? remaining / totalSec : 0);
  const percentDone = $derived(Math.round((1 - fraction) * 100));
  const mm = $derived(String(Math.floor(remaining / 60)).padStart(2, "0"));
  const ss = $derived(String(remaining % 60).padStart(2, "0"));
  const isSaving = $derived(store.updateTask.isPending);
  const priorityLabel = $derived(String(task.priority ?? "").toLowerCase());

  const R = 92;
  const C = 2 * Math.PI * R;

  const statusLabel = $derived(
    {
      ready: "Ready when you are",
      running: "Focus in progress",
      paused: "Paused",
      done: "Time's up",
    }[status],
  );

  function tick() {
    elapsed = elapsedAtStart + Math.floor((Date.now() - startedAt) / 1000);
    if (elapsed >= totalSec) {
      elapsed = totalSec;
      if (status !== "done") {
        status = "done";
        void persistProgress();
        void notifyTaskCompleted(task.id);
      }
    }
  }

  $effect(() => {
    const previous = document.body.style.overflow;
    document.body.style.overflow = "hidden";
    root?.focus();
    return () => (document.body.style.overflow = previous);
  });

  $effect(() => {
    if (status !== "running") return;
    const id = setInterval(tick, 250);
    return () => clearInterval(id);
  });

  onMount(() => {
    prepareCompletionSound();
    // Catch up if the app was suspended/minimized past the end of the session.
    if (restored.wasRunning && restored.status === "done") {
      void persistProgress();
      void notifyTaskCompleted(task.id);
    }

    const catchUpWhenVisible = () => {
      if (document.visibilityState === "visible" && status === "running")
        tick();
    };
    document.addEventListener("visibilitychange", catchUpWhenVisible);
    window.addEventListener("focus", catchUpWhenVisible);
    return () => {
      document.removeEventListener("visibilitychange", catchUpWhenVisible);
      window.removeEventListener("focus", catchUpWhenVisible);
    };
  });

  /* ---- store actions ---- */

  /** Saves the elapsed time and clears focusStartedAt (timer is not running). */
  async function persistProgress(): Promise<boolean> {
    try {
      const result = await store.updateTask.mutateAsync({
        id: task.id,
        task: { focusStartedAt: null, focusElapsedSeconds: elapsed },
      });
      if (result.isSuccess) return true;
      errorMsg = result.errorMsg ?? "Couldn't save your progress.";
    } catch {
      errorMsg = "Couldn't save your progress. Check your connection.";
    }
    return false;
  }

  /**
   * Saves what changed before the timer starts, in a single PATCH:
   * - focusStartedAt / focusElapsedSeconds, so the session survives a refresh
   * - an Inbox task becomes "In progress"
   * - a changed session length is saved as the task's estimate
   * Returns false (and sets errorMsg) if saving fails, so the timer is not started.
   */
  async function saveBeforeStart(startTime: Date): Promise<boolean> {
    const payload: UpdateTaskRequest = {
      focusStartedAt: startTime,
      focusElapsedSeconds: elapsed,
    };
    if (task.status === "INBOX") payload.status = "IN_PROGRESS";

    if (plannedMinutes !== (task.durationMinutes || DEFAULT_MINUTES)) {
      payload.estimatedMinutes = plannedMinutes;
    }

    try {
      const result = await store.updateTask.mutateAsync({
        id: task.id,
        task: payload,
      });
      if (result.isSuccess) return true;
      errorMsg = result.errorMsg ?? "Couldn't start the session. Try again.";
    } catch {
      errorMsg =
        "Couldn't start the session. Check your connection and try again.";
    }
    return false;
  }

  /**
   * Marks the task as done. focusElapsedSeconds is kept (it's the time actually spent,
   * useful to compare with the estimate); only focusStartedAt is cleared.
   */
  async function complete() {
    errorMsg = "";
    if (status === "running") tick();

    try {
      const result = await store.updateTask.mutateAsync({
        id: task.id,
        task: { status: "DONE", focusStartedAt: null, focusElapsedSeconds: 0 },
      });
      if (result.isSuccess) {
        onexit();
      } else {
        errorMsg = result.errorMsg ?? "Couldn't complete this task. Try again.";
      }
    } catch {
      errorMsg =
        "Couldn't complete this task. Check your connection and try again.";
    }
  }

  /* ---- timer controls ---- */

  async function start() {
    if (isStarting) return;
    prepareCompletionSound(true);
    errorMsg = "";
    const startTime = new Date();

    isStarting = true;
    const saved = await saveBeforeStart(startTime);
    isStarting = false;
    if (!saved) return; // nothing was saved, so the timer does not start

    // Same time that was saved in the DB, so a refresh resumes at the same point.
    startedAt = startTime.getTime();
    elapsedAtStart = elapsed;
    status = "running";
  }

  async function pause() {
    tick();
    if (status !== "running") return;
    status = "paused";
    await persistProgress();
  }

  /** Pauses first (so the DB is not left "running"), then closes the overlay. */
  async function exit() {
    if (isExiting) return;

    isExiting = true;
    if (status === "running") await pause();

    isExiting = false;
    onexit();
  }

  function adjust(deltaMin: number) {
    if (status !== "ready" || isStarting) return;
    plannedMinutes = Math.min(
      MAX_MINUTES,
      Math.max(MIN_MINUTES, plannedMinutes + deltaMin),
    );
  }

  function extend() {
    overtimeSec += OVERTIME_BLOCK;
    start();
  }
</script>

<div
  bind:this={root}
  role="dialog"
  aria-modal="true"
  aria-label="Focus session: {title}"
  tabindex="-1"
  transition:fade|global={{ duration: 200 }}
  class="fixed inset-0 z-50 flex flex-col overflow-y-auto bg-cream text-navy outline-none"
  style="padding-top: env(safe-area-inset-top); padding-bottom: env(safe-area-inset-bottom);"
>
  <!-- ambient glow -->
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-0 overflow-hidden"
  >
    <div
      class="absolute -top-40 -left-40 size-112 rounded-full bg-orange/10 blur-3xl transition-opacity duration-700 {status ===
      'running'
        ? 'opacity-100'
        : 'opacity-50'}"
    ></div>
    <div
      class="absolute -right-40 -bottom-40 size-120 rounded-full bg-coral/10 blur-3xl transition-opacity duration-700 {status ===
      'running'
        ? 'opacity-100'
        : 'opacity-40'}"
    ></div>
  </div>

  <!-- top bar -->
  <header
    class="relative flex items-center justify-between gap-4 px-4 py-4 sm:px-8 sm:py-6"
  >
    <span class="text-lg font-bold tracking-tight">FlowDo</span>

    <p
      class="hidden items-center gap-2 text-xs text-muted sm:flex"
      role="status"
      aria-live="polite"
    >
      <span
        class="size-1.5 rounded-full motion-reduce:animate-none {status ===
        'running'
          ? 'animate-pulse bg-coral'
          : 'bg-muted/60'}"
      ></span>
      {statusLabel}
    </p>

    <Button
      onclick={exit}
      disabled={isExiting}
      variant="outline"
      size="sm"
      class="border-line bg-surface/80 text-muted hover:text-navy"
    >
      <X class="size-3.5" />
      Exit focus
    </Button>
  </header>

  <!-- main -->
  <main
    class="relative mx-auto flex w-full max-w-2xl flex-1 flex-col items-center justify-center px-4 py-6 text-center sm:px-6"
  >
    <p class="mb-3 text-xs text-muted sm:hidden">{statusLabel}</p>

    <h1 class="text-3xl font-semibold tracking-tight text-balance sm:text-5xl">
      {title}
    </h1>
    {#if task.description}
      <p class="mt-3 line-clamp-3 max-w-md text-sm text-muted sm:text-base">
        {task.description}
      </p>
    {/if}

    <!-- timer ring -->
    <div
      class="relative mt-8 aspect-square w-[min(74vw,20rem)] sm:mt-10 sm:w-88"
    >
      <svg
        viewBox="0 0 200 200"
        class="size-full -rotate-90 transition-[filter] duration-700 {status ===
        'running'
          ? 'drop-shadow-[0_0_18px_rgb(233_120_98/0.35)]'
          : ''}"
        aria-hidden="true"
      >
        <circle
          cx="100"
          cy="100"
          r={R}
          fill="none"
          class="stroke-line"
          stroke-width="5"
        />
        <circle
          cx="100"
          cy="100"
          r={R}
          fill="none"
          class="stroke-coral transition-[stroke-dashoffset] duration-300 ease-linear motion-reduce:transition-none {status ===
          'paused'
            ? 'opacity-50'
            : ''}"
          stroke-width="5"
          stroke-linecap="round"
          stroke-dasharray={C}
          stroke-dashoffset={C * (1 - fraction)}
        />
      </svg>

      <div class="absolute inset-0 flex flex-col items-center justify-center">
        <p
          class="text-6xl font-light tracking-tight tabular-nums sm:text-7xl"
          role="timer"
          aria-label="{mm} minutes {ss} seconds remaining"
        >
          {mm}<span class="text-orange/70">:</span><span class="text-navy/70"
            >{ss}</span
          >
        </p>
        <p class="mt-2 text-xs text-muted">
          {status === "ready" ? "session length" : "remaining"}
        </p>

        {#if status === "ready"}
          <div class="mt-4 flex items-center gap-3">
            <Button
              variant="outline"
              size="icon"
              class="size-8 rounded-full border-line bg-surface text-muted hover:text-navy"
              onclick={() => adjust(-5)}
              disabled={plannedMinutes <= MIN_MINUTES || isStarting}
              aria-label="Shorten session by 5 minutes"
            >
              <Minus class="size-4" />
            </Button>
            <Button
              variant="outline"
              size="icon"
              class="size-8 rounded-full border-line bg-surface text-muted hover:text-navy"
              onclick={() => adjust(5)}
              disabled={plannedMinutes >= MAX_MINUTES || isStarting}
              aria-label="Extend session by 5 minutes"
            >
              <Plus class="size-4" />
            </Button>
          </div>
        {/if}
      </div>
    </div>

    <!-- actions -->
    <div
      class="mt-8 flex w-full max-w-xs flex-col gap-3 sm:mt-10 sm:max-w-none sm:flex-row sm:justify-center"
    >
      {#if status === "ready"}
        <Button
          size="lg"
          class="bg-coral px-8 font-semibold text-white shadow-lg shadow-coral/25 hover:bg-coral/90"
          onclick={start}
          disabled={isStarting}
        >
          {#if isStarting}
            <LoaderCircle
              class="size-4 animate-spin motion-reduce:animate-none"
            />
          {:else}
            <Play class="size-4 fill-current" />
          {/if}
          Start timer
        </Button>
      {:else if status === "running"}
        <Button
          variant="outline"
          size="lg"
          class="border-line bg-surface"
          onclick={pause}
        >
          <Pause class="size-4" />
          Pause
        </Button>
      {:else if status === "paused"}
        <Button
          size="lg"
          class="bg-coral font-semibold text-white shadow-lg shadow-coral/25 hover:bg-coral/90"
          onclick={start}
          disabled={isStarting}
        >
          {#if isStarting}
            <LoaderCircle
              class="size-4 animate-spin motion-reduce:animate-none"
            />
          {:else}
            <Play class="size-4 fill-current" />
          {/if}
          Resume
        </Button>
      {:else}
        <Button
          variant="outline"
          size="lg"
          class="border-line bg-surface"
          onclick={extend}
          disabled={isStarting}
        >
          <Plus class="size-4" />
          Add 5 minutes
        </Button>
      {/if}

      {#if status !== "ready"}
        <Button
          size="lg"
          variant={status === "done" ? "default" : "outline"}
          class="font-semibold {status === 'done'
            ? 'bg-coral text-white shadow-lg shadow-coral/25 hover:bg-coral/90'
            : 'border-coral/40 bg-coral/10 text-coral hover:bg-coral/15 hover:text-coral'}"
          onclick={complete}
          disabled={isSaving}
        >
          {#if isSaving}
            <LoaderCircle
              class="size-4 animate-spin motion-reduce:animate-none"
            />
          {:else}
            <Check class="size-4" />
          {/if}
          Complete task
        </Button>
      {/if}
    </div>

    {#if errorMsg}
      <AlertMessage message={errorMsg} type="error" />
    {/if}
  </main>

  <!-- footer -->
  <footer
    class="relative flex flex-wrap items-center justify-between gap-x-6 gap-y-2 px-4 py-4 text-xs text-muted sm:px-8 sm:py-6"
  >
    <span class="inline-flex items-center gap-2">
      <Timer class="size-3.5" />
      {Math.round(totalSec / 60)} min session
    </span>
    <span class="inline-flex items-center gap-2 capitalize">
      <Flag class="size-3.5" />
      {priorityLabel} priority
    </span>
    <span class="tabular-nums">{percentDone}% done</span>
  </footer>
</div>
