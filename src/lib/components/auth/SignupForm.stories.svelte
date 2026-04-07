<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import SignupForm from './SignupForm.svelte';

const { Story } = defineMeta({
  component: SignupForm,
  tags: ['autodocs'],
  argTypes: {
    error: { control: 'text' },
    loading: { control: 'boolean' },
  },
});
</script>

<Story name="Default" args={{ onsubmit: (data) => console.log(data) }}>
	{#snippet template(args)}
		<SignupForm onsubmit={args.onsubmit} />
	{/snippet}
</Story>

<Story name="With Error" args={{ error: 'User already exists', onsubmit: (data) => console.log(data) }}>
	{#snippet template(args)}
		<SignupForm error={args.error} onsubmit={args.onsubmit} />
	{/snippet}
</Story>

<Story name="Loading" args={{ loading: true, onsubmit: (data) => console.log(data) }}>
	{#snippet template(args)}
		<SignupForm loading={args.loading} onsubmit={args.onsubmit} />
	{/snippet}
</Story>
