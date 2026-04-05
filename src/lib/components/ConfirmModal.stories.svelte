<script lang="ts" module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import ConfirmModal from './ConfirmModal.svelte';

  const { Story } = defineMeta({
    component: ConfirmModal,
    tags: ['autodocs'],
    argTypes: {
      isShow: { control: 'boolean' },
      title: { control: 'text' },
      confirmText: { control: 'text' },
      cancelText: { control: 'text' },
    },
  });
</script>

<Story name="Default" args={{ isShow: true, title: 'Confirm', confirmText: 'Confirm', cancelText: 'Cancel', onconfirm: () => {}, oncancel: () => {} }}>
  {#snippet template(args)}
    <ConfirmModal isShow={args.isShow} onconfirm={args.onconfirm} oncancel={args.oncancel} title={args.title} confirmText={args.confirmText} cancelText={args.cancelText}>
      {#snippet content()}
        <p>Are you sure you want to proceed?</p>
      {/snippet}
    </ConfirmModal>
  {/snippet}
</Story>

<Story name="Delete Confirmation" args={{ isShow: true, title: 'Delete Note', confirmText: 'Delete', cancelText: 'Cancel', onconfirm: () => {}, oncancel: () => {} }}>
  {#snippet template(args)}
    <ConfirmModal isShow={args.isShow} onconfirm={args.onconfirm} oncancel={args.oncancel} title={args.title} confirmText={args.confirmText} cancelText={args.cancelText}>
      {#snippet content()}
        <p>Are you sure you want to delete this note?</p>
        <p style="margin-top: 0.5rem; font-size: 0.85rem; color: var(--color-text-muted); font-style: italic">"This is a preview of the note content..."</p>
      {/snippet}
    </ConfirmModal>
  {/snippet}
</Story>

<Story name="Closed" args={{ isShow: false, title: 'Confirm', confirmText: 'Confirm', cancelText: 'Cancel', onconfirm: () => {}, oncancel: () => {} }}>
  {#snippet template(args)}
    <ConfirmModal isShow={args.isShow} onconfirm={args.onconfirm} oncancel={args.oncancel} title={args.title} confirmText={args.confirmText} cancelText={args.cancelText}>
      {#snippet content()}
        <p>Hidden confirm modal.</p>
      {/snippet}
    </ConfirmModal>
  {/snippet}
</Story>
