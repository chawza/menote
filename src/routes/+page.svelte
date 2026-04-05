<script lang="ts">
import { onMount } from 'svelte';
import { type NoteDetail } from '../bindings';
import ConfirmModal from '../lib/components/ConfirmModal.svelte';
import Modal from '../lib/components/Modal.svelte';
import ToastContainer from '../lib/components/ToastContainer.svelte';
import { toastStore } from '../lib/stores/toast';
import { commands, tryCommand } from '../lib/utils/tauri';

let notes = $state<NoteDetail[]>([]);
let selectedNote = $state<NoteDetail | null>(null);
let isDrawerOpen = $state(false);
let contentForm = $state('');
let isUpdating = $state(false);
let isCreateModalOpen = $state(false);
let newNoteContent = $state('');
let isCreating = $state(false);
let isDeleteModalOpen = $state(false);
let noteToDelete = $state<NoteDetail | null>(null);

onMount(async () => {
  notes = (await tryCommand(() => commands.getNotes(1))).sort(
    (a, b) => b.created_at - a.created_at,
  );
});

const formatLocal = (ts: number) => {
  const date = new Date(ts * 1000);
  return date.toLocaleString(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short',
  });
};

function openDrawer(note: NoteDetail) {
  selectedNote = note;
  contentForm = note.content || '';
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
    const updatedNote = await tryCommand(() =>
      commands.updateNote({
        id: note.id,
        content: contentForm,
        updated_at: Math.floor(Date.now() / 1000),
      }),
    );
    notes = notes.map((n) => (n.id === updatedNote.id ? updatedNote : n));
    toastStore.success('Note updated successfully!');
    closeDrawer();
  } catch {
  } finally {
    isUpdating = false;
  }
}

function handleDelete(note: NoteDetail) {
  noteToDelete = note;
  isDeleteModalOpen = true;
}

function cancelDelete() {
  isDeleteModalOpen = false;
  noteToDelete = null;
}

async function confirmDelete() {
  const note = noteToDelete;
  if (!note) return;
  try {
    await tryCommand(() => commands.deleteNote(note.id));
    notes = notes.filter((n) => n.id !== note.id);
    toastStore.success('Note deleted');
  } catch {}
  cancelDelete();
}

async function handleCreateNote() {
  if (!newNoteContent.trim()) {
    toastStore.error('Please enter some content');
    return;
  }
  isCreating = true;
  const now = Math.round(Date.now() / 1000);
  try {
    const newNote = await tryCommand(() =>
      commands.createNote({
        user_id: 1,
        content: newNoteContent,
        created_at: now,
        updated_at: now,
      }),
    );
    notes = [...notes, newNote].sort((a, b) => b.created_at - a.created_at);
    toastStore.success('Note created successfully!');
    closeCreateModal();
  } catch {
  } finally {
    isCreating = false;
  }
}

function openCreateModal() {
  isCreateModalOpen = true;
}

function closeCreateModal() {
  isCreateModalOpen = false;
  newNoteContent = '';
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
        <div class="card note-card" role="button" tabindex="0" onclick={() => openDrawer(note)} onkeydown={(e) => e.key === 'Enter' && openDrawer(note)}>
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

  <div class="overlay drawer-overlay" class:overlay--open={isDrawerOpen} onclick={closeDrawer} onkeydown={(e) => e.key === 'Escape' && closeDrawer()} role="button" tabindex="-1"></div>

  <div class="drawer" class:drawer--open={isDrawerOpen}>
    <div class="drawer__handle"></div>
    {#if selectedNote}
      <div class="drawer__header">
        <h2 class="drawer__title">Edit Note</h2>
        <button class="close-btn drawer__close" onclick={closeDrawer}>×</button>
      </div>

      <div class="drawer__meta">
        <span class="drawer__date">Created: {formatLocal(selectedNote.created_at)}</span>
        <span class="drawer__date">Updated: {formatLocal(selectedNote.updated_at)}</span>
      </div>

      <div class="drawer__form">
        <textarea class="input drawer__textarea" bind:value={contentForm}></textarea>
        <button
          class="btn btn--primary drawer__submit"
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

  <Modal isShow={isCreateModalOpen} onclose={closeCreateModal} title="New Note">
    {#snippet children()}
      <div class="create-form">
        <textarea
          class="input create-form__textarea"
          bind:value={newNoteContent}
          placeholder="Write your note here..."
        ></textarea>
        <button
          class="btn btn--primary create-form__submit"
          onclick={handleCreateNote}
          disabled={isCreating}
        >
          {isCreating ? "Creating..." : "Create"}
        </button>
      </div>
    {/snippet}
  </Modal>
</main>

<ConfirmModal
  isShow={isDeleteModalOpen}
  onconfirm={confirmDelete}
  oncancel={cancelDelete}
  title="Delete Note"
  confirmText="Delete"
  cancelText="Cancel"
>
  {#snippet content()}
    <p>Are you sure you want to delete this note?</p>
    {#if noteToDelete}
      <p class="delete-preview">"{(noteToDelete.content || 'this note').substring(0, 50)}..."</p>
    {/if}
  {/snippet}
</ConfirmModal>

<ToastContainer />

<style>
  .main {
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
    cursor: pointer;
    transition: box-shadow var(--transition-fast), transform var(--transition-fast);
  }

  .note-card:hover {
    box-shadow: var(--shadow-sm);
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
    transition: opacity var(--transition-fast), color var(--transition-fast);
    flex-shrink: 0;
  }

  .note-card:hover .note-card__delete {
    opacity: 1;
  }

  .note-card__delete:hover {
    color: var(--color-danger);
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
    transition: opacity var(--transition-fast);
  }

  .note-card:hover .note-card__date--updated {
    opacity: 0.7;
  }

  .drawer-overlay {
    z-index: 10;
  }

  .drawer {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: var(--color-surface);
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    padding: 1rem 1.5rem 2rem;
    transform: translateY(100%);
    transition: transform var(--transition-normal);
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
    font-size: 1.5rem;
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
    min-height: 150px;
  }

  .drawer__submit {
    width: 100%;
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
    box-shadow: var(--shadow-sm);
    transition: background-color var(--transition-fast), transform var(--transition-fast), box-shadow var(--transition-fast);
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

  .create-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .create-form__textarea {
    min-height: 200px;
    font-size: 1rem;
  }

  .create-form__submit {
    width: 100%;
    font-size: 1rem;
    padding: 0.875rem 1.5rem;
  }

  .delete-preview {
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: var(--color-text-muted);
    font-style: italic;
  }

  @media (max-width: 640px) {
    .create-note-fab {
      bottom: 1.5rem;
      right: 1.5rem;
      width: 48px;
      height: 48px;
      font-size: 1.75rem;
    }

    .create-form__textarea {
      min-height: 150px;
      font-size: 0.875rem;
    }
  }
</style>
