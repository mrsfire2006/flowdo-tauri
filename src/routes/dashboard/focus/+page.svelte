<script lang="ts">
	import { onMount } from 'svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import * as RadioGroup from '$lib/components/ui/radio-group/index.js';
	import type { TaskCard } from '$lib/shared-types/task';
	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';
	import FocusSession from './_components/FocusSession.svelte';

	const store = getTasksContext();

	/* ---------------- focus ranking ---------------- */

	const FOCUS_LIMIT = 3;
	const DEFAULT_MINUTES = 25;
	const ACTIVE_FOCUS_TASK_KEY = 'flowdo:active-focus-task';

	const PRIORITY = {
		HIGH: { rank: 0, label: 'High', dot: 'bg-coral', chip: 'bg-coral/10 text-coral' },
		MEDIUM: { rank: 1, label: 'Medium', dot: 'bg-orange', chip: 'bg-orange/10 text-orange' },
		LOW: { rank: 2, label: 'Low', dot: 'bg-mint', chip: 'bg-mint/10 text-mint' }
	} as const;

	function priorityOf(task: TaskCard) {
		const key = String(task.priority ?? '').toUpperCase() as keyof typeof PRIORITY;
		return PRIORITY[key] ?? PRIORITY.LOW;
	}

	const minutesOf = (task: TaskCard) => task.durationMinutes || DEFAULT_MINUTES;
	const titleOf = (task: TaskCard) => task.title || 'Untitled task';

	/** priority → already in progress → shortest task */
	function rankForFocus(tasks: TaskCard[]) {
		return [...tasks].sort((a, b) => {
			const byPriority = priorityOf(a).rank - priorityOf(b).rank;
			if (byPriority) return byPriority;

			const byActive = Number(b.status === 'IN_PROGRESS') - Number(a.status === 'IN_PROGRESS');
			if (byActive) return byActive;

			return minutesOf(a) - minutesOf(b);
		});
	}

	const focusTasks = $derived(
		rankForFocus(store.tasks.filter((t) => t.status !== 'DONE')).slice(0, FOCUS_LIMIT)
	);
	const totalMinutes = $derived(focusTasks.reduce((sum, t) => sum + minutesOf(t), 0));

	/* ---------------- selection + session overlay ---------------- */

	// Defaults to the highest-priority task; the user can pick another one.
	let pickedId = $state('');
	const selected = $derived(focusTasks.find((t) => t.id === pickedId) ?? focusTasks[0]);

	// The session opens as a full-screen overlay on this same page (no navigation).
	let activeId = $state<string | null>(null);
	const activeTask = $derived(activeId ? store.tasks.find((t) => t.id === activeId) : undefined);

	function start() {
		if (selected) {
			activeId = selected.id;
			localStorage.setItem(ACTIVE_FOCUS_TASK_KEY, selected.id);
		}
	}

	function closeSession() {
		activeId = null;
		localStorage.removeItem(ACTIVE_FOCUS_TASK_KEY);
	}

	/* ---------------- loading ---------------- */

	let loaded = $state(store.tasks.length > 0);
	onMount(async () => {
		if (!loaded) {
			await store.loadTasks();
			loaded = true;
		}

		const savedTaskId = localStorage.getItem(ACTIVE_FOCUS_TASK_KEY);
		if (savedTaskId && store.tasks.some((task) => task.id === savedTaskId)) {
			activeId = savedTaskId;
		} else if (savedTaskId) {
			localStorage.removeItem(ACTIVE_FOCUS_TASK_KEY);
		}
	});

	/* ---------------- formatting ---------------- */

	function formatDuration(minutes: number) {
		const h = Math.floor(minutes / 60);
		const m = minutes % 60;
		if (h === 0) return `${m}m`;
		return m === 0 ? `${h}h` : `${h}h ${m}m`;
	}
</script>

<svelte:head>
	<title>
		{activeTask ? `${activeTask.title || 'Untitled task'} · FlowDo` : 'Focus · FlowDo'}
	</title>
</svelte:head>

{#snippet startButton(extra: string)}
	<Button
		size="lg"
		onclick={start}
		disabled={!selected}
		class="bg-coral font-semibold text-white shadow-lg shadow-coral/20 hover:bg-coral/90 {extra}"
	>
		<!-- <Play class="size-4 fill-current" /> -->
		Start focus
	</Button>
{/snippet}

<div class="mx-auto w-full max-w-3xl px-4 pt-8 pb-32 sm:px-6 sm:pt-12 sm:pb-12">
	<header class="flex flex-col gap-6 sm:flex-row sm:items-end sm:justify-between">
		<div class="max-w-md">
			<h1 class="text-3xl font-semibold tracking-tight text-navy sm:text-4xl">Today's focus</h1>
			<p class="mt-2 text-sm leading-relaxed text-muted">
				Pick one thing and give it your full attention. The most important task is selected for you.
			</p>
		</div>

		{#if focusTasks.length > 0}
			<div class="flex items-center gap-4 sm:flex-col sm:items-end sm:gap-3">
				<p class="text-sm text-muted">
					<span class="font-medium text-navy">{focusTasks.length} tasks</span>
					· {formatDuration(totalMinutes)}
				</p>
				{@render startButton('hidden sm:inline-flex')}
			</div>
		{/if}
	</header>

	{#if !loaded}
		<div class="mt-8 grid gap-3 sm:mt-10" aria-busy="true" aria-label="Loading tasks">
			{#each [0, 1, 2] as i (i)}
				<Skeleton class="h-[116px] rounded-2xl bg-surface" />
			{/each}
		</div>
	{:else if store.errorGetTasks}
		<Alert.Root variant="destructive" class="mt-10">
			<Alert.Title>Couldn't load your tasks</Alert.Title>
			<Alert.Description>
				<p>{store.errorGetTasks}</p>
				<Button
					variant="outline"
					size="sm"
					class="mt-3 border-line bg-surface text-navy"
					onclick={() => store.loadTasks()}
				>
					<!-- <RotateCw class="size-4" /> -->
					Try again
				</Button>
			</Alert.Description>
		</Alert.Root>
	{:else if focusTasks.length === 0}
		<div class="mt-10 rounded-2xl border border-dashed border-line px-6 py-14 text-center">
			<p class="font-medium text-navy">Nothing to focus on yet</p>
			<p class="mt-1 text-sm text-muted">Add a task to your inbox and it will show up here.</p>
		</div>
	{:else}
		<RadioGroup.Root
			value={selected?.id ?? ''}
			onValueChange={(v) => (pickedId = v)}
			class="mt-8 grid gap-3 sm:mt-10"
			aria-label="Choose a task to focus on"
		>
			{#each focusTasks as task, i (task.id)}
				{@const p = priorityOf(task)}
				<Label
					for="focus-task-{task.id}"
					class="flex cursor-pointer items-start gap-4 rounded-2xl border border-line bg-surface p-4 font-normal transition-colors hover:border-white/15 has-data-[state=checked]:border-coral/60 has-data-[state=checked]:bg-coral/[0.06] sm:p-5"
				>
					<RadioGroup.Item
						value={task.id}
						id="focus-task-{task.id}"
						class="mt-0.5 border-muted/60 text-coral data-[state=checked]:border-coral"
					/>

					<div class="min-w-0 flex-1">
						<div class="flex flex-wrap items-center gap-x-3 gap-y-1">
							<h2 class="text-base font-medium text-navy">{titleOf(task)}</h2>
							<Badge variant="secondary" class="gap-1.5 rounded-full {p.chip}">
								<span class="size-1.5 rounded-full {p.dot}"></span>
								{p.label}
							</Badge>
							{#if task.status === 'IN_PROGRESS'}
								<span class="text-xs font-medium text-orange">In progress</span>
							{:else if i === 0}
								<span class="text-xs font-medium text-mint">Recommended</span>
							{/if}
						</div>

						{#if task.description}
							<p class="mt-1.5 line-clamp-2 text-sm leading-relaxed text-muted">
								{task.description}
							</p>
						{/if}

						<div class="mt-3 flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-muted">
							<span class="inline-flex items-center gap-1.5">
								<!-- <Clock class="size-3.5" /> -->
								{minutesOf(task)} min
							</span>
						</div>
					</div>
				</Label>
			{/each}
		</RadioGroup.Root>

		{#if selected}
			<p class="mt-5 hidden text-sm text-muted sm:block">
				Starts with <span class="font-medium text-navy">{titleOf(selected)}</span>
				· {minutesOf(selected)} min
			</p>
		{/if}
	{/if}
</div>

<!-- Mobile: sticky Start bar -->
{#if loaded && focusTasks.length > 0}
	<div
		class="fixed inset-x-0 bottom-0 z-20 border-t border-line bg-cream/90 px-4 pt-3 pb-[max(1rem,env(safe-area-inset-bottom))] backdrop-blur sm:hidden"
	>
		{#if selected}
			<p class="mb-2 truncate text-center text-xs text-muted">
				Starts with <span class="font-medium text-navy">{titleOf(selected)}</span>
			</p>
		{/if}
		{@render startButton('w-full')}
	</div>
{/if}

<!-- Full-screen focus session, rendered over the whole app -->
{#if activeTask}
	<FocusSession task={activeTask} onexit={closeSession} />
{/if}
