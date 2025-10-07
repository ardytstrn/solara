<script lang="ts">
    import '../app.css';
    import { getCurrentWindow } from '@tauri-apps/api/window';
</script>

<div class="app">
    <div class="title-bar" data-tauri-drag-region>
        <button class="title-bar-button" onclick={() => getCurrentWindow().minimize()} aria-label="Minimize">
            <svg width="12" height="12" viewBox="0 0 12 12">
                <line x1="0" y1="12" x2="12" y2="12" stroke="white" stroke-width="1" />
            </svg>
        </button>

        <button class="title-bar-button" onclick={() => getCurrentWindow().close()} aria-label="Close">
            <svg viewBox="0 0 12 12"><path d="M0 0 L12 12 M0 12 L12 0" stroke="white" stroke-width="0.5"/></svg>
        </button>
    </div>

    <slot />
</div>

<style>
    .app {
        position: relative;
        height: 100vh;
        width: 100vw;
        background: var(--color-background);
    }

    .title-bar {
        position: absolute;
        top: 0;
        right: 0;
        padding: 10px;
        z-index: 1000;
        display: flex;
        justify-content: flex-end;
        width: 100%;
        gap: 8px;
    }

    .title-bar-button {
        display: grid;
        place-items: center;
        opacity: 0.4;
        background: transparent;
        padding: 0;
        width: 28px;
        height: 28px;
        transition: opacity 0.2s ease;
    }

    .title-bar-button svg {
        width: 50%;
        height: 50%;
    }

    .title-bar-button:hover {
        opacity: 1;
    }
</style>