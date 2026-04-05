<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import { toastStore } from '../stores/toast';
import ToastContainer from './ToastContainer.svelte';

const { Story } = defineMeta({
  component: ToastContainer,
  tags: ['autodocs'],
});

function addSampleToasts() {
  toastStore.success('Note saved');
  toastStore.info('Syncing...');
  toastStore.warning('Low disk space');
}

function addAllTypes() {
  toastStore.success('Operation successful');
  toastStore.info('Tip: Use keyboard shortcuts');
  toastStore.warning('Auto-save paused');
  toastStore.error('Connection lost');
}
</script>

<Story name="Empty">
  {#snippet template()}
    <div style="position: relative; width: 100%; height: 400px; border: 1px dashed var(--color-border); border-radius: var(--radius-md);">
      <ToastContainer />
    </div>
  {/snippet}
</Story>

<Story name="Single Toast">
  {#snippet template()}
    <div style="position: relative; width: 100%; height: 400px; border: 1px dashed var(--color-border); border-radius: var(--radius-md);">
      <ToastContainer />
    </div>
    {@const _ = toastStore.success('Note created successfully!')}
  {/snippet}
</Story>

<Story name="Multiple Toasts">
  {#snippet template()}
    <div style="position: relative; width: 100%; height: 400px; border: 1px dashed var(--color-border); border-radius: var(--radius-md);">
      <ToastContainer />
    </div>
    {@const _ = addSampleToasts()}
  {/snippet}
</Story>

<Story name="All Types">
  {#snippet template()}
    <div style="position: relative; width: 100%; height: 400px; border: 1px dashed var(--color-border); border-radius: var(--radius-md);">
      <ToastContainer />
    </div>
    {@const _ = addAllTypes()}
  {/snippet}
</Story>
