<script lang="ts">
    import type { ActivePage } from "$lib/types";

    const { activePage }: {
        activePage: ActivePage;
    } = $props();

    import AccountsPage from "$lib/components/pages/AccountsPage.svelte";
</script>

<main class="main-view-container">
        {#if activePage === 'accounts'}
            <AccountsPage />
        {:else}
            Active page: {activePage}
        {/if}
</main>

<style>
    .main-view-container {
        width: 90%;
        height: 100%;
    }
</style>