<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import Header from './Header.svelte';

const { Story } = defineMeta({
  component: Header,
  tags: ['autodocs'],
  argTypes: {
    title: { control: 'text' },
  },
});
</script>

<Story name="Default" args={{ title: 'MeNote', onToggleSidebar: () => {} }}>
  {#snippet template(args)}
    <Header title={args.title} onToggleSidebar={args.onToggleSidebar} />
  {/snippet}
</Story>

<Story name="Custom Title" args={{ title: 'My Notes', onToggleSidebar: () => {} }}>
  {#snippet template(args)}
    <Header title={args.title} onToggleSidebar={args.onToggleSidebar} />
  {/snippet}
</Story>
