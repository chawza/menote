<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import Button from './Button.svelte';

const { Story } = defineMeta({
  component: Button,
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: 'select',
      options: ['primary', 'danger', 'ghost'],
    },
    disabled: { control: 'boolean' },
    type: {
      control: 'select',
      options: ['button', 'submit', 'reset'],
    },
  },
});
</script>

<Story name="Primary" args={{ variant: 'primary', onclick: () => {} }}>
  {#snippet template(args)}
    <Button variant={args.variant} onclick={args.onclick}>
      {#snippet children()}
        Primary Button
      {/snippet}
    </Button>
  {/snippet}
</Story>

<Story name="Danger" args={{ variant: 'danger', onclick: () => {} }}>
  {#snippet template(args)}
    <Button variant={args.variant} onclick={args.onclick}>
      {#snippet children()}
        Delete
      {/snippet}
    </Button>
  {/snippet}
</Story>

<Story name="Ghost" args={{ variant: 'ghost', onclick: () => {} }}>
  {#snippet template(args)}
    <Button variant={args.variant} onclick={args.onclick}>
      {#snippet children()}
        Cancel
      {/snippet}
    </Button>
  {/snippet}
</Story>

<Story name="Disabled" args={{ variant: 'primary', disabled: true, onclick: () => {} }}>
  {#snippet template(args)}
    <Button variant={args.variant} disabled={args.disabled} onclick={args.onclick}>
      {#snippet children()}
        Disabled
      {/snippet}
    </Button>
  {/snippet}
</Story>
