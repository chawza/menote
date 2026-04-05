<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import TextArea from './TextArea.svelte';

const { Story } = defineMeta({
  component: TextArea,
  tags: ['autodocs'],
  argTypes: {
    placeholder: { control: 'text' },
    disabled: { control: 'boolean' },
    rows: { control: 'number' },
  },
});
</script>

<Story name="Default" args={{ placeholder: '', disabled: false }}>
  {#snippet template(args)}
    <TextArea placeholder={args.placeholder} disabled={args.disabled}>
    </TextArea>
  {/snippet}
</Story>

<Story name="With Placeholder" args={{ placeholder: 'Type something here...', disabled: false }}>
  {#snippet template(args)}
    <TextArea placeholder={args.placeholder} disabled={args.disabled}>
    </TextArea>
  {/snippet}
</Story>

<Story name="Disabled" args={{ placeholder: 'Cannot type here', disabled: true }}>
  {#snippet template(args)}
    <TextArea placeholder={args.placeholder} disabled={args.disabled}>
    </TextArea>
  {/snippet}
</Story>

<Story name="With Rows" args={{ placeholder: 'Taller textarea', disabled: false, rows: 8 }}>
  {#snippet template(args)}
    <TextArea placeholder={args.placeholder} disabled={args.disabled} rows={args.rows}>
    </TextArea>
  {/snippet}
</Story>
