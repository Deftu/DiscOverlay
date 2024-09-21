<script lang="ts">
    import { checkForUpdates, performUpdate } from "$lib/updater";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { getVersion } from "@tauri-apps/api/app";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount, untrack } from "svelte";
    import type { Update } from "@tauri-apps/plugin-updater";
    import { writable } from "svelte/store";

    const appWindow = getCurrentWebviewWindow();

    let version: string | null = $state(null);

    let updateNotFound = writable(false);
    let checkingForUpdates = $state(false);
    let pendingUpdate: Update | null = $state(null);
    const updateProgress = writable(-1);
    let downloadedUpdate = $state(false);

    updateNotFound.subscribe((newValue) => {
        if (newValue) {
            setTimeout(() => {
                updateNotFound.set(false);
            }, 3000);
        }
    });

    onMount(() => {
        getVersion().then((v) => {
            version = v;
        });
    });

    async function minimize() {
        await appWindow.minimize();
    }

    async function close() {
        await invoke("close_completely");
    }

    async function onUpdateButtonClicked(event: MouseEvent) {
        if (checkingForUpdates) {
            return;
        }

        $updateNotFound = false;

        if (downloadedUpdate && pendingUpdate) {
            await pendingUpdate.install();

            return;
        }

        if (pendingUpdate) {
            await performUpdate(pendingUpdate, updateProgress);
            downloadedUpdate = true;
            updateProgress.set(-1);

            return;
        }

        checkingForUpdates = true;
        pendingUpdate = await checkForUpdates();
        checkingForUpdates = false;

        if (!pendingUpdate) {
            $updateNotFound = true;
        }
    }
</script>

<nav class="dfg-titlebar" data-tauri-drag-region>
    <h2 class="dfg-header-2">
        DiscOverlay {#if version}v{version}{/if}
    </h2>

    <div class="dfg-titlebar-buttons">
        <div class="updater">
            <button class="dfg-button" onclick={onUpdateButtonClicked}>
                {#if !pendingUpdate}
                    {#if $updateNotFound}
                        No updates found
                    {:else if checkingForUpdates}
                        Checking...
                    {:else}
                        Check for updates
                    {/if}
                {:else}
                    {#if !downloadedUpdate}
                        {#if $updateProgress === -1}
                            Download update
                        {:else}
                            Downloading... [{$updateProgress}%]
                        {/if}
                    {:else}
                        Click to install update
                    {/if}
                {/if}
            </button>

            {#if pendingUpdate && $updateProgress === -1}
                <button class="dfg-button" onclick={async () => {
                    checkingForUpdates = false;
                    pendingUpdate = null;
                    downloadedUpdate = false;
                    updateProgress.set(-1);
                }}>
                    Cancel update
                </button>
            {/if}
        </div>

        <button class="dfg-button" onclick={minimize}>
            <svg xmlns="http://www.w3.org/2000/svg" width="10" height="2" viewBox="0 0 10 2" fill="none">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M0 0.375H10V1.625H0V0.375Z" fill="#FDFBF9"/>
            </svg>
        </button>
        <button class="dfg-button" onclick={close}>
            <svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10" fill="none">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M9.18798 10L8.76226e-05 0.812103L0.812191 0L10.0001 9.1879L9.18798 10Z" fill="#FDFBF9"/>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M0.812104 10L10 0.812103L9.1879 0L0 9.1879L0.812104 10Z" fill="#FDFBF9"/>
              </svg>
        </button>
    </div>
</nav>

<style lang="scss">
    .dfg-titlebar {
        background: var(--dfg-background-2);

        position: sticky;
        top: 0;

        display: flex;
        width: 100%;
        padding: 10px 25px;
        justify-content: space-between;
        align-items: center;

        user-select: none;

        .dfg-titlebar-buttons {
            display: flex;
            padding: 10px;
            justify-content: center;
            align-items: center;
            gap: 20px;

            .dfg-button {
                background: var(--dfg-background-1);
            }

            .updater {
                display: flex;
                align-items: flex-start;
                gap: 10px;
                align-self: stretch;
            }
        }
    }
</style>
