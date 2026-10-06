<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import type { Snippet } from 'svelte';

	interface Props {
		content: Snippet;
		trigger?: Snippet;
		contentTitle: Snippet;
		contentFooter: Snippet;
		open?: boolean;
		onOpenChange?: (o: boolean) => void;
	}
	let { content, trigger, contentTitle, contentFooter, open, onOpenChange }: Props = $props();
</script>

<Dialog.Root {open} {onOpenChange}>
	{#if trigger}
		<Dialog.Trigger class="primary-button! h-auto!">
			{#snippet child({ props })}
				<Button {...props} class="primary-button h-auto">
					{@render trigger()}
				</Button>
			{/snippet}
			{@render trigger()}
		</Dialog.Trigger>
	{/if}
	<Dialog.Content class="bg-[#1b1815] max-h-[calc(100dvh-2rem)] overflow-y-auto p-[clamp(20px,2vw,25px)]  ring-line md:max-w-125">
		<Dialog.Header>
			<Dialog.Title>
				{@render contentTitle()}
			</Dialog.Title>
		</Dialog.Header>
		{@render content()}
		<Dialog.Footer class="gap-2.5 border-0 bg-transparent! sm:items-center">
			<Dialog.Close
				type="button"
				class="inline-flex h-11.25 cursor-pointer items-center justify-center rounded-[9px] border border-line bg-transparent px-5 text-xs font-bold text-[#c7b9a8] transition-all duration-200 hover:border-[#514538] hover:bg-[#28221d] hover:text-navy focus-visible:ring-[3px] focus-visible:ring-coral/20 focus-visible:outline-none"
			>
				Cancel
			</Dialog.Close>
			{@render contentFooter()}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
