import { writable } from 'svelte/store';
import type { Toast, ToastOptions } from '../types/toast';

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  function add(options: ToastOptions): string {
    const id = `toast-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    const toast: Toast = {
      id,
      type: options.type,
      message: options.message,
      showIcon: options.showIcon ?? true,
      duration: options.duration ?? 5000,
      onConfirm: options.onConfirm,
      confirmText: options.confirmText ?? 'OK',
    };

    update((toasts) => [...toasts, toast].slice(-5));

    if (toast.duration && toast.duration > 0) {
      setTimeout(() => dismiss(id), toast.duration);
    }

    return id;
  }

  function dismiss(id: string) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  function success(message: string, options?: Partial<ToastOptions>): string {
    return add({ ...options, type: 'success', message });
  }

  function info(message: string, options?: Partial<ToastOptions>): string {
    return add({ ...options, type: 'info', message });
  }

  function warning(message: string, options?: Partial<ToastOptions>): string {
    return add({ ...options, type: 'warning', message });
  }

  function error(message: string, options?: Partial<ToastOptions>): string {
    return add({ ...options, type: 'error', message });
  }

  return {
    subscribe,
    add,
    dismiss,
    success,
    info,
    warning,
    error,
  };
}

export const toastStore = createToastStore();
