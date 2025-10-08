<script lang="ts">
    import type { ActivePage } from "$lib/types";
    import Tooltip from '$lib/components/ui/Tooltip.svelte';
    import UserIcon from "$lib/icons/UserIcon.svelte";
    import InstancesIcon from "$lib/icons/InstancesIcon.svelte";
    import ModsIcon from "$lib/icons/ModsIcon.svelte";
    import PerformanceIcon from "$lib/icons/PerformanceIcon.svelte";
    import AddInstanceIcon from "$lib/icons/AddInstanceIcon.svelte";
    import SettingsIcon from "$lib/icons/SettingsIcon.svelte";

    const { navigate, activePage }: {
        navigate(page: ActivePage): void;
        activePage: ActivePage;
    } = $props();
</script>

<nav class="sidebar">
    <div class="top">
        <svg class="logo" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M18 12C18 15.3137 15.3137 18 12 18C8.68629 18 6 15.3137 6 12C6 8.68629 8.68629 6 12 6C15.3137 6 18 8.68629 18 12Z"/>
            <path fill-rule="evenodd" clip-rule="evenodd" d="M12 1.25C12.4142 1.25 12.75 1.58579 12.75 2V3C12.75 3.41421 12.4142 3.75 12 3.75C11.5858 3.75 11.25 3.41421 11.25 3V2C11.25 1.58579 11.5858 1.25 12 1.25ZM1.25 12C1.25 11.5858 1.58579 11.25 2 11.25H3C3.41421 11.25 3.75 11.5858 3.75 12C3.75 12.4142 3.41421 12.75 3 12.75H2C1.58579 12.75 1.25 12.4142 1.25 12ZM20.25 12C20.25 11.5858 20.5858 11.25 21 11.25H22C22.4142 11.25 22.75 11.5858 22.75 12C22.75 12.4142 22.4142 12.75 22 12.75H21C20.5858 12.75 20.25 12.4142 20.25 12ZM12 20.25C12.4142 20.25 12.75 20.5858 12.75 21V22C12.75 22.4142 12.4142 22.75 12 22.75C11.5858 22.75 11.25 22.4142 11.25 22V21C11.25 20.5858 11.5858 20.25 12 20.25Z"/>
            <g>
                <path d="M4.39838 4.39838C4.69127 4.10549 5.16615 4.10549 5.45904 4.39838L5.85188 4.79122C6.14477 5.08411 6.14477 5.55898 5.85188 5.85188C5.55898 6.14477 5.08411 6.14477 4.79122 5.85188L4.39838 5.45904C4.10549 5.16615 4.10549 4.69127 4.39838 4.39838Z"/>
                <path d="M19.6009 4.39864C19.8938 4.69153 19.8938 5.16641 19.6009 5.4593L19.2081 5.85214C18.9152 6.14503 18.4403 6.14503 18.1474 5.85214C17.8545 5.55924 17.8545 5.08437 18.1474 4.79148L18.5402 4.39864C18.8331 4.10575 19.308 4.10575 19.6009 4.39864Z"/>
                <path d="M18.1474 18.1474C18.4403 17.8545 18.9152 17.8545 19.2081 18.1474L19.6009 18.5402C19.8938 18.8331 19.8938 19.308 19.6009 19.6009C19.308 19.8938 18.8331 19.8938 18.5402 19.6009L18.1474 19.2081C17.8545 18.9152 17.8545 18.4403 18.1474 18.1474Z"/>
                <path d="M5.85188 18.1477C6.14477 18.4406 6.14477 18.9154 5.85188 19.2083L5.45904 19.6012C5.16615 19.8941 4.69127 19.8941 4.39838 19.6012C4.10549 19.3083 4.10549 18.8334 4.39838 18.5405L4.79122 18.1477C5.08411 17.8548 5.55898 17.8548 5.85188 18.1477Z"/>
            </g>
        </svg>
    </div>

    <div class="middle">
        <div class="sidebar-capsule">
            <Tooltip text="Accounts">
                <button
                        class="sidebar-item"
                        aria-label="Accounts"
                        class:active={activePage === 'accounts'}
                        onclick={() => navigate('accounts')}>
                    <UserIcon/>
                </button>
            </Tooltip>
        </div>

        <div class="sidebar-capsule">
            <Tooltip text="Instances">
                <button class="sidebar-item" aria-label="Instances" class:active={activePage === 'instances'} onclick={() => navigate('instances')}>
                    <InstancesIcon/>
                </button>
            </Tooltip>

            <Tooltip text="Mods">
                <button class="sidebar-item" aria-label="Mods" class:active={activePage === 'mods'} onclick={() => navigate('mods')}>
                    <ModsIcon/>
                </button>
            </Tooltip>

            <Tooltip text="Performance Center">
                <button class="sidebar-item" aria-label="Performance Center" class:active={activePage === 'performance'} onclick={() => navigate('performance')}>
                    <PerformanceIcon/>
                </button>
            </Tooltip>
        </div>

        <div class="sidebar-capsule">
            <Tooltip text="Add Instance">
                <button class="sidebar-item" aria-label="Add Instance">
                    <AddInstanceIcon/>
                </button>
            </Tooltip>
        </div>
    </div>

    <div class="bottom">
        <div class="sidebar-capsule">
            <Tooltip text="Settings">
                <button class="sidebar-item" aria-label="Settings" class:active={activePage === 'settings'} onclick={() => navigate('settings')}>
                    <SettingsIcon/>
                </button>
            </Tooltip>
        </div>
    </div>
</nav>

<style>
    .sidebar {
        width: 60px;
        background-color: var(--color-sidebar);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        padding: 16px;
        gap: 25px;
    }

    .sidebar-capsule {
        background-color: rgba(255, 255, 255, 0.05);
        border-radius: 99999px;
        padding: 10px;
        display: flex;
        flex-direction: column;
        gap: 8px;
        align-items: center;
        justify-content: center;
        width: min-content;
    }

    .sidebar-item {
        width: 40px;
        height: 40px;
        background-color: transparent;
        color: var(--color-text-primary);
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        border: none;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    :global(.sidebar-item svg) {
        width: 50%;
        height: 50%;
        fill: var(--color-text-secondary);
        transition: all 0.2s ease;
    }

    .top, .middle, .bottom {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;
    }

    :global(.sidebar-item:hover svg) {
        fill: var(--color-text-primary);
    }

    :global(.sidebar-item.active svg) {
        fill: var(--color-accent);
    }

    .logo {
        width: 40px;
        height: 40px;
        fill: var(--color-accent);
    }
</style>