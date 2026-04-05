<script lang="ts">
interface Props {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  class?: string;
  oninput?: (e: Event) => void;
}

let {
  value = $bindable(''),
  placeholder = '',
  disabled = false,
  rows,
  class: className = '',
  oninput,
}: Props = $props();

let classes = $derived(['input', className].filter(Boolean).join(' '));
</script>

<textarea
  {placeholder}
  {disabled}
  {rows}
  class={classes}
  bind:value
  oninput={oninput}
></textarea>
