<script lang="ts">
    let {
        value = $bindable(),
        disabled
    }: {
        value: boolean;
        disabled?: boolean;
    } = $props();
</script>

<label class="toggle" class:disabled>
    <input bind:checked={value} type="checkbox" {disabled} />
    <!-- svelte-ignore element_invalid_self_closing_tag -->
    <span class="slider" />
</label>

<style>
    .toggle {
        --toggle-width: 45px;
        --toggle-height: 22.5px;

        position: relative;
        display: flex;
        align-items: center;
        cursor: pointer;
        user-select: none;
        width: var(--toggle-width);
        height: var(--toggle-height);
    }

    .toggle.disabled {
        filter: brightness(0.75);
        cursor: not-allowed;
    }

    .toggle input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .toggle .slider {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;

        background-color: var(--dfg-background-2);
        border-radius: 35px;
        transition: 0.4s;
    }

    .toggle .slider:before {
        position: absolute;
        content: "";
        height: calc(var(--toggle-height) - 10px);
        width: calc(var(--toggle-height) - 10px);
        left: 5px;
        bottom: 5px;

        background-color: var(--dfg-text);
        border-radius: 50%;
        transition: 0.4s;
    }

    .toggle input:checked + .slider {
        background-color: var(--dfg-primary);
    }

    .toggle input:focus + .slider {
        box-shadow: 0 0 1px var(--dfg-primary);
    }

    .toggle input:checked + .slider:before {
        transform: translateX(calc(var(--toggle-width) - var(--toggle-height)));
    }
</style>
