<script lang="ts">
interface Props {
  value?: string;
  placeholder?: string;
  error?: string;
  disabled?: boolean;
  type?: 'password' | 'text';
  autocomplete?: HTMLInputElement['autocomplete'];
  onsubmit?: () => void;
}

let {
  value = $bindable(''),
  placeholder = '',
  error = '',
  disabled = false,
  type = 'password',
  autocomplete = 'current-password' as const,
  onsubmit,
}: Props = $props();

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !disabled) {
    onsubmit?.();
  }
}
</script>

<div class="underline-input">
	<div class="underline-input__wrapper">
		<input
			class="underline-input__field"
			{type}
			bind:value
			{placeholder}
			{disabled}
			onkeydown={handleKeydown}
			{autocomplete}
		/>
		<div class="underline-input__line"></div>
	</div>
	{#if error}
		<p class="underline-input__error">{error}</p>
	{/if}
</div>

<style>
	.underline-input {
		width: 100%;
	}

	.underline-input__wrapper {
		position: relative;
	}

	.underline-input__field {
		display: block;
		width: 100%;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		border-bottom: 1px solid var(--color-border);
		border-radius: 0;
		text-align: center;
		font-family: inherit;
		font-size: 1.125rem;
		line-height: 1.5;
		color: var(--color-text);
		transition: border-color 0.3s;
	}

	.underline-input__field::placeholder {
		color: var(--color-text-muted);
		opacity: 0.5;
	}

	.underline-input__field:focus {
		outline: none;
		border-color: transparent;
	}

	.underline-input__field:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.underline-input__line {
		position: absolute;
		bottom: 0;
		left: 50%;
		transform: translateX(-50%);
		width: 0;
		height: 1px;
		background-color: var(--color-accent);
		transition: width 0.5s;
	}

	.underline-input__wrapper:focus-within .underline-input__line {
		width: 100%;
	}

	.underline-input__error {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: var(--color-danger);
		text-align: center;
	}
</style>
