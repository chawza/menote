<script lang="ts">
import type { Snippet } from 'svelte';

interface Props {
  variant?: 'primary' | 'danger' | 'ghost';
  type?: 'button' | 'submit' | 'reset';
  disabled?: boolean;
  class?: string;
  onclick?: (e: MouseEvent) => void;
  children: Snippet;
}

let {
  variant,
  type = 'button',
  disabled = false,
  class: className = '',
  onclick,
  children,
}: Props = $props();

let classes = $derived(
  ['btn', variant ? `btn--${variant}` : '', className]
    .filter(Boolean)
    .join(' '),
);
</script>

<button {type} {disabled} class={classes} {onclick}>
  {@render children()}
</button>
