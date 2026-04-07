<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import UserAvatar from './UserAvatar.svelte';

const { Story } = defineMeta({
  component: UserAvatar,
  tags: ['autodocs'],
  argTypes: {
    displayName: { control: 'text' },
    selected: { control: 'boolean' },
  },
});
</script>

<Story name="Default" args={{ displayName: 'Alex River', onclick: () => {} }}>
	{#snippet template(args)}
		<UserAvatar displayName={args.displayName} onclick={args.onclick} />
	{/snippet}
</Story>

<Story name="Selected" args={{ displayName: 'Alex River', selected: true, onclick: () => {} }}>
	{#snippet template(args)}
		<UserAvatar displayName={args.displayName} selected={args.selected} onclick={args.onclick} />
	{/snippet}
</Story>

<Story name="Single Name" args={{ displayName: 'Nabeel', onclick: () => {} }}>
	{#snippet template(args)}
		<UserAvatar displayName={args.displayName} onclick={args.onclick} />
	{/snippet}
</Story>

<Story name="Multi Word Name" args={{ displayName: 'Mary Jane Watson', onclick: () => {} }}>
	{#snippet template(args)}
		<UserAvatar displayName={args.displayName} onclick={args.onclick} />
	{/snippet}
</Story>

<Story name="Gallery" args={{ onclick: () => {} }}>
	{#snippet template(args)}
		<div style="display: flex; gap: 2rem; flex-wrap: wrap; justify-content: center;">
			<UserAvatar displayName="Alex River" onclick={args.onclick} />
			<UserAvatar displayName="Nabeel" onclick={args.onclick} />
			<UserAvatar displayName="Mary Jane Watson" selected={true} onclick={args.onclick} />
			<UserAvatar displayName="Sam" onclick={args.onclick} />
		</div>
	{/snippet}
</Story>
