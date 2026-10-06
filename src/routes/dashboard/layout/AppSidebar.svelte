<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { authClient } from "$lib/auth-client";
  import { getTasksContext } from "$lib/components/contexts/tasks/tasks.context.svelte";
  import { getUserContext } from "$lib/components/contexts/user/user.context.svelte";
  import FlowdoIcon from "$lib/components/shared/FlowdoIcon.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import {
    ChevronsUpDown,
    Focus,
    LayoutList,
    LoaderCircle,
    LogOut,
  } from "@lucide/svelte";
  let isLoggingOut = $state(false);
  let logoutError = $state("");
  const tasksStore = getTasksContext();
  const userStore = getUserContext();
  const handleLogout = async () => {
    isLoggingOut = true;
    logoutError = "";
    try {
      const result = await authClient.signOut();
      if (!result.isSuccess) {
        logoutError = result.errorMsg ?? "Could not sign out.";
        return;
      }
      userStore.removeUser();
      tasksStore.removeTasks();
      await goto(resolve("/login"));
    } finally {
      isLoggingOut = false;
    }
  };
  const navItems = [
    {
      label: "Overview",
      href: resolve("/dashboard/overview"),
      icon: LayoutList,
    },
    { label: "Focus", href: resolve("/dashboard/focus"), icon: Focus },
  ] as const;

  const isActive = (href: string) => page.url.pathname.startsWith(href);
  const username = $derived(userStore.userBio?.name?.slice(0, 2));
</script>

<Sidebar.Root class="justify-between border-line bg-[#0c0b0a] px-4.5 py-7.5">
  <Sidebar.Header>
    <div
      class="mx-auto flex min-w-[90%] flex-col items-start justify-center gap-10"
    >
      <div>
        <FlowdoIcon />
      </div>
      <div
        class="flex w-full items-center gap-3 rounded-xl border border-transparent p-2 transition-all duration-200 hover:border-line hover:bg-[#1b1815]"
      >
        <!-- Avatar -->
        <span
          class="grid size-9 shrink-0 place-items-center rounded-full bg-[#d4e7f2] text-[11px] font-extrabold text-[#397087] uppercase"
        >
          {username}
        </span>

        <span class="grid min-w-0 flex-1 gap-0.5">
          <strong class="truncate text-[13px] font-bold text-navy"
            >{userStore.userBio?.name}'s space</strong
          >
          <small class="truncate text-[11px] text-[#a99b8c]"
            >Personal workspace</small
          >
        </span>

        <ChevronsUpDown class="size-4 shrink-0 text-[#9aa7c0]" />
      </div>
    </div>
    <Sidebar.Separator class="mx-0 w-full bg-line" />
  </Sidebar.Header>
  <Sidebar.Content class="overflow-visible pt-6">
    <Sidebar.Group class="p-0">
      <Sidebar.GroupLabel
        class="mb-2 h-auto px-2 text-[11px] font-bold tracking-[0.14em] text-[#6f86a8] uppercase"
      >
        Workspace
      </Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu class="gap-1.5">
          {#each navItems as item (item.href)}
            {@const active = isActive(item.href)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton
                isActive={active}
                class="relative h-11 gap-3 rounded-xl px-3 text-[13px] font-semibold text-[#9aa7c0] transition-colors before:absolute before:top-1/2 before:-left-4.5 before:h-5 before:w-0.75 before:-translate-y-1/2 before:rounded-r-full before:bg-[#e3a766] before:opacity-0 hover:bg-[#1b1815] hover:text-navy data-[active=true]:bg-[#1f1a14] data-[active=true]:text-navy data-[active=true]:before:opacity-100"
              >
                {#snippet child({ props })}
                  <a
                    href={item.href}
                    aria-current={active ? "page" : undefined}
                    {...props}
                  >
                    <item.icon class="size-4.5 shrink-0" strokeWidth={1.8} />
                    <span>{item.label}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer class="p-0 pt-4">
    {#if logoutError}
      <p class="mb-2 text-center text-xs text-red-400" role="alert">
        {logoutError}
      </p>
    {/if}
    <Button
      type="button"
      variant="ghost"
      disabled={isLoggingOut}
      onclick={handleLogout}
      class="group relative h-11.25 w-full cursor-pointer justify-center gap-2.5 overflow-hidden rounded-[10px] border border-line bg-linear-to-b from-[#1d1a17] to-[#171512] px-4 text-xs font-bold tracking-wide text-[#c7b9a8] shadow-[inset_0_1px_0_#ffffff08] transition-all duration-300 hover:border-[#6b3a30] hover:from-[#2a1b17] hover:to-[#231713] hover:text-[#ffad91] hover:shadow-[inset_0_1px_0_#ffad9114,0_8px_20px_#e9786214] focus-visible:ring-[3px] focus-visible:ring-coral/20 active:scale-[0.98] disabled:opacity-70"
    >
      {#if isLoggingOut}
        <LoaderCircle class="size-4 animate-spin" />
        <span>Signing out...</span>
      {:else}
        <LogOut
          class="size-4 transition-transform duration-300 group-hover:translate-x-0.5"
          strokeWidth={1.9}
        />
        <span>Log out</span>
      {/if}
    </Button>
  </Sidebar.Footer>
</Sidebar.Root>
