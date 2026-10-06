<script lang="ts">
 	import { createUserContext } from "./user.context.svelte";

 
 
	let { children } = $props();

	createUserContext();
</script>

{@render children()}