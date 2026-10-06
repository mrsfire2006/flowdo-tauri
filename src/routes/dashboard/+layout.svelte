<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { getUserContext } from "$lib/components/contexts/user/user.context.svelte";
  import TasksProvider from "$lib/components/contexts/tasks/TasksProvider.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppNavbar from "./layout/AppNavbar.svelte";
  import AppSidebar from "./layout/AppSidebar.svelte";

  let { children } = $props();
  let authenticated = $state(false);
  const userStore = getUserContext();

  onMount(async () => {
    if (await userStore.loadUser()) {
      authenticated = true;
    } else {
      await goto(resolve("/login"));
    }
  });
</script>

{#if authenticated}
  <Sidebar.Provider>
    <TasksProvider>
      <AppSidebar />
      <Sidebar.Inset>
        <div class="px-5 md:px-11.5">
          <AppNavbar />
          {@render children()}
        </div>
      </Sidebar.Inset>
    </TasksProvider>
  </Sidebar.Provider>
{:else}
  <main
    class="grid min-h-screen place-items-center bg-[#0d0b0a] text-sm text-[#b7aaa1]"
  >
    Checking your session…
  </main>
{/if}
