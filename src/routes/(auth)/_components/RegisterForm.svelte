<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { authClient } from "$lib/auth-client";
  import { getUserContext } from "$lib/components/contexts/user/user.context.svelte";
  import { ArrowRight, Eye, EyeOff, LoaderCircle } from "@lucide/svelte";

  let showPassword = $state(false);
  let loading = $state(false);
  let error = $state<string | undefined>(undefined);
  let RegisterData = $state({ username: "", email: "", password: "" });
  let confirmPassword = $state("");
  const userStore = getUserContext();
  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = undefined;

    if (RegisterData.password !== confirmPassword) {
      error = "Passwords do not match.";
      return;
    }

    loading = true;

    try {
      const result = await authClient.signUp.email({
        name: RegisterData.username,
        email: RegisterData.email,
        password: RegisterData.password,
      });
      if (!result.isSuccess) {
        error = result.errorMsg ?? "Could not create your account.";
        return;
      }
      await userStore.loadUser();
      await goto(resolve("/dashboard/overview"));
    } catch {
      error = "Could not create your account. Please try again.";
    } finally {
      loading = false;
    }
  }
</script>

<form class="auth-form" onsubmit={handleSubmit}>
  {#if error}
    <div class="auth-error" role="alert">{error}</div>
  {/if}

  <label class="auth-field">
    <span>Full name</span>
    <input
      bind:value={RegisterData.username}
      autocomplete="name"
      name="name"
      type="text"
      placeholder="Your name"
      required
    />
  </label>

  <label class="auth-field">
    <span>Email address</span>
    <input
      bind:value={RegisterData.email}
      autocomplete="email"
      name="email"
      type="email"
      placeholder="you@example.com"
      required
    />
  </label>

  <label class="auth-field">
    <span>Password</span>
    <span class="auth-password-wrap">
      <input
        bind:value={RegisterData.password}
        autocomplete="new-password"
        name="password"
        type={showPassword ? "text" : "password"}
        placeholder="At least 3 characters"
        minlength="3"
        required
      />
      <button
        class="auth-password-toggle"
        type="button"
        onclick={() => (showPassword = !showPassword)}
        aria-label={showPassword ? "Hide password" : "Show password"}
        tabindex={-1000}
      >
        {#if showPassword}<EyeOff size={17} />{:else}<Eye size={17} />{/if}
      </button>
    </span>
  </label>

  <label class="auth-field">
    <span>Confirm password</span>
    <span class="auth-password-wrap">
      <input
        bind:value={confirmPassword}
        autocomplete="new-password"
        name="confirmPassword"
        type={showPassword ? "text" : "password"}
        placeholder="Repeat your password"
        minlength="3"
        required
      />
      <button
        class="auth-password-toggle"
        type="button"
        onclick={() => (showPassword = !showPassword)}
        aria-label={showPassword ? "Hide password" : "Show password"}
        tabindex={-1000}
      >
        {#if showPassword}<EyeOff size={17} />{:else}<Eye size={17} />{/if}
      </button>
    </span>
  </label>

  <button class="auth-submit" type="submit" disabled={loading}>
    {#if loading}
      <LoaderCircle size={17} class="auth-spin" /> Creating account…
    {:else}
      Create account <ArrowRight size={17} />
    {/if}
  </button>
</form>
