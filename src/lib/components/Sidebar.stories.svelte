<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import Sidebar from './Sidebar.svelte';

const { Story } = defineMeta({
  component: Sidebar,
  tags: ['autodocs'],
  argTypes: {
    isOpen: { control: 'boolean' },
  },
});
</script>

<Story name="Closed" args={{ isOpen: false, onclose: () => {} }}>
  {#snippet template(args)}
    <Sidebar isOpen={args.isOpen} onclose={args.onclose} />
  {/snippet}
</Story>

<Story name="Open" args={{ isOpen: true, onclose: () => {} }}>
  {#snippet template(args)}
    <Sidebar isOpen={args.isOpen} onclose={args.onclose} />
  {/snippet}
</Story>
