<script lang="ts">
	import { resolve } from '$app/paths';
	import { Check } from '@lucide/svelte';
	import { cn } from 'cn';

	let { size }: { size?: number } = $props();
</script>

<a class="auth-brand" href={resolve('/')}>
	<span class={cn('auth-brand-mark', size ? `size-${size}!` : '')}
		><Check size={16} stroke-width={3} /></span
	>
	Flowdo
</a>
