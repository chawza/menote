<script lang="ts">
interface Props {
  class?: string;
}

let { class: className = '' }: Props = $props();

let classes = $derived(['auth-footer', className].filter(Boolean).join(' '));
</script>

<div class={classes}>
	<svg
		class="auth-footer__icon"
		xmlns="http://www.w3.org/2000/svg"
		width="14"
		height="14"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		<rect width="18" height="11" x="3" y="11" rx="2" ry="2" />
		<path d="M7 11V7a5 5 0 0 1 10 0v4" />
	</svg>
	<span class="auth-footer__text">End-to-End Encrypted</span>
</div>

<style>
	.auth-footer {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		opacity: 0.4;
		margin-top: 6rem;
	}

	.auth-footer__icon {
		flex-shrink: 0;
	}

	.auth-footer__text {
		font-size: 0.5625rem;
		text-transform: uppercase;
		letter-spacing: 0.3em;
		color: var(--color-text-muted);
	}
</style>
