<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import Card from './Card.svelte';

const { Story } = defineMeta({
  component: Card,
  tags: ['autodocs'],
  argTypes: {
    onclick: { action: 'clicked' },
  },
});
</script>

<Story name="Default" args={{}}>
  {#snippet template(args)}
    <Card onclick={args.onclick}>
      {#snippet children()}
        <p>This is a card with some content inside it.</p>
      {/snippet}
    </Card>
  {/snippet}
</Story>

<Story name="Clickable" args={{ onclick: () => {} }}>
  {#snippet template(args)}
    <Card onclick={args.onclick}>
      {#snippet children()}
        <p>Click me! This card has an onclick handler and shows a pointer cursor.</p>
      {/snippet}
    </Card>
  {/snippet}
</Story>

<Story name="Custom Content" args={{}}>
  {#snippet template(args)}
    <Card>
      {#snippet children()}
        <h3 style="margin-bottom: 0.5rem">Card Title</h3>
        <p style="color: var(--color-text-muted); font-size: 0.875rem">Supports any content via snippets.</p>
      {/snippet}
    </Card>
  {/snippet}
</Story>
