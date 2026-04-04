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
        <button class="toast__confirm" onclick={handleConfirm}>
          {toast.confirmText}
        </button>
      {/if}
    </div>

    <button class="toast__close" onclick={() => ondismiss?.(toast.id)} aria-label="Dismiss">
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
    border-radius: 0.75rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    overflow: hidden;
    min-width: 300px;
    max-width: 400px;
    transition: transform 0.3s ease, opacity 0.3s ease;
  }

  .toast--success {
    border-left: 4px solid var(--toast-success, #7d9f7d);
  }

  .toast--info {
    border-left: 4px solid var(--toast-info, #7d9fc4);
  }

  .toast--warning {
    border-left: 4px solid var(--toast-warning, #c4a77d);
  }

  .toast--error {
    border-left: 4px solid var(--toast-error, #c47d7d);
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
    background-color: var(--toast-success, #7d9f7d);
    color: white;
  }

  .toast--info .toast__icon {
    background-color: var(--toast-info, #7d9fc4);
    color: white;
  }

  .toast--warning .toast__icon {
    background-color: var(--toast-warning, #c4a77d);
    color: white;
  }

  .toast--error .toast__icon {
    background-color: var(--toast-error, #c47d7d);
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
    background-color: var(--color-accent);
    color: var(--color-bg);
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .toast__confirm:hover {
    background-color: var(--color-accent-hover);
  }

  .toast__close {
    flex-shrink: 0;
    background: none;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.25rem;
    font-size: 1.25rem;
    line-height: 1;
    transition: color 0.2s ease;
  }

  .toast__close:hover {
    color: var(--color-text);
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
    background-color: var(--toast-success, #7d9f7d);
  }

  .toast__progress-bar--info {
    background-color: var(--toast-info, #7d9fc4);
  }

  .toast__progress-bar--warning {
    background-color: var(--toast-warning, #c4a77d);
  }

  .toast__progress-bar--error {
    background-color: var(--toast-error, #c47d7d);
  }

  @media (prefers-color-scheme: dark) {
    .toast {
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    }

    :global(:root) {
      --toast-success: #8fb08f;
      --toast-info: #8fb0d4;
      --toast-warning: #d4b78d;
      --toast-error: #d48f8f;
    }
  }
</style>
