<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import UserSelector from './UserSelector.svelte';

const { Story } = defineMeta({
  component: UserSelector,
  tags: ['autodocs'],
  argTypes: {
    onselect: { action: 'selected' },
  },
});

const mockUsers = [
  { id: 1, display_name: 'Alex River', email: 'ar@local', created_at: 0 },
  { id: 2, display_name: 'Nabeel', email: 'nb@local', created_at: 0 },
  { id: 3, display_name: 'Sam', email: 'sm@local', created_at: 0 },
];
</script>

<Story name="Single User" args={{ onselect: () => {} }}>
	{#snippet template(args)}
		<UserSelector users={[mockUsers[0]]} onselect={(id) => args.onselect(id)} />
	{/snippet}
</Story>

<Story name="Multiple Users" args={{ onselect: () => {} }}>
	{#snippet template(args)}
		<UserSelector users={mockUsers} onselect={(id) => args.onselect(id)} />
	{/snippet}
</Story>

<Story name="With Selection" args={{ onselect: () => {} }}>
	{#snippet template(args)}
		<UserSelector users={mockUsers} selectedUserId={2} onselect={(id) => args.onselect(id)} />
	{/snippet}
</Story>

<Story name="Empty" args={{ onselect: () => {} }}>
	{#snippet template(args)}
		<UserSelector users={[]} onselect={(id) => args.onselect(id)} />
	{/snippet}
</Story>
