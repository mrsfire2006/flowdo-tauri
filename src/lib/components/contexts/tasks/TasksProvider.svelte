<script lang="ts">
	import { createTasksContext } from "./tasks.context.svelte";

 
 
	let { children } = $props();

	createTasksContext();
</script>

{@render children()}