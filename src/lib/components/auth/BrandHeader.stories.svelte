<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import BrandHeader from './BrandHeader.svelte';

const { Story } = defineMeta({
  component: BrandHeader,
  tags: ['autodocs'],
  argTypes: {
    subtitle: { control: 'text' },
  },
});
</script>

<Story name="Default">
	{#snippet template()}
		<BrandHeader />
	{/snippet}
</Story>

<Story name="Custom Subtitle" args={{ subtitle: 'Your Notes, Your Space' }}>
	{#snippet template(args)}
		<BrandHeader subtitle={args.subtitle} />
	{/snippet}
</Story>
