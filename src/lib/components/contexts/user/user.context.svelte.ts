import { getContext, setContext } from 'svelte';
import { UserStore } from './user.store.svelte';

const KEY = Symbol('user');

export function createUserContext() {
	const store = new UserStore();

	setContext(KEY, store);

	return store;
}

export function getUserContext() {
	return getContext<UserStore>(KEY);
}
