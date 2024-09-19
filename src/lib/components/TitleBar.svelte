<script lang="ts">
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { invoke } from "@tauri-apps/api/core";
const appWindow = getCurrentWebviewWindow()

    async function minimize() {
        await appWindow.minimize();
    }

    async function close() {
        await invoke("close_completely");
    }
</script>

<nav class="dfg-titlebar" data-tauri-drag-region>
    <h2 class="dfg-header-2">DiscOverlay</h2>
    <div class="dfg-titlebar-buttons">
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

        .dfg-titlebar-buttons {
            display: flex;
            padding: 10px;
            justify-content: center;
            align-items: center;
            gap: 20px;

            .dfg-button {
                background: none;
                border: none;
                cursor: pointer;
                min-width: 35px;
                padding: 5px;
                border-radius: 5px;
                transition: background 0.2s;

                &:hover {
                    background: var(--dfg-background-3, #2c2c2e);
                }
            }
        }
    }
</style>
