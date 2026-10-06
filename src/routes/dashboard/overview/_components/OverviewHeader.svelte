<script lang="ts">
	import { Sparkles, Target, Timer, CircleCheck } from '@lucide/svelte'; // أو lucide-svelte
	import { getTasksContext } from '$lib/components/contexts/tasks/tasks.context.svelte';
	import CreateTask from '../../_components/create-edit-task/CreateTask.svelte';
	import { getUserContext } from '$lib/components/contexts/user/user.context.svelte';

	const tasksStore = getTasksContext();
	const userStore = getUserContext();

	const now = new Date();
	const dateLabel = now.toLocaleDateString('en-US', {
		weekday: 'long',
		month: 'long',
		day: 'numeric'
	});

	const hour = now.getHours();
	const greeting = hour < 12 ? 'Good morning' : hour < 18 ? 'Good afternoon' : 'Good evening';

	const openTasks = $derived([...tasksStore.inboxTasks, ...tasksStore.inProgressTasks]);
	const openCount = $derived(openTasks.length);
	const plannedMinutes = $derived(openTasks.reduce((sum, t) => sum + (t.durationMinutes ?? 0), 0));
	const doneCount = $derived(tasksStore.doneTasks.length);
	const completedMinutes = $derived(
		tasksStore.doneTasks.reduce((sum, t) => sum + (t.durationMinutes ?? 0), 0)
	);

	const stats = $derived([
		{ icon: Target, tone: 'coral', value: String(openCount), label: 'Open tasks' },
		{ icon: Timer, tone: 'blue', value: `${plannedMinutes} min`, label: 'On your plate' },
		{ icon: CircleCheck, tone: 'mint', value: String(doneCount), label: 'Completed' },
		{ icon: Timer, tone: 'purple', value: String(completedMinutes), label: 'Time completed' }
	]);
</script>

<header class="page-header">
	<div>
		<span class="eyebrow">
			<Sparkles size={14} />
			{dateLabel}
		</span>

		<h1>{greeting}, {userStore.userBio?.name} <span>✦</span></h1>
		<p>Capture, organize, and finish your work.</p>

		<div class="motivation-note">
			<Sparkles size={14} />
			Small steps still move you forward.
		</div>
	</div>

	<CreateTask />
</header>

<section class="stats-row" aria-label="Task summary">
	{#each stats as stat (stat.label)}
		<div>
			<span class="stat-icon {stat.tone}">
				<stat.icon size={16} />
			</span>
			<div>
				<strong>{stat.value}</strong>
				<small>{stat.label}</small>
			</div>
		</div>
	{/each}
</section>

<style>
	.stat-icon.purple {
		background: #8e79d91a;
		color: #8e79d9;
	}
</style>
