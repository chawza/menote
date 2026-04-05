<script lang="ts">
import { page } from '$app/stores';

interface Props {
  isOpen: boolean;
  onclose: () => void;
}

let { isOpen, onclose }: Props = $props();
let currentPath = $derived($page.url.pathname);
</script>

{#if isOpen}
  <div class="sidebar-overlay" onclick={onclose} onkeydown={(e) => e.key === 'Escape' && onclose()} role="button" tabindex="-1"></div>
{/if}

<aside class="sidebar" class:sidebar--open={isOpen}>
  <nav class="sidebar__nav">
    <a href="/notes" class="sidebar__nav-item" class:sidebar__nav-item--active={currentPath.startsWith('/notes')}>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
        <polyline points="14 2 14 8 20 8" />
        <line x1="16" y1="13" x2="8" y2="13" />
        <line x1="16" y1="17" x2="8" y2="17" />
        <polyline points="10 9 9 9 8 9" />
      </svg>
      <span>Notes</span>
    </a>
    <button class="sidebar__nav-item sidebar__nav-item--disabled" disabled>
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2" />
        <circle cx="12" cy="7" r="4" />
      </svg>
      <span>Profile</span>
    </button>
  </nav>

  <div class="sidebar__section">
    <h3 class="sidebar__section-title">Tags</h3>
    <p class="sidebar__placeholder">Coming soon</p>
  </div>
</aside>

<style>
  .sidebar-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.3);
    z-index: 60;
  }

  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 260px;
    background-color: var(--color-surface);
    border-right: 1px solid var(--color-border);
    transform: translateX(-100%);
    transition: transform var(--transition-normal);
    z-index: 65;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
  }

  .sidebar--open {
    transform: translateX(0);
  }

  .sidebar__nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 0.5rem;
  }

  .sidebar__nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border-radius: var(--radius-sm);
    color: var(--color-text);
    text-decoration: none;
    font-size: 0.9375rem;
    border: none;
    background: none;
    width: 100%;
    cursor: pointer;
    transition: background-color var(--transition-fast);
    font-family: inherit;
  }

  .sidebar__nav-item:hover:not(.sidebar__nav-item--disabled) {
    background-color: var(--color-border);
  }

  .sidebar__nav-item--active {
    background-color: var(--color-border);
    font-weight: 500;
  }

  .sidebar__nav-item--disabled {
    color: var(--color-text-muted);
    cursor: not-allowed;
    opacity: 0.5;
  }

  .sidebar__section {
    margin-top: 1.5rem;
    padding: 0 1.25rem;
  }

  .sidebar__section-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
    margin-bottom: 0.5rem;
  }

  .sidebar__placeholder {
    font-size: 0.8125rem;
    color: var(--color-text-muted);
    font-style: italic;
  }
</style>
