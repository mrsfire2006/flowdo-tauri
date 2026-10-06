<script lang="ts">
	import FlowdoIcon from '$lib/components/shared/FlowdoIcon.svelte';

	let { children } = $props();
</script>

<main class="auth-page">
	<section class="auth-panel">
		<FlowdoIcon />

		<div class="auth-content">
			{@render children()}
		</div>

		<p class="auth-footer">© 2026 Flowdo. All rights reserved.</p>
	</section>
</main>
