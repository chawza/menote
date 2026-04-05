<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import type { Toast as ToastType } from '../types/toast';
import Toast from './Toast.svelte';

const { Story } = defineMeta({
  component: Toast,
  tags: ['autodocs'],
});

function createToast(overrides: Partial<ToastType> = {}): ToastType {
  return {
    id: 'toast-1',
    type: 'success',
    message: 'This is a toast message',
    showIcon: true,
    duration: 0,
    ...overrides,
  };
}
</script>

<Story name="Success" args={{ toast: createToast({ type: 'success', message: 'Note saved successfully!' }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="Info" args={{ toast: createToast({ type: 'info', message: 'New version available' }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="Warning" args={{ toast: createToast({ type: 'warning', message: 'You have unsaved changes' }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="Error" args={{ toast: createToast({ type: 'error', message: 'Failed to save note' }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="With Progress Bar" args={{ toast: createToast({ type: 'success', message: 'Auto-dismissing in 5s...', duration: 5000 }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="Without Icon" args={{ toast: createToast({ type: 'info', message: 'No icon toast', showIcon: false }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="With Confirm Action" args={{ toast: createToast({ type: 'warning', message: 'You have unsaved changes', confirmText: 'Discard', onConfirm: () => {} }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>

<Story name="Long Message" args={{ toast: createToast({ type: 'info', message: 'This is a very long toast message that demonstrates how the component handles text overflow and wrapping behavior in the UI.' }), ondismiss: () => {} }}>
  {#snippet template(args)}
    <Toast toast={args.toast} ondismiss={args.ondismiss} />
  {/snippet}
</Story>
