<script lang="ts">
  import { commands, type NoteDetail } from "../bindings";
  import { onMount } from "svelte";
  import ToastContainer from "../lib/components/ToastContainer.svelte";
  import { toastStore } from "../lib/stores/toast";

  let notes = $state<NoteDetail[]>([]);
  let selectedNote = $state<NoteDetail | null>(null);
  let isDrawerOpen = $state(false);
  let contentForm = $state("");
  let isUpdating = $state(false);
  let isCreateModalOpen = $state(false);
  let newNoteContent = $state("");
  let isCreating = $state(false);

  onMount(async () => {
    notes = (await commands.getNotes(1)).sort((a, b) => b.created_at - a.created_at);
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
      const updatedNote = await commands.updateNote({
        id: note.id,
        content: contentForm,
        updated_at: Math.floor(Date.now() / 1000),
      })
      notes = notes.map(n => n.id === updatedNote.id ? updatedNote : n);
      toastStore.success('Note updated successfully!');
      closeDrawer();
    } finally {
      isUpdating = false;
    }
  }

  function handleDelete(note: NoteDetail) {
    const preview = (note.content || 'this note').substring(0, 20);
    toastStore.warning(`Delete "${preview}..."?`, {
      confirmText: 'Delete',
      onConfirm: () => {
        notes = notes.filter(n => n.id !== note.id);
        toastStore.success('Note deleted');
      }
    });
  }

  async function handleCreateNote() {
    if (!newNoteContent.trim()) {
      toastStore.error("Please enter some content");
      return;
    }
    isCreating = true;
    const now = Math.round(Date.now() / 1000);
    try {
      const newNote = await commands.createNote({
        user_id: 1,
        content: newNoteContent,
        created_at: now,
        updated_at: now
      })
      notes = [...notes, newNote].sort((a, b) => b.created_at - a.created_at);
      toastStore.success('Note created successfully!');
      closeCreateModal();
    } catch (e) {
      console.error(e);
      toastStore.error("Failed to create note");
    } finally {
      isCreating = false;
    }
  }

  function openCreateModal() {
    isCreateModalOpen = true;
  }

  function closeCreateModal() {
    isCreateModalOpen = false;
    newNoteContent = "";
  }
</script>

<main class="main">
  <div class="main__container">
    <header class="main__header">
      <h1 class="main__title">MeNote</h1>
    </header>

    <div class="notes">
      <!-- add unique note by note.id and updated_at also -->
      {#each notes as note (note.id)}
        <div class="note-card" role="button" tabindex="0" onclick={() => openDrawer(note)} onkeydown={(e) => e.key === 'Enter' && openDrawer(note)}>
          <div class="note-card__body">
            <p class="note-card__content">{note.content}</p>
            <button class="note-card__delete" onclick={(e) => { e.stopPropagation(); handleDelete(note); }} aria-label="Delete note">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2M10 11v6M14 11v6"/>
              </svg>
            </button>
          </div>
          <div class="note-card__meta">
            <span class="note-card__date">{formatLocal(note.created_at)}</span>
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

  <button
    class="create-note-fab"
    onclick={openCreateModal}
    aria-label="Create new note"
  >
    +
  </button>

  <div
    class="modal-overlay"
    class:modal-overlay--open={isCreateModalOpen}
    onclick={closeCreateModal}
    onkeydown={(e) => e.key === 'Escape' && closeCreateModal()}
    role="button"
    tabindex="-1"
  ></div>

  <div class="modal" class:modal--open={isCreateModalOpen}>
    <div class="modal__header">
      <h2 class="modal__title">New Note</h2>
      <button class="modal__close" onclick={closeCreateModal}>×</button>
    </div>

    <div class="modal__form">
      <textarea
        class="modal__textarea"
        bind:value={newNoteContent}
        placeholder="Write your note here..."
      ></textarea>
      <button
        class="modal__submit"
        onclick={handleCreateNote}
        disabled={isCreating}
      >
        {isCreating ? "Creating..." : "Create"}
      </button>
    </div>
  </div>
</main>

<ToastContainer />

<style>
  :root {
    --color-bg: #faf8f5;
    --color-surface: #ffffff;
    --color-text: #2d2a26;
    --color-text-muted: #8a857d;
    --color-border: #e8e4df;
    --color-accent: #c4a77d;
    --color-accent-hover: #b3976d;
    --toast-success: #7d9f7d;
    --toast-info: #7d9fc4;
    --toast-warning: #c4a77d;
    --toast-error: #c47d7d;
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
      --toast-success: #8fb08f;
      --toast-info: #8fb0d4;
      --toast-warning: #d4b78d;
      --toast-error: #d48f8f;
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

  .note-card__body {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .note-card__content {
    font-size: 1rem;
    line-height: 1.5;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    flex: 1;
  }

  .note-card__delete {
    background: none;
    border: none;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.25rem;
    opacity: 0;
    transition: opacity 0.2s ease, color 0.2s ease;
    flex-shrink: 0;
  }

  .note-card:hover .note-card__delete {
    opacity: 1;
  }

  .note-card__delete:hover {
    color: #dc3545;
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
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .note-card:hover .note-card__date--updated {
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

  .create-note-fab {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background-color: var(--color-accent);
    color: var(--color-bg);
    border: none;
    font-size: 2rem;
    font-weight: 300;
    line-height: 1;
    cursor: pointer;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    transition: background-color 0.2s ease, transform 0.2s ease, box-shadow 0.2s ease;
    z-index: 30;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .create-note-fab:hover {
    background-color: var(--color-accent-hover);
    transform: scale(1.05);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
  }

  .create-note-fab:active {
    transform: scale(0.95);
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.4);
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s ease, visibility 0.3s ease;
    z-index: 40;
  }

  .modal-overlay--open {
    opacity: 1;
    visibility: visible;
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) scale(0.9);
    background-color: var(--color-surface);
    border-radius: 1rem;
    padding: 1.5rem;
    max-width: 500px;
    width: 90%;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s ease, transform 0.3s ease, visibility 0.3s ease;
    z-index: 50;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  }

  .modal--open {
    opacity: 1;
    visibility: visible;
    transform: translate(-50%, -50%) scale(1);
  }

  .modal__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .modal__title {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .modal__close {
    background: none;
    border: none;
    font-size: 1.75rem;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0.25rem;
    line-height: 1;
    transition: color 0.2s ease;
  }

  .modal__close:hover {
    color: var(--color-text);
  }

  .modal__form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .modal__textarea {
    width: 100%;
    min-height: 200px;
    padding: 1rem;
    border: 1px solid var(--color-border);
    border-radius: 0.75rem;
    font-family: inherit;
    font-size: 1rem;
    line-height: 1.5;
    resize: vertical;
    background-color: var(--color-bg);
    color: var(--color-text);
    transition: border-color 0.2s ease;
  }

  .modal__textarea:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px rgba(196, 167, 125, 0.2);
  }

  .modal__textarea::placeholder {
    color: var(--color-text-muted);
  }

  .modal__submit {
    background-color: var(--color-accent);
    color: var(--color-bg);
    border: none;
    padding: 0.875rem 1.5rem;
    border-radius: 0.75rem;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .modal__submit:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .modal__submit:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  @media (prefers-color-scheme: dark) {
    .modal {
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    }
  }

  @media (max-width: 640px) {
    .create-note-fab {
      bottom: 1.5rem;
      right: 1.5rem;
      width: 48px;
      height: 48px;
      font-size: 1.75rem;
    }

    .modal {
      width: 95%;
      padding: 1.25rem;
    }

    .modal__title {
      font-size: 1.25rem;
    }

    .modal__textarea {
      min-height: 150px;
      font-size: 0.875rem;
    }
  }
</style>
