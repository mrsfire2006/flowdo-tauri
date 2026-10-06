import { getContext, setContext } from 'svelte';
import { TaskStore } from './tasks.store.svelte';

const KEY = Symbol('tasks');

export function createTasksContext() {
	const store = new TaskStore();

	setContext(KEY, store);

	return store;
}

export function getTasksContext() {
	return getContext<TaskStore>(KEY);
}
