<script lang="ts">
import { getAvatarColor, getInitials } from './auth-utils';

interface Props {
  displayName: string;
  selected?: boolean;
  onclick?: () => void;
}

let { displayName, selected = false, onclick }: Props = $props();

let initials = $derived(getInitials(displayName));
let bgColor = $derived(getAvatarColor(displayName));
</script>

<button
	class="user-avatar"
	class:user-avatar--selected={selected}
	onclick={onclick}
	type="button"
>
	<div
		class="user-avatar__circle"
		style="background-color: {bgColor};"
	>
		<span class="user-avatar__initials">{initials}</span>
	</div>
	<span class="user-avatar__name">{displayName}</span>
</button>

<style>
	.user-avatar {
		display: flex;
		flex-direction: column;
		align-items: center;
		background: none;
		border: none;
		cursor: pointer;
		padding: 0;
		outline: none;
	}

	.user-avatar__circle {
		position: relative;
		width: 6rem;
		height: 6rem;
		border-radius: 9999px;
		padding: 0.25rem;
		border: 1px solid var(--color-border);
		overflow: hidden;
		display: flex;
		align-items: center;
		justify-content: center;
		transition:
			border-color 0.5s,
			transform 0.5s,
			box-shadow 0.5s;
	}

	.user-avatar:hover .user-avatar__circle {
		border-color: var(--color-accent);
		transform: scale(1.05);
		box-shadow: 0 0 1.25rem var(--auth-glow);
	}

	.user-avatar--selected .user-avatar__circle {
		border-color: var(--color-accent);
		box-shadow: 0 0 1.25rem var(--auth-glow);
	}

	.user-avatar__initials {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-bg);
		user-select: none;
	}

	.user-avatar__name {
		margin-top: 1.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--color-text);
		letter-spacing: -0.01em;
		transition: color 0.3s;
	}

	.user-avatar:hover .user-avatar__name {
		color: var(--color-accent);
	}

	.user-avatar--selected .user-avatar__name {
		color: var(--color-accent);
	}
</style>
