<script lang="ts">
import type { Snippet } from 'svelte';
import Modal from './Modal.svelte';

interface Props {
  isShow: boolean;
  content: Snippet;
  onconfirm: () => void;
  oncancel: () => void;
  title?: string;
  confirmText?: string;
  cancelText?: string;
}

let {
  isShow,
  content,
  onconfirm,
  oncancel,
  title = 'Confirm',
  confirmText = 'Confirm',
  cancelText = 'Cancel',
}: Props = $props();
</script>

<Modal isShow={isShow} onclose={oncancel} title={title}>
  {#snippet children()}
    <div class="confirm__content">
      {@render content()}
    </div>

    <div class="confirm__actions">
      <button class="btn btn--ghost confirm__btn confirm__btn--cancel" onclick={oncancel}>{cancelText}</button>
      <button class="btn btn--danger confirm__btn confirm__btn--confirm" onclick={onconfirm}>{confirmText}</button>
    </div>
  {/snippet}
</Modal>

<style>
  .confirm__content {
    margin-bottom: 1.25rem;
  }

  .confirm__actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }
</style>
