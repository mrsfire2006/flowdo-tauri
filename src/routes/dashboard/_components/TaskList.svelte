<script lang="ts">
	import TaskItem from './TaskItem.svelte';
	import type { TaskStatus } from '$lib/db/schemas/task.schema';
	import type { Component } from 'svelte';
	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';
	import TaskCardSkeleton from '$lib/components/shared/TaskCardSkeleton.svelte';
	import type { TaskCard } from '$lib/shared-types/task';
	import EditTask from './create-edit-task/EditTask.svelte';
	import { Ellipsis, LoaderCircle, Plus } from '@lucide/svelte';
	import { dropZone } from '$lib/dropZone';

	interface Props {
		type: { status: TaskStatus; description: string; icon: Component };
		style?: string;
		search?: string;
	}
	let dropState = $state<'idle' | 'over' | 'saving'>('idle');

	let { type, style, search = '' }: Props = $props();

	const section = $derived.by(() => {
		const words = type.status
			.toString()
			.toLowerCase()
			.split(/[_\s]+/);

		return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
	});

	const tasksStore = getTasksContext();

	const filteredTasks = $derived.by(() => {
		const tasks =
			type.status === 'INBOX'
				? tasksStore.inboxTasks
				: type.status === 'IN_PROGRESS'
					? tasksStore.inProgressTasks
					: tasksStore.doneTasks;

		const query = search.trim().toLowerCase();

		if (!query) {
			return tasks;
		}

		return tasks.filter((task) => {
			return (
				task.title?.toLowerCase().startsWith(query) ||
				task.description?.toLowerCase().includes(query)
			);
		});
	});

	const groupedTasks = $derived.by(() => {
		const groups: Record<string, TaskCard[]> = {};

		for (const task of filteredTasks) {
			const createdAt = new Date(task.createdAt);
			const key = createdAt.toDateString();

			(groups[key] ??= []).push(task);
		}

		return Object.entries(groups).map(([key, tasks]) => ({
			key,
			tasks,
			label: formatTaskDate(tasks[0].createdAt)
		}));
	});

	function formatTaskDate(value: Date | string) {
		const date = new Date(value);
		const now = new Date();

		const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
		const taskDate = new Date(date.getFullYear(), date.getMonth(), date.getDate());

		const diffDays = Math.round((today.getTime() - taskDate.getTime()) / 86_400_000);

		if (diffDays === 0) return 'Today';
		if (diffDays === 1) return 'Yesterday';

		return new Intl.DateTimeFormat('en-US', {
			month: 'long',
			day: 'numeric',
			year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined
		}).format(date);
	}
	let selectedTask = $state<TaskCard>();
	let openPop = $state<boolean>(false);
	const tones: Record<TaskStatus, { icon: string; emptyTitle: string; emptyHint: string }> = {
		INBOX: {
			icon: 'bg-[#3a2a25] text-coral',
			emptyTitle: 'Inbox is clear',
			emptyHint: 'New ideas and incoming work will land here.'
		},
		IN_PROGRESS: {
			icon: 'bg-[#3a2e22] text-[#d2a878]',
			emptyTitle: 'Nothing in progress',
			emptyHint: 'Start a task from your inbox to see it here.'
		},
		DONE: {
			icon: 'bg-[#22332a] text-[#7fb892]',
			emptyTitle: 'No wins yet',
			emptyHint: 'Finished tasks will show up here.'
		}
	};
	const tone = $derived(tones[type.status]);
	const isEmpty = $derived(!tasksStore.isLoadingTasks && filteredTasks.length === 0);
	const isHiddenBySearch = $derived(
		!tasksStore.isLoadingTasks && search.trim() !== '' && filteredTasks.length === 0
	);
</script>

{#snippet dropHint()}
	<div class="flex flex-col items-center gap-2.5">
		<div
			class="relative grid size-12 place-items-center rounded-2xl border border-white/10 bg-white/5 text-[#a99b8c]"
		>
			<Plus
				class="absolute size-5 transition-[opacity,transform] duration-200
				{dropState === 'saving' ? 'scale-50 rotate-90 opacity-0' : 'scale-100 rotate-0 opacity-100'}"
				strokeWidth={2.25}
			/>
			<LoaderCircle
				class="absolute size-5 animate-spin transition-opacity duration-200
				{dropState === 'saving' ? 'opacity-100' : 'opacity-0'}"
				strokeWidth={2.25}
			/>
		</div>
		<span class="text-[11px] font-medium tracking-wide text-[#a99b8c]">
			{dropState === 'saving' ? 'Saving…' : 'Drop to move here'}
		</span>
	</div>
{/snippet}
<div class="{style ? style : ''} {isHiddenBySearch ? 'hidden!' : ''}">
	<!-- Header -->
	<header class="mb-5 flex items-start justify-between gap-3 px-1">
		<div class="flex items-center gap-3">
			<span class="grid size-10 shrink-0 place-items-center rounded-xl {tone.icon}">
				<type.icon class="size-5" />
			</span>
			<div class="min-w-0">
				<h3 class="flex items-baseline gap-2 text-sm font-semibold text-navy">
					{section}
					{#if !tasksStore.isLoadingTasks}
						<span class="text-xs font-medium text-[#d2a878]">{filteredTasks.length}</span>
					{/if}
				</h3>
				<p class="truncate text-xs text-[#a99b8c]">{type.description}</p>
			</div>
		</div>

		<button
			type="button"
			aria-label="{section} options"
			class="grid size-8 cursor-pointer place-items-center rounded-lg text-[#a99b8c] transition-colors hover:bg-[#1b1917] hover:text-navy"
		>
			<Ellipsis class="size-4" />
		</button>
	</header>

	<!-- Body -->
	<div
		class="flex flex-col gap-5"
		use:dropZone={{
			getColumnStatus: () => {
				return type.status;
			},

			onDragEnter: (status) => {
				console.log('enter', status, type.status, dropState);

				if (dropState === 'saving') return;

				if (status === type.status) return;
				dropState = 'over';
			},
			onDragLeave: () => {
				if (dropState !== 'saving') dropState = 'idle';
			},
			onDrop: async (t, column) => {
				if (t.status === column) {
					dropState = 'idle';
					return;
				}
				dropState = 'saving';

				try {
					await tasksStore.updateTask.mutateAsync({
						id: t.id,
						task: { status: column }
					});
				} finally {
					dropState = 'idle';
				}
			}
		}}
	>
		<div
			class="grid ease-out
	{dropState === 'idle' || isEmpty
				? 'grid-rows-[0fr] transition-none'
				: 'grid-rows-[1fr] transition-[grid-template-rows] duration-200'}"
		>
			<div class="overflow-hidden">
				<div
					class="pointer-events-none grid h-28 place-items-center rounded-2xl border border-dashed ease-out
			{dropState === 'idle'
						? 'scale-95 opacity-0 transition-none'
						: 'scale-100 opacity-100 transition-[opacity,transform,background-color,border-color] duration-200'}
			{dropState === 'saving' ? 'border-white/10 bg-[#131110]/70' : 'border-white/20 bg-[#131110]/60'}"
				>
					{@render dropHint()}
				</div>
			</div>
		</div>

		{#if tasksStore.isLoadingTasks}
			<TaskCardSkeleton />
			<TaskCardSkeleton withDescription={false} />
			<TaskCardSkeleton />
		{:else if isEmpty}
			<div class="relative">
				<div
					class="pointer-events-none flex flex-col items-center gap-3 rounded-xl border border-dashed border-line bg-[#131110]/60 px-6 py-10 text-center transition-opacity duration-150
			{dropState === 'idle' ? 'opacity-100' : 'opacity-0'}"
				>
					<span class="grid size-11 place-items-center rounded-full {tone.icon} opacity-70">
						<type.icon class="size-5" />
					</span>
					<div class="flex flex-col gap-1">
						<p class="text-xs font-semibold text-navy">{tone.emptyTitle}</p>
						<p class="text-[11px] text-[#a99b8c]">{tone.emptyHint}</p>
					</div>
				</div>

				<div
					class="pointer-events-none absolute inset-0 grid place-items-center rounded-2xl border border-dashed transition-opacity duration-150
			{dropState === 'idle' ? 'opacity-0' : 'opacity-100'}
			{dropState === 'saving' ? 'border-white/10 bg-[#131110]/70' : 'border-white/20 bg-[#131110]/60'}"
				>
					{@render dropHint()}
				</div>
			</div>
		{:else}
			{#each groupedTasks as group (group.key)}
				<div class="flex flex-col gap-3">
					<div class="flex items-center gap-3 px-1">
						<span class="text-[11px] font-semibold whitespace-nowrap text-[#a99b8c]">
							{group.label}
						</span>

						<div class="h-px flex-1 bg-line"></div>
					</div>

					<div class="flex flex-col gap-3">
						{#each group.tasks as task (task.id)}
							<TaskItem
								{task}
								onEdit={(task) => {
									selectedTask = task;
									openPop = true;
								}}
								onStart={async (id) => {
									await tasksStore.updateTask.mutateAsync({
										id,
										task: { ...task, status: 'IN_PROGRESS' }
									});
								}}
								onDone={async (id) => {
									await tasksStore.updateTask.mutateAsync({
										id,
										task: { ...task, status: 'DONE' }
									});
								}}
							/>
						{/each}
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<EditTask
	task={selectedTask}
	status={selectedTask?.status ?? 'DONE'}
	open={openPop}
	onOpenChange={(o) => {
		if (o === false) {
			selectedTask = undefined;
		}
		openPop = o;
	}}
/>
