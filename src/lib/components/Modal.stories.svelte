<script lang="ts" module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Modal from './Modal.svelte';

  const { Story } = defineMeta({
    component: Modal,
    tags: ['autodocs'],
    argTypes: {
      isShow: { control: 'boolean' },
      title: { control: 'text' },
    },
  });
</script>

<Story name="Open" args={{ isShow: true, onclose: () => {}, title: 'Modal Title' }}>
  {#snippet template(args)}
    <Modal isShow={args.isShow} onclose={args.onclose} title={args.title}>
      {#snippet children()}
        <p>This is the modal body content. You can put anything here.</p>
      {/snippet}
    </Modal>
  {/snippet}
</Story>

<Story name="Closed" args={{ isShow: false, onclose: () => {}, title: 'Modal Title' }}>
  {#snippet template(args)}
    <Modal isShow={args.isShow} onclose={args.onclose} title={args.title}>
      {#snippet children()}
        <p>You cannot see this.</p>
      {/snippet}
    </Modal>
  {/snippet}
</Story>

<Story name="Without Title" args={{ isShow: true, onclose: () => {} }}>
  {#snippet template(args)}
    <Modal isShow={args.isShow} onclose={args.onclose}>
      {#snippet children()}
        <p>A modal without a title bar. The close button is also hidden.</p>
      {/snippet}
    </Modal>
  {/snippet}
</Story>

<Story name="Long Content" args={{ isShow: true, onclose: () => {}, title: 'Terms & Conditions' }}>
  {#snippet template(args)}
    <Modal isShow={args.isShow} onclose={args.onclose} title={args.title}>
      {#snippet children()}
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
        <p style="margin-top: 0.75rem">Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.</p>
        <p style="margin-top: 0.75rem">Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.</p>
      {/snippet}
    </Modal>
  {/snippet}
</Story>
