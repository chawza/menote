<script lang="ts">
import type { UserData } from '$lib/types';
import UserAvatar from './UserAvatar.svelte';

interface Props {
  users: UserData[];
  selectedUserId?: number;
  onselect: (userId: number) => void;
}

let { users, selectedUserId, onselect }: Props = $props();
</script>

{#if users.length > 0}
	<div class="user-selector">
		{#each users as user (user.id)}
			<UserAvatar
				displayName={user.display_name}
				selected={user.id === selectedUserId}
				onclick={() => onselect(user.id)}
			/>
		{/each}
	</div>
{/if}

<style>
	.user-selector {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 2rem;
	}
</style>
