<script lang="ts">
	import { cn } from '$lib/utils';

	interface Props {
		withFooter?: boolean;

		withDescription?: boolean;
		class?: string;
	}

	let { withFooter = true, withDescription = true, class: className }: Props = $props();

	const bone = 'animate-pulse rounded-md bg-white/6';
</script>

<article
	aria-hidden="true"
	class={cn(
		'relative rounded-2xl border border-line bg-surface p-5 shadow-[0_6px_18px_#0206172e]',
		className
	)}
>
	<!-- Header: status icon + action buttons -->
	<header class="flex items-center justify-between">
		<div class={cn(bone, 'size-6 rounded-full')}></div>
		<div class="flex items-center gap-1">
			<div class={cn(bone, 'size-7')}></div>
			<div class={cn(bone, 'size-7')}></div>
		</div>
	</header>

	<!-- Title + description -->
	<div class="mt-4 space-y-2.5">
		<div class={cn(bone, 'h-4.25 w-3/4')}></div>
		{#if withDescription}
			<div class="space-y-2 pt-1">
				<div class={cn(bone, 'h-3 w-full')}></div>
				<div class={cn(bone, 'h-3 w-2/3')}></div>
			</div>
		{/if}
	</div>

	<!-- Meta: priority + duration -->
	<div class="mt-4 flex items-center gap-x-4">
		<div class={cn(bone, 'h-3.5 w-16')}></div>
		<div class={cn(bone, 'h-3.5 w-12')}></div>
	</div>

	<!-- Footer -->
	{#if withFooter}
		<footer class="mt-4 flex items-center gap-2 border-t border-line pt-3">
			<div class={cn(bone, 'h-8 w-16 rounded-lg')}></div>
			<div class={cn(bone, 'h-8 w-16 rounded-lg')}></div>
		</footer>
	{/if}
</article>
