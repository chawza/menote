<script lang="ts">
import Header from '$lib/components/Header.svelte';
import Sidebar from '$lib/components/Sidebar.svelte';

let { children } = $props();
let isSidebarOpen = $state(false);

function toggleSidebar() {
  isSidebarOpen = !isSidebarOpen;
}

function closeSidebar() {
  isSidebarOpen = false;
}
</script>

<div class="app-shell">
  <Header onToggleSidebar={toggleSidebar} />
  <Sidebar isOpen={isSidebarOpen} onclose={closeSidebar} />
  <main class="app-shell__main">
    {@render children()}
  </main>
</div>
