<script lang="ts" module>
import { defineMeta } from '@storybook/addon-svelte-csf';
import AuthFooter from './AuthFooter.svelte';

const { Story } = defineMeta({
  component: AuthFooter,
  tags: ['autodocs'],
});
</script>

<Story name="Default">
	{#snippet template()}
		<AuthFooter />
	{/snippet}
</Story>
