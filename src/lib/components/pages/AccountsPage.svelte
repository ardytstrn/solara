<script>
</script>

<div class="content-area">
    <div class="top-content">
        <div class="left-column">
            <div class="widget">
                <h2 class="widget-title">Accounts</h2>
                <div class="account-list">
                    <div class="account-item">
                        <img src="https://s.namemc.com/2d/skin/face.png?id=b3c20094038702c6&scale=4">
                        <div class="account-info">
                            <span class="name">nick1</span>
                            <span class="type">Microsoft Account</span>
                        </div>
                    </div>
                    <div class="account-item">
                        <img src="https://s.namemc.com/2d/skin/face.png?id=b84ac9a32c374ef1&scale=4">
                        <div class="account-info">
                            <span class="name">nick2</span>
                            <span class="type">Offline Account</span>
                        </div>
                    </div>
                    <div class="account-item active">
                        <img src="https://s.namemc.com/2d/skin/face.png?id=071a5adae0927f93&scale=4">
                        <div class="account-info">
                            <span class="name">nick3</span>
                            <span class="type">Microsoft Account</span>
                        </div>
                    </div>
                    <div class="account-item">
                        <img src="https://s.namemc.com/2d/skin/face.png?id=458a9409b73a9618&scale=4">
                        <div class="account-info">
                            <span class="name">nick4</span>
                            <span class="type">Offline Account</span>
                        </div>
                    </div>

                    <button class="add-account-button">+ Add Account</button>
                </div>
            </div>
        </div>
        <div class="right-column">
            <div class="widget">
                <h2 class="widget-title">You</h2>
                <div class="data-row">
                    <span class="label">Nickname</span>
                    <span class="value">nick3</span>
                </div>
                <div class="data-row">
                    <span class="label">Email</span>
                    <span class="value">example3@hotmail.com</span>
                </div>
                <div class="data-row">
                    <span class="label">Account Type</span>
                    <span class="value">Microsoft Account</span>
                </div>
            </div>
            <div class="widget">
                <h2 class="widget-title">Statistics</h2>
                <div class="data-row">
                    <span class="label">Total Playtime</span>
                    <span class="value">275 hours</span>
                </div>
                <div class="data-row">
                    <span class="label">Favorite Instance</span>
                    <span class="value">Fabric 1.20.4</span>
                </div>
                <div class="data-row">
                    <span class="label">Last Played</span>
                    <span class="value">Today</span>
                </div>
            </div>
        </div>
    </div>

    <div class="widget full-width">
        <h2 class="widget-title">Skin & Cape</h2>
        <div class="skin-widget-content">
            <div class="skin-preview">
                <img src="https://s.namemc.com/3d/skin/body.png?id=071a5adae0927f93&model=slim&width=384&height=384" alt="Minecraft Skin Preview">
            </div>

            <div class="info-panel">
                <h3 class="title">Your Look</h3>
                <p class="description">Your current skin and cape are linked to your Microsoft Account. To make changes please visit the official Minecraft website. Your changes will sync automatically.</p>
                <div class="action-buttons">
                    <button class="primary">
                        Manage on Minecraft.net
                    </button>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .content-area {
        padding: 60px 30px;
        max-width: 1000px;
        display: flex;
        flex-direction: column;
        gap: 24px;
        overflow: auto;
        height: 100vh;
    }

    .top-content {
        display: flex;
        gap: 24px;
        width: 100%;
    }

    .left-column {
        flex: 1;
        display: flex;
    }

    .full-width {
        width: 100%;
    }

    .right-column {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 24px;
    }

    .left-column > div, .right-column > div {
        flex: 1;
    }

    .widget {
        background-color: #24252D;
        border-radius: 12px;
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .data-row {
        display: flex;
        justify-content: space-between;
        font-size: 14px;
    }

    .data-row .label {
        color: var(--color-text-secondary);
    }

    .data-row .value {
        color: var(--color-text-primary);
        font-weight: 500;
    }

    .widget-title {
        color: var(--color-text-secondary);
        font-size: 14px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        margin: -24px -24px 0 -24px;
        padding: 16px 24px;
    }

    .skin-widget-content {
        display: flex;
        height: 100%;
        gap: 24px;
        align-items: center;
    }

    .skin-preview img {
        height: 200px;
    }

    .info-panel {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .info-panel .title {
        font-size: 20px;
        font-weight: 700;
        margin: 0;
    }

    .info-panel .description {
        font-size: 14px;
        color: var(--color-text-secondary);
        margin: 0;
        line-height: 1.6;
        max-width: 400px;
    }

    .action-buttons button.primary {
        background-color: var(--color-accent);
        border-color: var(--color-accent);
        color: white;
    }

    .action-buttons button.primary:hover {
        background-color: var(--color-accent-dark);
        border-color: var(--color-accent-dark);
    }

    .account-list {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .account-item {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 10px;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .account-item:hover {
        background-color: #2C2D37;
    }

    .account-item.active {
        background-color: #2C2D37;
    }

    .account-item img {
        width: 40px;
        height: 40px;
        border-radius: 20%;
    }

    .account-info {
        display: flex;
        flex-direction: column;
        gap: 3px;
    }

    .account-info .name {
        font-weight: 600;
    }

    .account-info .type {
        font-size: 12px;
        color: var(--color-text-secondary);
    }

    .add-account-button {
        margin-top: 12px;
        padding: 12px;
        border: 1px dashed var(--color-text-secondary);
        border-radius: 8px;
        color: var(--color-text-secondary);
        background: none;
        cursor: pointer;
        width: 100%;
        transition: all 0.2s ease;
        text-transform: uppercase;
        font-weight: 600;
    }

    .add-account-button:hover {
        border-color: var(--color-accent);
        color: var(--color-accent);
    }
</style>