<script lang="ts">
  import { page } from "$app/state";
  import { Bell, Check, ChevronDown } from "@lucide/svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import { resolve } from "$app/paths";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { goto } from "$app/navigation";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { getUserContext } from "$lib/components/contexts/user/user.context.svelte";

  const pages = [
    { label: "Overview", href: resolve("/dashboard/overview") },
    { label: "Focus", href: resolve("/dashboard/focus") },
  ];
  const currentPage = $derived(
    pages.find((p) => page.url.pathname.startsWith(p.href))?.label ??
      "Overview",
  );

  const userStore = getUserContext();
</script>

<header class="flex h-19.25 items-center justify-between border-b border-line">
  <!-- Breadcrumb -->
  <nav aria-label="Breadcrumb" class="flex items-center gap-2.5 text-xs">
    <Sidebar.Trigger
      aria-label="Toggle sidebar"
      class="size-9 cursor-pointer rounded-[9px] border border-transparent text-[#9aa7c0] transition-all duration-200 hover:border-[#4a4034] hover:bg-[#26211c] hover:text-navy focus-visible:ring-[3px] focus-visible:ring-coral/20"
    />
    <div class="h-4 w-px bg-line" aria-hidden="true"></div>

    <span class="text-[#71809a]">Workspace</span>
    <span class="text-[#4a5468]">/</span>

    <DropdownMenu.Root>
      <DropdownMenu.Trigger
        class="group flex cursor-pointer items-center gap-1.5 rounded-lg px-2 py-1 font-bold text-navy transition-colors duration-200 outline-none hover:bg-[#26211c] focus-visible:ring-[3px] focus-visible:ring-coral/20 data-[state=open]:bg-[#26211c]"
        aria-current="page"
      >
        {currentPage}
        <ChevronDown
          class="size-3.5 text-[#9aa7c0] transition-transform duration-200 group-data-[state=open]:rotate-180"
        />
      </DropdownMenu.Trigger>

      <DropdownMenu.Content
        align="start"
        sideOffset={8}
        class="min-w-44 rounded-xl border-line bg-[#1d1a17] p-1.5 text-navy shadow-[0_20px_50px_#08060480]"
      >
        {#each pages as p (p.href)}
          <DropdownMenu.Item
            onSelect={async () => await goto(p.href)}
            class="flex cursor-pointer items-center justify-between rounded-lg px-3 py-2.5 text-xs font-semibold text-[#c7b9a8] transition-colors data-highlighted:bg-[#28221d] data-highlighted:text-navy {currentPage ===
            p.label
              ? 'text-coral'
              : ''}"
          >
            {p.label}
            {#if currentPage === p.label}
              <Check class="size-3.5 text-coral" />
            {/if}
          </DropdownMenu.Item>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </nav>

  <!-- Actions -->
  <div class="flex items-center gap-4.5 max-sm:ml-auto">
    <Button
      variant="ghost"
      size="icon"
      aria-label="Notifications"
      class="size-9 cursor-pointer rounded-[9px] border border-transparent text-[#9aa7c0] transition-all duration-200 hover:border-[#4a4034] hover:bg-[#26211c] hover:text-navy focus-visible:ring-[3px] focus-visible:ring-coral/20"
    >
      <Bell class="size-4.5" strokeWidth={1.8} />
    </Button>
    <Button
      variant="ghost"
      size="icon"
      aria-label="Profile"
      class="relative size-9 uppercase cursor-pointer rounded-full bg-linear-to-br from-[#f7d3c6] to-[#e9a795] text-[11px] font-extrabold tracking-wide text-[#7a2f26] shadow-[0_4px_14px_#e9786233] ring-2 ring-[#e97862]/30 ring-offset-2 ring-offset-[#11100e] transition-all duration-200 hover:-translate-y-px hover:bg-linear-to-br hover:from-[#fbdcd1] hover:to-[#efb3a2] hover:text-[#7a2f26] hover:shadow-[0_6px_18px_#e9786240] hover:ring-[#e97862]/60 focus-visible:ring-coral active:translate-y-0 max-sm:hidden"
    >
      {#if userStore.isLoadingUser || !userStore.userBio}
        <span class="size-4 animate-pulse rounded-full bg-white/50"></span>
      {:else}
        {userStore.userBio?.name?.slice(0, 2)}
      {/if}
      <span
        class="absolute right-0 bottom-0 size-2.5 rounded-full border-2 border-[#11100e] bg-[#83b58f]"
      ></span>
    </Button>
  </div>
</header>
