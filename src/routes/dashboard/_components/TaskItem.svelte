<script module lang="ts">
	const priorityStyles: Record<TaskPriority, { label: string; text: string }> = {
		HIGH: { label: 'High', text: 'text-coral' },
		MEDIUM: { label: 'Medium', text: 'text-orange' },
		LOW: { label: 'Low', text: 'text-mint' }
	};

	// const dueStyles = {
	// 	overdue: 'text-coral',
	// 	today: 'text-orange',
	// 	later: 'text-muted'
	// } as const;

	function formatEstimate(min: number) {
		if (min < 60) return `${min} min`;
		const h = Math.floor(min / 60);
		const m = min % 60;
		return m ? `${h}h ${m}m` : `${h}h`;
	}
	function formatRelativeDate(value: Date | string, label: string) {
		const date = new Date(value);
		const diff = Date.now() - date.getTime();

		const minute = 60 * 1000;
		const hour = 60 * minute;
		const day = 24 * hour;

		if (diff < minute) return `${label} just now`;

		if (diff < hour) {
			const minutes = Math.floor(diff / minute);
			return `${label} ${minutes}m ago`;
		}

		if (diff < day) {
			const hours = Math.floor(diff / hour);
			return `${label} ${hours}h ago`;
		}

		if (diff < 2 * day) return `${label} yesterday`;

		return `${label} ${new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric'
		}).format(date)}`;
	}
</script>

<script lang="ts">
	import {
		Check,
		CheckCircle,
		Circle,
		Clock,
		Copy,
		Flag,
		GripVertical,
		MoreHorizontal,
		Pencil,
		Trash,
		Zap
	} from '@lucide/svelte';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { cn } from '$lib/utils';
	import type { TaskPriority } from '$lib/db/schemas/task.schema';
	import type { TaskCard } from '$lib/shared-types/task';
	import { draggable } from '$lib/draggable';

	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';

	interface Props {
		task: TaskCard;
		onStart?: (id: string) => void;
		onDone?: (id: string) => void;
		onEdit: (task: TaskCard) => void;
		class?: string;
	}

	let { task, onStart, onDone, onEdit, class: className }: Props = $props();

	const isDone = $derived(task.status === 'DONE');
	const isActive = $derived(task.status === 'IN_PROGRESS');
	const priority = $derived(priorityStyles[task.priority!]);
	const StatusIcon = $derived(isDone ? CheckCircle : Circle);
	const iconBtn =
		'rounded-md p-1.5 text-muted transition-colors hover:bg-[#26211c] hover:text-navy focus-visible:outline-2 focus-visible:outline-coral';

	const store = getTasksContext();

	const createTask = store.createTask;
	const deleteTask = store.deleteTask;

	const handleDelete = async () => {
		await deleteTask.mutateAsync({ id: task.id });
	};
	const handleDublicate = async () => {
		await createTask.mutateAsync({
			task: {
				title: task.title,
				description: task.description,
				estimatedMinutes: task.durationMinutes,
				priority: task.priority
			}
		});
	};
</script>

<article
	use:draggable={{
		getTask: () => {
			return task;
		}
	}}
	data-status={task.status}
	class={cn(
		'group   relative rounded-2xl border border-line bg-surface p-5 shadow-[0_6px_18px_#0206172e] transition-colors hover:border-[#4a4034]',
		isActive && 'border-coral/40 hover:border-coral/60',
		className
	)}
>
	<button
		data-drag-handle
		type="button"
		title="Drag to move"
		aria-label="Drag to move"
		class={cn(
			'absolute top-4.5 -left-5 hidden h-8   w-7 cursor-grab! touch-none items-center justify-start rounded-l-md pl-1 text-muted @[650px]:flex',
			'opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100 [@media(hover:none)]:opacity-100',
			'hover:text-navy focus-visible:outline-2 focus-visible:outline-coral active:cursor-grabbing'
		)}
	>
		<GripVertical class="size-4" strokeWidth={1.75} />
	</button>

	<!-- Header: status + quick edit + more menu -->
	<header class="flex items-center justify-between">
		<button
			type="button"
			onclick={() => {
				if (!isDone) {
					onDone?.(task.id);
				}
			}}
			aria-label={isDone ? 'Mark as not done' : 'Mark as done'}
			class={cn(
				'-m-1 rounded-full p-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-coral',
				isDone ? 'text-mint' : isActive ? 'text-coral' : 'text-muted hover:text-coral'
			)}
		>
			<StatusIcon
				class={cn('size-6', isActive && 'animate-spin animation-duration-[3s]')}
				strokeWidth={1.75}
			/>
			{#if isDone && task.completedAt}
				<span
					class="truncate text-[11px] font-medium text-mint/70"
					title={new Date(task.completedAt).toLocaleString()}
				>
					{formatRelativeDate(task.completedAt, 'Completed')}
				</span>
			{/if}
		</button>

		<div class="flex items-center gap-1">
			<button
				type="button"
				onclick={() => {
					onEdit(task);
				}}
				aria-label="Edit task"
				class={iconBtn}
			>
				<Pencil class="size-4" strokeWidth={1.75} />
			</button>

			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<button
							{...props}
							type="button"
							aria-label="More actions"
							class={cn(
								iconBtn,
								'data-[state=open]:border-line data-[state=open]:bg-white/6 data-[state=open]:text-navy'
							)}
						>
							<MoreHorizontal class="size-4" strokeWidth={1.75} />
						</button>
					{/snippet}
				</DropdownMenu.Trigger>

				<DropdownMenu.Content
					align="end"
					sideOffset={8}
					class="w-56 rounded-xl bg-surface  p-1.5 text-navy shadow-[0_20px_50px_-12px_rgba(0,0,0,0.75),0_0_0_1px_rgba(255,255,255,0.03)_inset]
			ring-line"
				>
					<DropdownMenu.Label
						class="px-2.5 pt-1.5 pb-2 text-[10px] font-bold tracking-[0.12em] text-muted/70 uppercase"
					>
						Task actions
					</DropdownMenu.Label>

					<DropdownMenu.Item
						class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors
				focus:bg-white/6 data-highlighted:bg-white/6"
						onSelect={handleDublicate}
					>
						<span
							class="grid size-8 shrink-0 place-items-center rounded-md border border-line bg-white/4
					text-muted transition-colors group-data-highlighted:border-sky/30 group-data-highlighted:bg-sky/10 group-data-highlighted:text-sky"
						>
							<Copy class="size-4" strokeWidth={1.75} />
						</span>
						<span class="grid leading-tight">
							<span class="text-[13px] font-semibold text-navy"
								>{createTask.isPaused ? 'Duplicating' : 'Duplicate'}</span
							>
							<span class="text-[11px] text-muted">Create a copy of this task</span>
						</span>
					</DropdownMenu.Item>

					<DropdownMenu.Separator class="-mx-1.5 my-1.5 bg-line" />

					<DropdownMenu.Item
						variant="destructive"
						class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors
				focus:bg-coral/10 data-highlighted:bg-coral/10"
						onSelect={handleDelete}
					>
						<span
							class="grid size-8 shrink-0 place-items-center rounded-md border border-coral/20 bg-coral/10 text-coral"
						>
							<Trash class="size-4" strokeWidth={1.75} />
						</span>
						<span class="grid leading-tight">
							<span class="text-[13px] font-semibold text-coral"
								>{deleteTask.isPending ? 'Deleting' : 'Delete'}</span
							>
							<span class="text-[11px] text-coral/60">Remove</span>
						</span>
					</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</header>

	<!-- Content -->
	<div class="mt-4 space-y-1.5">
		<h3
			class={cn(
				'text-[17px] leading-snug font-medium text-navy',
				isDone && 'text-muted line-through decoration-line'
			)}
		>
			{task.title}
		</h3>
		{#if task.description}
			<p class="max-w-[60ch] text-sm leading-relaxed text-muted">
				{task.description}
			</p>
		{/if}
	</div>

	<!-- Meta -->
	<ul class="mt-4 flex flex-wrap items-center gap-x-4 gap-y-2 text-[13px]">
		<li class={cn('flex items-center gap-1.5 font-medium', priority.text)}>
			<Flag class="size-3.5 fill-current" strokeWidth={1.75} />
			{priority.label}
		</li>
		{#if task.durationMinutes != null}
			<li class="flex items-center gap-1.5 text-muted">
				<Clock class="size-3.5" strokeWidth={1.75} />
				{formatEstimate(task.durationMinutes)}
			</li>
		{/if}
		<li
			class="ml-auto flex items-center gap-1.5 text-[11px] text-muted/70"
			title={new Date(task.updatedAt).toLocaleString()}
		>
			<Clock class="size-3" strokeWidth={1.75} />
			{formatRelativeDate(task.updatedAt, 'Updated')}
		</li>
	</ul>

	<!-- Actions -->
	{#if !isDone}
		<footer class="mt-4 flex items-center gap-2 border-t border-line pt-3">
			{#if !isActive}
				<button
					type="button"
					onclick={() => onStart?.(task.id)}
					class="flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-sm text-muted transition-colors hover:bg-coral/10 hover:text-coral focus-visible:outline-2 focus-visible:outline-coral"
				>
					<Zap class="size-4" strokeWidth={1.75} />
					Start
				</button>
			{/if}
			<button
				type="button"
				onclick={() => onDone?.(task.id)}
				class="flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-sm text-muted transition-colors hover:bg-mint/10 hover:text-mint focus-visible:outline-2 focus-visible:outline-mint"
			>
				<Check class="size-4" strokeWidth={2} />
				Done
			</button>
		</footer>
	{/if}
</article>
