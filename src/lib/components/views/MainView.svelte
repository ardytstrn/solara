<script lang="ts">
    import type { ActivePage } from "$lib/types";

    const { activePage }: {
        activePage: ActivePage;
    } = $props();

    import HomePage from "$lib/components/pages/HomePage.svelte";
</script>

<main class="main-view-container">
    <div class="content-area">
        {#if activePage === 'home'}
            <HomePage />
        {:else}
            Active page: {activePage}
        {/if}
    </div>
</main>

<style>
    .content-area {
        flex-grow: 1;
        display: flex;
        position: relative;
    }
</style>