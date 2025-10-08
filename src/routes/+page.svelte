<script lang="ts">
  import type {ActivePage} from "$lib/types";

  let activePage = $state<ActivePage>('accounts');

  function handleNavigation(page: ActivePage) {
    activePage = page;
  }

  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import MainView from "$lib/components/views/MainView.svelte";
</script>

<main class="container">
  <Sidebar navigate={handleNavigation} {activePage}/>
  <MainView {activePage}/>
</main>

<style>
  .container {
    display: flex;
    width: 100%;
    height: 100%;
  }
</style>