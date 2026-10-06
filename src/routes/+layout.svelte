<script lang="ts">
  import "../app.css";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import UserProvider from "$lib/components/contexts/user/UserProvider.svelte";

  let { children } = $props();
  const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
  <UserProvider>
    {@render children()}
    <SvelteQueryDevtools />
  </UserProvider>
</QueryClientProvider>
