<script lang="ts">
	import { Goal, Sparkles } from '@lucide/svelte';
	import RegisterForm from '../_components/RegisterForm.svelte';
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>Sign up · Flowdo</title>
</svelte:head>

<header class="auth-heading">
	<span class="auth-eyebrow"><Sparkles size={12} /> Get started</span>
	<h1>Create your account</h1>
	<p class="auth-heading-note">
		<span class="auth-heading-icon"><Goal size={16} /></span>
		<span>Every goal starts with a single task. Start yours today.</span>
	</p>
</header>

<RegisterForm />

<p class="auth-switch">
	Already have an account? <a href={resolve('/(auth)/login')}>Sign in</a>
</p>

