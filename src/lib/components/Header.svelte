<script lang="ts">
interface Props {
  title?: string;
  onToggleSidebar?: () => void;
}

let { title = 'MeNote', onToggleSidebar }: Props = $props();
</script>

<header class="app-header">
  <button
    class="app-header__menu-btn"
    onclick={() => onToggleSidebar?.()}
    aria-label="Toggle sidebar"
  >
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="3" y1="6" x2="21" y2="6" />
      <line x1="3" y1="12" x2="21" y2="12" />
      <line x1="3" y1="18" x2="21" y2="18" />
    </svg>
  </button>
  <h1 class="app-header__title">{title}</h1>
</header>

<style>
  .app-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    background-color: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
    z-index: 50;
  }

  .app-header__title {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .app-header__menu-btn {
    background: none;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.375rem;
    border-radius: var(--radius-sm);
    transition: color var(--transition-fast), background-color var(--transition-fast);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .app-header__menu-btn:hover {
    color: var(--color-text);
    background-color: var(--color-border);
  }
</style>
