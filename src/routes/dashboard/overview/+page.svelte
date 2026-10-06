<script lang="ts">
	import { CircleCheck, Inbox, Zap, Search, ListFilter, ArrowUpDown } from '@lucide/svelte';
	import TaskList from '../_components/TaskList.svelte';
	import OverviewHeader from './_components/OverviewHeader.svelte';
	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';
	import type { TaskStatus } from '$lib/db/schemas/task.schema';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let search = $state('');

	let statusFilter = $state<TaskStatus | 'ALL'>('ALL');
	const tasksStore = getTasksContext();
	tasksStore.loadTasks();
	function listClass(status: TaskStatus) {
		const matches = statusFilter === status;
		const isAll = statusFilter === 'ALL';
		const searching = search.trim() !== '';

		const desktop = isAll || matches ? '@[650px]:block' : '@[650px]:hidden';

		const mobileVisible = matches || (isAll && (status === 'INBOX' || searching));
		const mobile = mobileVisible ? 'block' : 'hidden';

		return `${mobile} ${desktop}`;
	}
</script>
<svelte:head>
	<title>
		Overview . FlowDo
	</title>
</svelte:head>
<OverviewHeader />

<!-- Toolbar -->
<div class="toolbar">
	<label class="search-box">
		<Search size={15} />
		<input type="search" placeholder="Search tasks..." bind:value={search} />
	</label>

	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			aria-label="Filter tasks"
			class="flex size-9 items-center justify-center gap-2 rounded-lg border border-line bg-surface text-muted transition-colors hover:bg-white/6 hover:text-navy md:w-auto md:px-3"
		>
			<ListFilter class="hidden size-4 md:block" strokeWidth={1.75} />

			<span class="flex md:hidden">
				{#if statusFilter === 'ALL'}
					<ListFilter class="size-4" strokeWidth={1.75} />
				{:else if statusFilter === 'INBOX'}
					<Inbox class="size-4" strokeWidth={1.75} />
				{:else if statusFilter === 'IN_PROGRESS'}
					<Zap class="size-4" strokeWidth={1.75} />
				{:else}
					<CircleCheck class="size-4" strokeWidth={1.75} />
				{/if}
			</span>
			<span class="hidden text-sm font-medium md:inline">
				{statusFilter === 'ALL'
					? 'Filter'
					: statusFilter === 'INBOX'
						? 'Inbox'
						: statusFilter === 'IN_PROGRESS'
							? 'In Progress'
							: 'Done'}
			</span>
		</DropdownMenu.Trigger>

		<!-- One Dropdown -->
		<DropdownMenu.Content
			align="end"
			sideOffset={8}
			class="w-56 rounded-xl bg-surface p-1.5 text-navy shadow-[0_20px_50px_-12px_rgba(0,0,0,0.75),0_0_0_1px_rgba(255,255,255,0.03)_inset] ring-line"
		>
			<DropdownMenu.Label
				class="px-2.5 pt-1.5 pb-2 text-[10px] font-bold tracking-[0.12em] text-muted/70 uppercase"
			>
				Filter tasks
			</DropdownMenu.Label>

			<DropdownMenu.RadioGroup
				value={statusFilter}
				onValueChange={(value) => {
					statusFilter = value as TaskStatus | 'ALL';
				}}
			>
				<DropdownMenu.RadioItem
					value="ALL"
					class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors focus:bg-white/6 data-highlighted:bg-white/6"
				>
					<span
						class="grid size-8 shrink-0 place-items-center rounded-md border border-line bg-white/4 text-muted transition-colors group-data-highlighted:border-sky/30 group-data-highlighted:bg-sky/10 group-data-highlighted:text-sky"
					>
						<ListFilter class="size-4" strokeWidth={1.75} />
					</span>

					<span class="grid leading-tight">
						<span class="text-[13px] font-semibold text-navy">All tasks</span>
						<span class="text-[11px] text-muted">Show every task</span>
					</span>
				</DropdownMenu.RadioItem>

				<DropdownMenu.RadioItem
					value="INBOX"
					class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors focus:bg-white/6 data-highlighted:bg-white/6"
				>
					<span
						class="grid size-8 shrink-0 place-items-center rounded-md border border-line bg-white/4 text-muted transition-colors group-data-highlighted:border-yellow/30 group-data-highlighted:bg-yellow/10 group-data-highlighted:text-yellow"
					>
						<Inbox class="size-4" strokeWidth={1.75} />
					</span>

					<span class="grid leading-tight">
						<span class="text-[13px] font-semibold text-navy">Inbox</span>
						<span class="text-[11px] text-muted">Waiting to be started</span>
					</span>
				</DropdownMenu.RadioItem>

				<DropdownMenu.RadioItem
					value="IN_PROGRESS"
					class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors focus:bg-white/6 data-highlighted:bg-white/6"
				>
					<span
						class="grid size-8 shrink-0 place-items-center rounded-md border border-line bg-white/4 text-muted transition-colors group-data-highlighted:border-coral/30 group-data-highlighted:bg-coral/10 group-data-highlighted:text-coral"
					>
						<Zap class="size-4" strokeWidth={1.75} />
					</span>

					<span class="grid leading-tight">
						<span class="text-[13px] font-semibold text-navy">In Progress</span>
						<span class="text-[11px] text-muted">Currently being worked on</span>
					</span>
				</DropdownMenu.RadioItem>

				<DropdownMenu.RadioItem
					value="DONE"
					class="group cursor-pointer gap-3 rounded-lg px-2 py-2 transition-colors focus:bg-white/6 data-highlighted:bg-white/6"
				>
					<span
						class="grid size-8 shrink-0 place-items-center rounded-md border border-line bg-white/4 text-muted transition-colors group-data-highlighted:border-mint/30 group-data-highlighted:bg-mint/10 group-data-highlighted:text-mint"
					>
						<CircleCheck class="size-4" strokeWidth={1.75} />
					</span>

					<span class="grid leading-tight">
						<span class="text-[13px] font-semibold text-navy">Done</span>
						<span class="text-[11px] text-muted">Completed tasks</span>
					</span>
				</DropdownMenu.RadioItem>
			</DropdownMenu.RadioGroup>
		</DropdownMenu.Content>
	</DropdownMenu.Root>

	<button type="button" class="outline-button">
		<ArrowUpDown size={15} />
		Sort
	</button>
</div>

<!-- Tasks -->
<div class="@container">
	<div
		class="grid grid-cols-1 items-start gap-5 @[650px]:auto-cols-fr @[650px]:grid-flow-col @[650px]:grid-cols-none"
	>
		<TaskList
			type={{
				status: 'INBOX',
				description: 'Things waiting to be started',
				icon: Inbox
			}}
			{search}
			style={listClass('INBOX')}
		/>
		<TaskList
			type={{
				status: 'IN_PROGRESS',
				description: "What you're working on now",
				icon: Zap
			}}
			style={listClass('IN_PROGRESS')}
			{search}
		/>
		<TaskList
			type={{
				status: 'DONE',
				description: 'A record of your progress',
				icon: CircleCheck
			}}
			style={listClass('DONE')}
			{search}
		/>
	</div>
</div>
