<script lang="ts">
  import { commands, type NoteDetail } from "../bindings";
  import { onMount } from "svelte";

  let notes = $state<NoteDetail[]>([]);

  onMount(async () => {
    notes = await commands.getNotes(1); // TODO: auth
  });

  const formatLocal = (ts: number) => {
    // If timestamp is in seconds, multiply by 1000 for JS Date
    const date = new Date(ts * 1000);
    return date.toLocaleString(undefined, {
      dateStyle: "medium",
      timeStyle: "short",
    });
  };
</script>

<main class="container">
  <h1>Welcome to Menote!</h1>
  <div>
    <a href="/notes/create">New</a>
  </div>

  <table>
    <thead>
      <tr>
        <th>ID</th>
        <th class="user-table--head">Content</th>
        <th class="user-table--head">Created</th>
      </tr>
    </thead>
    <tbody>
      {#each notes as note (note.id)}
        <tr>
          <td>{note.id}</td>
          <td class="user-table--head">{note.content}</td>
          <td>{formatLocal(note.created_at)}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</main>

<style>
  .user-table--head {
    text-align: left;
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }
  h1 {
    text-align: center;
  }
  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }
  }
</style>
