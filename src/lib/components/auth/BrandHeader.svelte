<script lang="ts">
interface Props {
  subtitle?: string;
}

let { subtitle = 'Private Workspace' }: Props = $props();
</script>

<div class="brand-header">
	<h1 class="brand-header__title">MeNote</h1>
	<p class="brand-header__subtitle">{subtitle}</p>
</div>

<style>
	.brand-header {
		text-align: center;
		margin-bottom: 4rem;
	}

	.brand-header__title {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		color: var(--color-text);
		margin-bottom: 0.25rem;
	}

	.brand-header__subtitle {
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.4em;
		color: var(--color-text-muted);
	}
</style>
