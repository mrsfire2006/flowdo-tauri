<script lang="ts">
	import { CircleAlert, CircleCheck } from '@lucide/svelte';

	interface Props {
		type: 'error' | 'success' | 'none';
		message: string;
	}
	const { message, type }: Props = $props();
</script>

{#if type !== 'none'}
	<div
		class="form-message"
		class:error={type === 'error'}
		class:success={type === 'success'}
		role={type === 'error' ? 'alert' : 'status'}
	>
		<span class="icon">
			{#if type === 'error'}
				<CircleAlert size={16} strokeWidth={2.2} />
			{:else}
				<CircleCheck size={16} strokeWidth={2.2} />
			{/if}
		</span>
		<p>{message}</p>
	</div>
{/if}

<style>
	.form-message {
		display: flex;
		align-items: center;
		gap: 12px;
		margin: 16px 0 4px;
		padding: 10px 14px 10px 12px;
		border: 1px solid transparent;
		border-radius: 10px;
		font-size: 12px;
		font-weight: 600;
		line-height: 1.5;
		animation: message-in 0.2s ease;
	}

	.form-message p {
		margin: 0;
		flex: 1;
		min-width: 0;
		overflow-wrap: anywhere;
	}

	.icon {
		display: grid;
		place-items: center;
		flex: 0 0 auto;
		width: 30px;
		height: 30px;
		border-radius: 8px;
	}

	.form-message.error {
		color: #ffad91;
		background: #2a1b17;
		border-color: #6b3a30;
	}

	.form-message.error .icon {
		color: #ffad91;
		background: #3b2923;
	}

	.form-message.success {
		color: #9bcea2;
		background: #17251d;
		border-color: #2f5540;
	}

	.form-message.success .icon {
		color: #9bcea2;
		background: #20382c;
	}

	@keyframes message-in {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.form-message {
			animation: none;
		}
	}
</style>
