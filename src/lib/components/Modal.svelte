<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    isShow: boolean;
    onclose: () => void;
    title?: string;
    children: Snippet;
  }

  let { isShow, onclose, title, children }: Props = $props();
</script>

<div
  class="overlay modal-overlay"
  class:overlay--open={isShow}
  onclick={onclose}
  onkeydown={(e) => e.key === 'Escape' && onclose()}
  role="button"
  tabindex="-1"
></div>

<div class="modal" class:modal--open={isShow}>
  {#if title}
    <div class="modal__header">
      <h2 class="modal__title">{title}</h2>
      <button class="close-btn modal__close" onclick={onclose}>×</button>
    </div>
  {/if}

  <div class="modal__body">
    {@render children()}
  </div>
</div>

<style>
  .modal-overlay {
    z-index: 60;
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) scale(0.95);
    background-color: var(--color-surface);
    border-radius: var(--radius-lg);
    padding: 1.5rem;
    max-width: 500px;
    width: 90%;
    opacity: 0;
    visibility: hidden;
    transition: opacity var(--transition-fast), transform var(--transition-fast), visibility var(--transition-fast);
    z-index: 70;
    box-shadow: var(--shadow-lg);
  }

  .modal--open {
    opacity: 1;
    visibility: visible;
    transform: translate(-50%, -50%) scale(1);
  }

  .modal__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .modal__title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .modal__close {
    font-size: 1.75rem;
  }

  .modal__body {
    font-size: 0.9rem;
    line-height: 1.5;
    color: var(--color-text);
  }

  @media (prefers-color-scheme: dark) {
    .modal {
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    }
  }

  @media (max-width: 640px) {
    .modal {
      width: 95%;
      padding: 1.25rem;
    }

    .modal__title {
      font-size: 1.125rem;
    }
  }
</style>
