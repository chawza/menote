<script lang="ts">
  import { toastStore } from '../stores/toast';
  import Toast from './Toast.svelte';
  import { fly } from 'svelte/transition';
  import type { Toast as ToastInterface } from '../types/toast';

  let toasts = $state<ToastInterface[]>([]);

  toastStore.subscribe(value => {
    toasts = value;
  });

  function handleDismiss(id: string) {
    toastStore.dismiss(id);
  }
</script>

{#if toasts.length > 0}
  <div class="toast-container">
    {#each toasts as toast (toast.id)}
      <div class="toast-wrapper" transition:fly={{ x: 100, duration: 300 }}>
        <Toast {toast} ondismiss={(id: string) => handleDismiss(id)} />
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    display: flex;
    flex-direction: column-reverse;
    gap: 0.75rem;
    z-index: 1000;
    max-height: calc(100vh - 4rem);
    overflow-y: auto;
    overflow-x: hidden;
  }

  @media (max-width: 640px) {
    .toast-container {
      left: 1rem;
      right: 1rem;
      bottom: 1rem;
    }
  }
</style>
