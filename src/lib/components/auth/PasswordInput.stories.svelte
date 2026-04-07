<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import PasswordInput from './PasswordInput.svelte';

const { Story } = defineMeta({
  component: PasswordInput,
  tags: ['autodocs'],
  argTypes: {
    placeholder: { control: 'text' },
    error: { control: 'text' },
    disabled: { control: 'boolean' },
    type: {
      control: 'select',
      options: ['password', 'text'],
    },
  },
});
</script>

<Story name="Default" args={{ placeholder: 'Key required', onsubmit: () => {} }}>
	{#snippet template(args)}
		<PasswordInput placeholder={args.placeholder} onsubmit={args.onsubmit} />
	{/snippet}
</Story>

<Story name="With Error" args={{ placeholder: 'Key required', error: 'Invalid password', onsubmit: () => {} }}>
	{#snippet template(args)}
		<PasswordInput placeholder={args.placeholder} error={args.error} onsubmit={args.onsubmit} />
	{/snippet}
</Story>

<Story name="Disabled" args={{ placeholder: 'Key required', disabled: true, onsubmit: () => {} }}>
	{#snippet template(args)}
		<PasswordInput placeholder={args.placeholder} disabled={args.disabled} onsubmit={args.onsubmit} />
	{/snippet}
</Story>

<Story name="Text Type" args={{ placeholder: 'Display name', type: 'text', onsubmit: () => {} }}>
	{#snippet template(args)}
		<PasswordInput placeholder={args.placeholder} type={args.type} onsubmit={args.onsubmit} />
	{/snippet}
</Story>
