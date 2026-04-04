<script lang="ts">
  import Modal from "./Modal.svelte";
  import type { Snippet } from 'svelte';

  interface Props {
    isShow: boolean;
    content: Snippet;
    onconfirm: () => void;
    oncancel: () => void;
    title?: string;
    confirmText?: string;
    cancelText?: string;
  }

  let { isShow, content, onconfirm, oncancel, title = "Confirm", confirmText = "Confirm", cancelText = "Cancel" }: Props = $props();
</script>

<Modal isShow={isShow} onclose={oncancel} title={title}>
  {#snippet children()}
    <div class="confirm__content">
      {@render content()}
    </div>

    <div class="confirm__actions">
      <button class="confirm__btn confirm__btn--cancel" onclick={oncancel}>{cancelText}</button>
      <button class="confirm__btn confirm__btn--confirm" onclick={onconfirm}>{confirmText}</button>
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

  .confirm__btn {
    border: none;
    padding: 0.625rem 1.25rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .confirm__btn--cancel {
    background-color: var(--color-border);
    color: var(--color-text);
  }

  .confirm__btn--cancel:hover {
    opacity: 0.8;
  }

  .confirm__btn--confirm {
    background-color: #dc3545;
    color: #ffffff;
  }

  .confirm__btn--confirm:hover {
    background-color: #c82333;
  }
</style>
