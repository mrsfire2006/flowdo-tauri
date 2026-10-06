<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { authClient } from "$lib/auth-client";
  import { getUserContext } from "$lib/components/contexts/user/user.context.svelte";
  import { ArrowRight, Eye, EyeOff, LoaderCircle } from "@lucide/svelte";

  let showPassword = $state(false);
  let loading = $state(false);
  let error = $state<string | undefined>(undefined);
  let rememberMe = $state(true);
  let LoginData = $state({ email: "", password: "" });
  const userStore = getUserContext();
  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = undefined;

    loading = true;

    try {
      const result = await authClient.signIn.email({
        email: LoginData.email,
        password: LoginData.password,
        rememberMe,
      });

      if (!result.isSuccess) {
        error = result.errorMsg ?? "Invalid email or password.";
        return;
      }
      await userStore.loadUser();
      await goto(resolve("/dashboard/overview"));
    } catch {
      error = "Invalid email or password. Please try again.";
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
    <span>Email address</span>
    <input
      bind:value={LoginData.email}
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
        bind:value={LoginData.password}
        autocomplete="current-password"
        name="password"
        type={showPassword ? "text" : "password"}
        placeholder="Enter your password"
        minlength="3"
        required
      />
      <button
        class="auth-password-toggle"
        type="button"
        onclick={() => (showPassword = !showPassword)}
        aria-label={showPassword ? "Hide password" : "Show password"}
      >
        {#if showPassword}<EyeOff size={17} />{:else}<Eye size={17} />{/if}
      </button>
    </span>
  </label>

  <div class="auth-options">
    <label class="auth-consent">
      <input bind:checked={rememberMe} type="checkbox" name="remember" />
      <span>Remember me</span>
    </label>
  </div>

  <button class="auth-submit" type="submit" disabled={loading}>
    {#if loading}
      <LoaderCircle size={17} class="auth-spin" /> Signing in…
    {:else}
      Sign in <ArrowRight size={17} />
    {/if}
  </button>
</form>
