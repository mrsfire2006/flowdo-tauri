<script lang="ts">
	import * as Field from '$lib/components/ui/field/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import * as Select from '$lib/components/ui/select/index.js';
	import type { TaskCard, UpdateTaskRequest } from '$lib/shared-types/task';
	import Button from '$lib/components/ui/button/button.svelte';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import AlertMessage from '$lib/components/shared/AlertMessage.svelte';
	import { taskPriority, type TaskStatus } from '$lib/db/schemas/task.schema';
	import CreateEditTask from './CreateEditTask.svelte';
	import { onDestroy } from 'svelte';
	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';
	import { resolve } from '$app/paths';
	import { goto } from '$app/navigation';
	let error = $state<string>('');
	let success = $state<string>('');

	let errorTimeout: ReturnType<typeof setTimeout> | undefined;
	let successTimeout: ReturnType<typeof setTimeout> | undefined;
	const showError = (message: string) => {
		if (errorTimeout) {
			clearTimeout(errorTimeout);
		}

		error = message;

		errorTimeout = setTimeout(() => {
			error = '';
			errorTimeout = undefined;
		}, 3000);
	};

	const showSuccess = (message: string) => {
		if (successTimeout) {
			clearTimeout(successTimeout);
		}

		success = message;

		successTimeout = setTimeout(() => {
			success = '';
			successTimeout = undefined;
		}, 3000);
	};

	onDestroy(() => {
		if (errorTimeout) clearTimeout(errorTimeout);
		if (successTimeout) clearTimeout(successTimeout);
	});

	let isLoading = $state<boolean>(false);
	interface Props {
		task?: TaskCard;
		status: TaskStatus;
		open: boolean;
		onOpenChange: (o: boolean) => void;
	}
	let { task, status, open, onOpenChange }: Props = $props();

	let updateRequest = $state<UpdateTaskRequest>({});

	$effect(() => {
		if (!open || !task) return;

		updateRequest = {
			title: task.title ?? '',
			description: task.description ?? '',
			estimatedMinutes: task.durationMinutes ?? 0,
			priority: task.priority ?? 'LOW'
		};
	});
	const store = getTasksContext();

	const handleEdit = async (e: SubmitEvent) => {
		e.preventDefault();

		const payload = {
			...updateRequest,
			title: updateRequest.title?.trim() ?? '',
			description: updateRequest.description?.trim() ?? ''
		};
		if (!payload.title) {
			showError('Task title is required');

			success = '';
			return;
		}
		isLoading = true;
		try {
			if (!task?.id) {
				return;
			}

			const result = await store.updateTask.mutateAsync({ id: task.id, task: updateRequest });
			if (result.statusCode === 401) {
				error = 'Unauthorized, redirecting to login...';
				success = '';
				await new Promise((resolve) => setTimeout(resolve, 500));
				await goto(resolve('/(auth)/login'));
				return;
			}
			if (!result.isSuccess) {
				showError(result.errorMsg ?? 'Error');

				success = '';
			} else {
				showSuccess('task edited succussfully');

				error = '';
			}
		} catch {
			showError('Network error, check your connection and try again.');

			success = '';
		} finally {
			isLoading = false;
		}
	};
	const priorityList = taskPriority.enumValues;
	// const isEditInboxMode = $derived(status === 'INBOX');
	// const isEditInProgressMode = $derived(status === 'IN_PROGRESS');
	const isEditDoneMode = $derived(status === 'DONE');
</script>

<CreateEditTask {open} {onOpenChange}>
	{#snippet contentTitle()}
		<div class="modal flex flex-col gap-5">
			<span class="eyebrow">Edit Task</span>
			<h2>What will move this task forward?</h2>
		</div>
	{/snippet}
	{#snippet content()}
		<form id="create-task-form" onsubmit={handleEdit}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="fieldgroup-title">New title</Field.Label>
					<Input
						class="h-11.25! rounded-[9px] border-line bg-[#131110]! px-3.25 text-xs text-navy shadow-[inset_0_1px_3px_#00000050] transition-[border-color,box-shadow] duration-200 placeholder:text-[#777067] hover:border-[#514538] focus-visible:border-coral focus-visible:shadow-none focus-visible:ring-[3px] focus-visible:ring-coral/10"
						id="fieldgroup-title"
						placeholder="Learn Python basics"
						bind:value={updateRequest.title}
					/>
				</Field.Field>
				<Field.Field>
					<Field.Label for="fieldgroup-description">Description</Field.Label>
					<Textarea
						class="min-h-24 rounded-[9px] border-line bg-[#131110]! px-3.25 py-3 text-xs text-navy shadow-[inset_0_1px_3px_#00000050] transition-[border-color,box-shadow] duration-200 placeholder:text-[#777067] hover:border-[#514538] focus-visible:border-coral focus-visible:shadow-none focus-visible:ring-[3px] focus-visible:ring-coral/10"
						id="fieldgroup-description"
						placeholder="Study and practice"
						bind:value={updateRequest.description}
					/>
				</Field.Field>
			</Field.Group>
			<div class="mt-4.25 grid grid-cols-[1fr_1fr] gap-3.5">
				<!-- Priority -->

				<Field.Field
					class={isEditDoneMode? 'pointer-events-none opacity-50' : ''}
				>
					<Field.Label>Priority</Field.Label>
					<Select.Root
						type="single"
						bind:value={
							() => updateRequest.priority ?? undefined,
							(v) => (updateRequest.priority = v as 'LOW' | 'MEDIUM' | 'HIGH')
						}
					>
						<Select.Trigger
							class="h-11.25! w-full rounded-[9px] border-line bg-[#131110]! px-3.25 text-xs text-navy capitalize shadow-[inset_0_1px_3px_#00000050] transition-[border-color,box-shadow] duration-200 hover:border-[#514538] focus:border-coral focus:ring-[3px] focus:ring-coral/10 data-placeholder:text-[#777067] data-[state=open]:border-coral data-[state=open]:ring-[3px] data-[state=open]:ring-coral/10"
						>
							<Select.Value placeholder="Select a priority" />
						</Select.Trigger>

						<Select.Content
							class="rounded-xl border-line bg-[#1d1a17] p-1.5 text-navy shadow-[0_20px_50px_#08060480]"
						>
							{#each priorityList as priority (priority)}
								<Select.Item
									value={priority}
									class="cursor-pointer rounded-lg px-3 py-2.5 text-xs font-semibold text-[#c7b9a8] capitalize transition-colors data-highlighted:bg-[#28221d] data-highlighted:text-navy data-selected:text-coral"
								>
									{priority.toLowerCase()}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</Field.Field>
				<!-- Estimate Time -->
				<Field.Field class={`col-span-2 ${isEditDoneMode ? 'pointer-events-none opacity-50' : ''}`}>
					<Field.Label for="fieldgroup-estimate">Estimate</Field.Label>

					<div class="relative">
						<Input
							id="fieldgroup-estimate"
							type="number"
							min="0"
							step="1"
							bind:value={
								() => updateRequest.estimatedMinutes,
								(v) => (updateRequest.estimatedMinutes = v ?? 0)
							}
							class="h-11.25! [appearance:textfield] rounded-[9px] border-line bg-[#131110]! pr-14 pl-3.25 text-xs text-navy shadow-[inset_0_1px_3px_#00000050] transition-[border-color,box-shadow] duration-200 placeholder:text-[#777067] hover:border-[#514538] focus-visible:border-coral focus-visible:shadow-none focus-visible:ring-[3px] focus-visible:ring-coral/10 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
						/>
						<span
							class="pointer-events-none absolute top-1/2 right-3.25 -translate-y-1/2 rounded-md bg-[#28221d] px-2 py-0.5 text-[10px] font-bold tracking-wider text-[#d2a878] uppercase"
						>
							min
						</span>
					</div>

					<div class="flex flex-wrap gap-2">
						{#each [15, 25, 45, 60, 90] as m (m)}
							<button
								type="button"
								onclick={() => {
									updateRequest.estimatedMinutes = m;
								}}
								class="rounded-full border px-3 py-1.5 text-[11px] font-semibold transition-all duration-200
				{updateRequest.estimatedMinutes === m
									? 'border-coral bg-coral/15 text-coral'
									: 'border-line bg-[#1b1917] text-[#a99b8c] hover:border-[#514538] hover:text-navy'}"
							>
								{m < 60 ? `${m}m` : m === 60 ? '1h' : '1h 30m'}
							</button>
						{/each}
					</div>
				</Field.Field>
				<!-- <Field.Field
					class={`col-span-2 ${isEditInProgressMode ? 'pointer-events-none opacity-50' : ''}`}
				>
					<Field.Label for="fieldgroup-estimate">Estimate</Field.Label>

					<div class="relative">
						<Input
							id="fieldgroup-estimate"
							type="number"
							min="0"
							step="5"
							bind:value={
								() => updateRequest.estimatedMinutes,
								(v) => (updateRequest.estimatedMinutes = v ?? 0)
							}
							class="h-11.25! [appearance:textfield] rounded-[9px] border-line bg-[#131110]! pr-14 pl-3.25 text-xs text-navy shadow-[inset_0_1px_3px_#00000050] transition-[border-color,box-shadow] duration-200 placeholder:text-[#777067] hover:border-[#514538] focus-visible:border-coral focus-visible:shadow-none focus-visible:ring-[3px] focus-visible:ring-coral/10 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
						/>
						<span
							class="pointer-events-none absolute top-1/2 right-3.25 -translate-y-1/2 rounded-md bg-[#28221d] px-2 py-0.5 text-[10px] font-bold tracking-wider text-[#d2a878] uppercase"
						>
							min
						</span>
					</div>

					<div class="flex flex-wrap gap-2">
						{#each [15, 25, 45, 60, 90] as m (m)}
							<button
								type="button"
								onclick={() => {
									updateRequest.estimatedMinutes = m;
								}}
								class="rounded-full border px-3 py-1.5 text-[11px] font-semibold transition-all duration-200
				{updateRequest.estimatedMinutes === m
									? 'border-coral bg-coral/15 text-coral'
									: 'border-line bg-[#1b1917] text-[#a99b8c] hover:border-[#514538] hover:text-navy'}"
							>
								{m < 60 ? `${m}m` : m === 60 ? '1h' : '1h 30m'}
							</button>
						{/each}
					</div>
				</Field.Field> -->
			</div>
			{#if error}
				<AlertMessage message={error} type="error" />
			{:else if success}
				<AlertMessage message={success} type="success" />
			{/if}

			<Separator orientation="horizontal" class="my-5 bg-line" />
		</form>
	{/snippet}
	{#snippet contentFooter()}
		<Button
			type="submit"
			form="create-task-form"
			disabled={isLoading}
			class="h-11.25 cursor-pointer rounded-[9px] border border-coral bg-coral px-6 text-xs font-extrabold text-white shadow-[0_7px_20px_#e9786226] transition-all duration-200 hover:-translate-y-px hover:border-[#f18a74] hover:bg-[#f18a74] focus-visible:ring-[3px] focus-visible:ring-coral/30 active:translate-y-0"
		>
			{isLoading ? 'Creating...' : 'Save changes'}
		</Button>
	{/snippet}
</CreateEditTask>

<style>
	.eyebrow {
		color: #d2a878;
	}

	.modal h2 {
		letter-spacing: -0.7px;
		margin: 8px 0 7px;
		font-size: 22px;
	}
</style>
