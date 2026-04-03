<script lang="ts">
    import { goto } from "$app/navigation";
    import { commands } from "../../../bindings";

  let content = $state("");
  let errorText = $state("");
  let isloading = $state(false);

  async function submit() {
    if (!content.trim()) {
      errorText = "Please enter some content";
      return;
    }
    errorText = ""
    isloading = true
    const now = Math.round(Date.now() / 1000);
    try {
      await commands.createNote({
        user_id: 1,
        content: content,
        created_at: now,
        updated_at: now
      })
      await goto("/")
    }
    catch (e) {
      console.error(e)
    }

    finally {
      isloading = false
    }

  }

</script>

<main class="container">
  <h1>Create</h1>

  <div class="form">
    <textarea bind:value={content}></textarea>
    {#if errorText}
    <p class="error">{errorText}</p>
    {/if}
    <button type="button" onclick={submit} disabled={isloading}>Submit</button>
  </div>

</main>

<style>
.form {
    max-width: 300px;
    height: fifit-content;
    padding: 10px;
    margin: auto;
}

textarea {
    width: 100%;
    min-height: 200px;
}

button[type=button] {
    width: 100%;
}

</style>
