<script lang="ts">
    let { position = "right", delay = 400, text }: {
        position?: "right" | "left" | "top" | "bottom",
        text: string,
    } = $props();
</script>

<div class="tooltip-container {position}">
    <slot/>
    <span class="tooltip">{text}</span>
</div>

<style>
    .tooltip-container {
        position: relative;
        display: inline-block;
        cursor: pointer;
    }

    .tooltip {
        position: absolute;
        opacity: 0;
        visibility: hidden;
        color: var(--color-text-primary);
        background-color: #18191C;
        padding: 8px 14px;
        border-radius: 6px;
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        transition: all 0.3s ease;
    }

    .tooltip-container:hover .tooltip {
        visibility: visible;
        opacity: 1;
        transition-delay: 0.4s;
    }

    .tooltip-container.right .tooltip {
        top: 50%;
        left: 100%;
        transform: translateY(-50%) translateX(12px);
    }

    .tooltip-container.left .tooltip {
        top: 50%;
        right: 100%;
        transform: translateY(-50%) translateX(-12px);
    }

    .tooltip-container.top .tooltip {
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%) translateY(-12px);
    }

    .tooltip-container.bottom .tooltip {
        top: 100%;
        left: 50%;
        transform: translateX(-50%) translateY(12px);
    }
</style>