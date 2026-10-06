<script lang="ts">
	import { ListChecks, Sparkles } from '@lucide/svelte';
	import LoginForm from '../_components/LoginForm.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Sign in · Flowdo</title>
</svelte:head>

<header class="auth-heading">
	<span class="auth-eyebrow"><Sparkles size={12} /> Welcome back</span>
	<h1>Sign in to your workspace</h1>
	<p class="auth-heading-note">
		<span class="auth-heading-icon"><ListChecks size={16} /></span>
		<span>Pick up where you left off. Your tasks are waiting for you.</span>
	</p>
</header>

<LoginForm />

<p class="auth-switch">
	New to Flowdo? <a href={resolve('/(auth)/register')}>Create an account</a>
</p>
