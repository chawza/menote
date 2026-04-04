<script lang="ts">
  import { commands, type NoteDetail } from "../bindings";
  import { onMount } from "svelte";

  let notes = $state<NoteDetail[]>([]);
  let selectedNote = $state<NoteDetail | null>(null);
  let isDrawerOpen = $state(false);
  let contentForm = $state("");
  let isUpdating = $state(false);

  onMount(async () => {
    notes = await commands.getNotes(1);
  });

  const formatLocal = (ts: number) => {
    const date = new Date(ts * 1000);
    return date.toLocaleString(undefined, {
      dateStyle: "medium",
      timeStyle: "short",
    });
  };

  function openDrawer(note: NoteDetail) {
    selectedNote = note;
    contentForm = note.content || "";
    isDrawerOpen = true;
  }

  function closeDrawer() {
    isDrawerOpen = false;
    setTimeout(() => {
      selectedNote = null;
    }, 300);
  }

  async function handleUpdate(note: NoteDetail) {
    isUpdating = true;
    try {
      // TODO: Implement update logic
      await new Promise((resolve) => setTimeout(resolve, 500));
      closeDrawer();
    } finally {
      isUpdating = false;
    }
  }
</script>

<main class="main">
  <div class="main__container">
    <header class="main__header">
      <h1 class="main__title">MeNote</h1>
      <a href="/notes/create" class="main__new-btn">+ New Note</a>
    </header>

    <div class="notes">
      {#each notes as note (note.id)}
        <div class="note-card" role="button" tabindex="0" onclick={() => openDrawer(note)} onkeydown={(e) => e.key === 'Enter' && openDrawer(note)}>
          <p class="note-card__content">{note.content}</p>
          <div class="note-card__meta">
            <span class="note-card__date">Created: {formatLocal(note.created_at)}</span>
            <span class="note-card__date note-card__date--updated">Updated: {formatLocal(note.updated_at)}</span>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <div class="drawer-overlay" class:drawer-overlay--open={isDrawerOpen} onclick={closeDrawer} onkeydown={(e) => e.key === 'Escape' && closeDrawer()} role="button" tabindex="-1"></div>

  <div class="drawer" class:drawer--open={isDrawerOpen}>
    <div class="drawer__handle"></div>
    {#if selectedNote}
      <div class="drawer__header">
        <h2 class="drawer__title">Edit Note</h2>
        <button class="drawer__close" onclick={closeDrawer}>×</button>
      </div>

      <div class="drawer__meta">
        <span class="drawer__date">Created: {formatLocal(selectedNote.created_at)}</span>
        <span class="drawer__date">Updated: {formatLocal(selectedNote.updated_at)}</span>
      </div>

      <div class="drawer__form">
        <textarea class="drawer__textarea" bind:value={contentForm}></textarea>
        <button
          class="drawer__submit"
          onclick={() => selectedNote && handleUpdate(selectedNote)}
          disabled={isUpdating}
        >
          {isUpdating ? "Updating..." : "Update"}
        </button>
      </div>
    {/if}
  </div>
</main>

<style>
  :root {
    --color-bg: #faf8f5;
    --color-surface: #ffffff;
    --color-text: #2d2a26;
    --color-text-muted: #8a857d;
    --color-border: #e8e4df;
    --color-accent: #c4a77d;
    --color-accent-hover: #b3976d;
    --font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --color-bg: #1a1816;
      --color-surface: #252220;
      --color-text: #f0ebe5;
      --color-text-muted: #9a948c;
      --color-border: #3a3632;
      --color-accent: #d4b78d;
      --color-accent-hover: #c4a77d;
    }
  }

  * {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  .main {
    font-family: var(--font-family);
    background-color: var(--color-bg);
    color: var(--color-text);
    min-height: 100vh;
    padding: 2rem;
  }

  .main__container {
    max-width: 800px;
    margin: 0 auto;
  }

  .main__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .main__title {
    font-size: 1.75rem;
    font-weight: 600;
  }

  .main__new-btn {
    background-color: var(--color-accent);
    color: var(--color-bg);
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    text-decoration: none;
    font-weight: 500;
    transition: background-color 0.2s ease;
  }

  .main__new-btn:hover {
    background-color: var(--color-accent-hover);
  }

  .notes {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .note-card {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 0.75rem;
    padding: 1.25rem;
    cursor: pointer;
    transition: box-shadow 0.2s ease, transform 0.2s ease;
  }

  .note-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    transform: translateY(-2px);
  }

  .note-card__content {
    font-size: 1rem;
    line-height: 1.5;
    margin-bottom: 0.75rem;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .note-card__meta {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .note-card__date {
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }

  .note-card__date--updated {
    opacity: 0.7;
  }

  .drawer-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.4);
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s ease, visibility 0.3s ease;
    z-index: 10;
  }

  .drawer-overlay--open {
    opacity: 1;
    visibility: visible;
  }

  .drawer {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: var(--color-surface);
    border-radius: 1.5rem 1.5rem 0 0;
    padding: 1rem 1.5rem 2rem;
    transform: translateY(100%);
    transition: transform 0.3s ease;
    z-index: 20;
    max-height: 80vh;
    overflow-y: auto;
    max-width: 800px;
    margin: auto;
  }

  .drawer--open {
    transform: translateY(0);
  }

  .drawer__handle {
    width: 2.5rem;
    height: 0.25rem;
    background-color: var(--color-border);
    border-radius: 0.125rem;
    margin: 0 auto 1rem;
  }

  .drawer__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .drawer__title {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .drawer__close {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
  }

  .drawer__close:hover {
    color: var(--color-text);
  }

  .drawer__meta {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }

  .drawer__date {
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }

  .drawer__form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .drawer__textarea {
    width: 100%;
    min-height: 150px;
    padding: 1rem;
    border: 1px solid var(--color-border);
    border-radius: 0.75rem;
    font-family: inherit;
    font-size: 0.875rem;
    resize: vertical;
    background-color: var(--color-bg);
    color: var(--color-text);
  }

  .drawer__textarea:focus {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }

  .drawer__submit {
    background-color: var(--color-accent);
    color: var(--color-bg);
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 0.75rem;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .drawer__submit:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .drawer__submit:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
