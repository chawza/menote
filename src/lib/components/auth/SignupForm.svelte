<script lang="ts">
import Button from '../Button.svelte';
import PasswordInput from './PasswordInput.svelte';

interface Props {
  onsubmit: (data: { display_name: string; password: string }) => void;
  error?: string;
  loading?: boolean;
}

let { onsubmit, error = '', loading = false }: Props = $props();

let displayName = $state('');
let password = $state('');
let confirmPassword = $state('');
let validationError = $state('');

function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  validationError = '';

  const trimmedName = displayName.trim();
  if (!trimmedName) {
    validationError = 'Display name is required';
    return;
  }
  if (!password) {
    validationError = 'Password is required';
    return;
  }
  if (password !== confirmPassword) {
    validationError = 'Passwords do not match';
    return;
  }

  onsubmit({ display_name: trimmedName, password });
}
</script>

<form class="signup-form" onsubmit={handleSubmit}>
	<PasswordInput
		bind:value={displayName}
		type="text"
		placeholder="Display name"
		disabled={loading}
		autocomplete="name"
	/>

	<PasswordInput
		bind:value={password}
		placeholder="Password"
		disabled={loading}
		autocomplete="new-password"
	/>

	<PasswordInput
		bind:value={confirmPassword}
		placeholder="Confirm password"
		disabled={loading}
		autocomplete="new-password"
	/>

	{#if validationError || error}
		<p class="signup-form__error">{validationError || error}</p>
	{/if}

	<div class="signup-form__actions">
		<Button type="submit" variant="primary" disabled={loading}>
			{loading ? 'Creating...' : 'Create User'}
		</Button>
	</div>
</form>

<style>
	.signup-form {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.signup-form__error {
		font-size: 0.75rem;
		color: var(--color-danger);
		text-align: center;
	}

	.signup-form__actions {
		display: flex;
		justify-content: center;
	}
</style>
