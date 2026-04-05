<script lang="ts">
import type { Snippet } from 'svelte';

interface Props {
  class?: string;
  onclick?: (e: MouseEvent) => void;
  children: Snippet;
}

let { class: className = '', onclick, children }: Props = $props();

let classes = $derived(['card', className].filter(Boolean).join(' '));
</script>

<div class={classes} {onclick} role={onclick ? 'button' : undefined} tabindex={onclick ? 0 : undefined}>
  {@render children()}
</div>
