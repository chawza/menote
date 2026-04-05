<script lang="ts">
  import type { Toast as ToastType } from '../types/toast';
  import { onMount } from 'svelte';

  interface Props {
    toast: ToastType;
    ondismiss?: (id: string) => void;
  }

  let { toast, ondismiss }: Props = $props();

  let progress = $state(100);

  onMount(() => {
    if (toast.duration && toast.duration > 0) {
      const startTime = Date.now();
      const duration = toast.duration;
      let animationFrame: number;

      function animate() {
        const elapsed = Date.now() - startTime;
        progress = Math.max(0, 100 - (elapsed / duration) * 100);

        if (progress > 0) {
          animationFrame = requestAnimationFrame(animate);
        }
      }

      animationFrame = requestAnimationFrame(animate);

      return () => cancelAnimationFrame(animationFrame);
    }
  });

  function getIcon(type: string): string {
    switch (type) {
      case 'success': return '✓';
      case 'info': return 'ℹ';
      case 'warning': return '⚠';
      case 'error': return '✕';
      default: return '•';
    }
  }

  function handleConfirm() {
    toast.onConfirm?.();
    ondismiss?.(toast.id);
  }
</script>

<div
  class="toast toast--{toast.type}"
  role="alert"
>
  <div class="toast__content">
    {#if toast.showIcon}
      <div class="toast__icon">
        <span class="toast__icon-text">{getIcon(toast.type)}</span>
      </div>
    {/if}

    <div class="toast__body">
      <p class="toast__message">{toast.message}</p>

      {#if toast.onConfirm}
    <button class="btn btn--primary toast__confirm" onclick={handleConfirm}>
          {toast.confirmText}
        </button>
      {/if}
    </div>

    <button class="close-btn toast__close" onclick={() => ondismiss?.(toast.id)} aria-label="Dismiss">
      ×
    </button>
  </div>

  {#if toast.duration && toast.duration > 0}
    <div class="toast__progress">
      <div
        class="toast__progress-bar toast__progress-bar--{toast.type}"
        style="width: {progress}%"
      ></div>
    </div>
  {/if}
</div>

<style>
  .toast {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    overflow: hidden;
    min-width: 300px;
    max-width: 400px;
    transition: transform var(--transition-normal), opacity var(--transition-normal);
  }

  .toast--success {
    border-left: 4px solid var(--toast-success);
  }

  .toast--info {
    border-left: 4px solid var(--toast-info);
  }

  .toast--warning {
    border-left: 4px solid var(--toast-warning);
  }

  .toast--error {
    border-left: 4px solid var(--toast-error);
  }

  .toast__content {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
  }

  .toast__icon {
    flex-shrink: 0;
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .toast--success .toast__icon {
    background-color: var(--toast-success);
    color: white;
  }

  .toast--info .toast__icon {
    background-color: var(--toast-info);
    color: white;
  }

  .toast--warning .toast__icon {
    background-color: var(--toast-warning);
    color: white;
  }

  .toast--error .toast__icon {
    background-color: var(--toast-error);
    color: white;
  }

  .toast__icon-text {
    line-height: 1;
  }

  .toast__body {
    flex: 1;
    min-width: 0;
  }

  .toast__message {
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--color-text);
    margin: 0;
  }

  .toast__confirm {
    margin-top: 0.75rem;
    padding: 0.5rem 1rem;
    font-size: 0.75rem;
  }

  .toast__close {
    font-size: 1.25rem;
  }

  .toast__progress {
    height: 3px;
    background-color: var(--color-border);
    overflow: hidden;
  }

  .toast__progress-bar {
    height: 100%;
    transition: width 0.1s linear;
  }

  .toast__progress-bar--success {
    background-color: var(--toast-success);
  }

  .toast__progress-bar--info {
    background-color: var(--toast-info);
  }

  .toast__progress-bar--warning {
    background-color: var(--toast-warning);
  }

  .toast__progress-bar--error {
    background-color: var(--toast-error);
  }
</style>
