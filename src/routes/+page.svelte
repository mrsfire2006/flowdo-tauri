<script lang="ts">
  import { resolve } from "$app/paths";
  import { ArrowRight, Check, Focus, ListTodo, Target } from "@lucide/svelte";

  const features = [
    { 
      icon: ListTodo,
      title: "Capture",
      text: "Put your tasks in one clear place.",
    },
    {
      icon: Target,
      title: "Prioritize",
      text: "Know what deserves your attention.",
    },
    {
      icon: Focus,
      title: "Focus",
      text: "Work on what matters right now.",
    },
  ];
</script>

<svelte:head>
  <title>FlowDo — Make progress feel natural.</title>
  <meta
    name="description"
    content="A simple workspace for tasks, priorities and focused work."
  />
</svelte:head>

<div class="min-h-screen bg-[#0d0b0a] text-[#f5ede3]">
  <!-- NAVBAR -->
  <header class="mx-auto flex max-w-6xl items-center justify-between px-6 py-7">
    <a href={resolve("/")} class="group flex items-center gap-2.5">
      <div
        class="flex size-8 rotate-[-4deg] items-center justify-center rounded-[9px]
				bg-[#f27d67] transition duration-300
				group-hover:scale-105 group-hover:rotate-0"
      >
        <Check class="size-4 text-[#21110e]" strokeWidth={3} />
      </div>

      <span class="text-[16px] font-semibold tracking-[-0.04em]"> FlowDo </span>
    </a>

    <a
      href={resolve("/dashboard/overview")}
      class="group flex items-center gap-2 rounded-full border border-[#342b27]
			px-4 py-2 text-[11px] text-[#b7aaa1]
			transition duration-300 hover:border-[#685047] hover:text-white"
    >
      Open workspace

      <ArrowRight
        class="size-3 transition-transform duration-300
				group-hover:translate-x-1"
      />
    </a>
  </header>

  <!-- HERO -->
  <main>
    <section class="relative overflow-hidden">
      <!-- subtle background -->
      <div
        class="pointer-events-none absolute top-24 left-1/2 size-105
				-translate-x-1/2 rounded-full bg-[#e67c5c]/5.5
				blur-[120px]"
      ></div>

      <div
        class="relative mx-auto max-w-4xl px-6 pt-28 pb-28 text-center
				sm:pt-36 sm:pb-36"
      >
        <div
          class="mx-auto mb-7 flex w-fit animate-[fadeIn_.7s_ease-out_both] items-center gap-2
					rounded-full border border-[#3c302b] bg-[#15110f] px-3.5
					py-1.5"
        >
          <span class="size-1.5 rounded-full bg-[#f27d67]"></span>

          <span
            class="text-[9px] font-medium tracking-[0.18em] text-[#9c8b80]
						uppercase"
          >
            A calmer way to get things done
          </span>
        </div>

        <h1
          class="animate-[heroIn_.8s_.1s_ease-out_both] text-[clamp(3.5rem,8vw,6.8rem)]
					leading-[0.9] font-medium
					tracking-[-0.075em]"
        >
          Make progress
          <span class="block text-[#ee916c]"> feel natural. </span>
        </h1>

        <p
          class="mx-auto mt-8 max-w-lg animate-[heroIn_.8s_.2s_ease-out_both] text-[14px]
					leading-7
					text-[#82776f]"
        >
          FlowDo brings your tasks, priorities, and focus together in one simple
          workspace — so you can spend less time organizing and more time moving
          forward.
        </p>

        <div
          class="mt-9 flex animate-[heroIn_.8s_.3s_ease-out_both] flex-col items-center justify-center
					gap-4
					sm:flex-row"
        >
          <a
            href={resolve("/dashboard/overview")}
            class="group flex items-center gap-3 rounded-full
						bg-[#f27d67] px-6 py-3.5 text-[12px] font-semibold
						text-[#241411]
						transition duration-300
						hover:-translate-y-1 hover:bg-[#ff8b74]
						hover:shadow-[0_12px_35px_rgba(242,125,103,.18)]"
          >
            Enter your workspace

            <span
              class="flex size-5 items-center justify-center
							rounded-full bg-black/10"
            >
              <ArrowRight
                class="size-3 transition-transform duration-300
								group-hover:translate-x-0.5"
              />
            </span>
          </a>

          <a
            href="#features"
            class="group flex items-center gap-2 text-[12px]
						text-[#756a63] transition duration-300
						hover:text-[#e8ddd2]"
          >
            See how it works

            <ArrowRight
              class="size-3 transition-transform duration-300
							group-hover:translate-x-1"
            />
          </a>
        </div>
      </div>
    </section>

    <!-- SMALL DIVIDER -->
    <div class="border-y border-[#211d1a]">
      <div
        class="mx-auto flex max-w-4xl items-center justify-center
				gap-4 px-6 py-4 text-[9px] tracking-[0.2em] text-[#625852]
				uppercase"
      >
        <span>Less noise</span>
        <span class="h-px w-8 bg-[#3b302b]"></span>
        <span>More intention</span>
        <span class="h-px w-8 bg-[#3b302b]"></span>
        <span>Better days</span>
      </div>
    </div>

    <!-- FEATURES -->
    <section id="features" class="mx-auto max-w-6xl px-6 py-28 sm:py-36">
      <div class="max-w-xl">
        <p
          class="text-[9px] font-semibold tracking-[0.2em] text-[#b8755a]
					uppercase"
        >
          Simple by design
        </p>

        <h2
          class="mt-5 text-[clamp(2.5rem,5vw,4.5rem)] leading-[.95]
					font-medium tracking-[-.065em]"
        >
          Everything you need.
          <span class="text-[#e78c68]"> Nothing in the way. </span>
        </h2>

        <p class="mt-6 max-w-md text-[13px] leading-6 text-[#756a63]">
          FlowDo keeps your workflow focused around three simple things.
        </p>
      </div>

      <div class="mt-16 grid gap-3 md:grid-cols-3">
        {#each features as feature, i (feature.title)}
          <div
            class="group rounded-2xl border border-[#29231f]
						bg-[#12100f] p-6
						transition duration-500
						hover:-translate-y-1
						hover:border-[#49372f]
						hover:bg-[#171310]"
            style={`animation: cardIn .7s ${i * 120}ms ease-out both`}
          >
            <div
              class="flex size-10 items-center justify-center
							rounded-xl bg-[#211814]
							transition duration-500
							group-hover:scale-105
							group-hover:bg-[#2c1e19]"
            >
              <feature.icon class="size-4 text-[#d98262]" strokeWidth={1.8} />
            </div>

            <h3
              class="mt-12 text-[17px] font-medium
							tracking-[-0.03em]"
            >
              {feature.title}
            </h3>

            <p
              class="mt-2 text-[12px] leading-6 text-[#6f655e]
							transition duration-300
							group-hover:text-[#897b72]"
            >
              {feature.text}
            </p>
          </div>
        {/each}
      </div>
    </section>

    <!-- SHORT STATEMENT -->
    <section class="border-y border-[#211d1a] bg-[#15100e]">
      <div class="mx-auto max-w-4xl px-6 py-28 text-center sm:py-36">
        <p
          class="text-[clamp(2rem,4vw,3.5rem)] leading-tight
					font-medium tracking-[-0.055em]"
        >
          The goal isn't to do more.
          <span class="text-[#e58a67]">
            It's to make room for what matters.
          </span>
        </p>
      </div>
    </section>

    <!-- CTA -->
    <section class="mx-auto max-w-6xl px-6 py-28 sm:py-36">
      <div
        class="flex flex-col items-start justify-between gap-8
				rounded-3xl border border-[#302721]
				bg-[#15110f] p-8 sm:p-12 md:flex-row md:items-end"
      >
        <div>
          <p
            class="text-[9px] tracking-[0.2em] text-[#a66d56]
						uppercase"
          >
            Start simple
          </p>

          <h2
            class="mt-4 text-[clamp(2.5rem,5vw,4.5rem)]
						leading-[.95] font-medium tracking-[-0.065em]"
          >
            Make today
            <span class="text-[#ed906c]">count.</span>
          </h2>
        </div>

        <a
          href={resolve("/dashboard/overview")}
          class="group flex shrink-0 items-center gap-3
					rounded-full bg-[#f27d67] px-5 py-3.5
					text-[12px] font-semibold text-[#241411]
					transition duration-300
					hover:-translate-y-1 hover:bg-[#ff8b74]
					hover:shadow-[0_12px_30px_rgba(242,125,103,.15)]"
        >
          Get started

          <ArrowRight
            class="size-3.5 transition-transform duration-300
						group-hover:translate-x-1"
          />
        </a>
      </div>
    </section>
  </main>

  <!-- FOOTER -->
  <footer class="border-t border-[#211d1a]">
    <div
      class="mx-auto flex max-w-6xl items-center justify-between
			px-6 py-7"
    >
      <div class="flex items-center gap-2">
        <div
          class="flex size-7 items-center justify-center
					rounded-lg bg-[#f27d67]"
        >
          <Check class="size-3.5 text-[#21110e]" strokeWidth={3} />
        </div>

        <span class="text-[13px] font-semibold">FlowDo</span>
      </div>

      <span class="text-[9px] text-[#554c47]"> Make today count. </span>
    </div>
  </footer>
</div>

<style>
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes heroIn {
    from {
      opacity: 0;
      transform: translateY(22px);
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes cardIn {
    from {
      opacity: 0;
      transform: translateY(18px);
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    * {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
      transition-duration: 0.01ms !important;
    }
  }
</style>
