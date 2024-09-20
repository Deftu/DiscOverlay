<script lang="ts">
    import TitleBar from "$lib/components/TitleBar.svelte";
    import Authenticated from "$lib/components/Authenticated.svelte";
    import VoiceOverlaySettings from "$lib/components/VoiceOverlaySettings.svelte";
    import Footer from "$lib/components/Footer.svelte";

    import { loadSettings } from "$lib/settings";
    import { loadDiscordConfig } from "$lib/discord_settings";
    import { openCustomCssPath } from "$lib/overlay";

    import { onMount } from "svelte";

    let clientId: string = $state("");
    let clientSecret: string = $state("");

    onMount(() => {
        loadSettings();

        loadDiscordConfig().then((config) => {
            if (config !== null) {
                clientId = config.client_id;
                clientSecret = config.client_secret;
            }
        });
    });
</script>

<TitleBar />

<main class="container">
    <Authenticated {clientId} {clientSecret}>
        <div class="main-content">
            <button class="dfg-button" onclick={async () => {
                await openCustomCssPath();
            }}>Open custom CSS folder</button>

            <VoiceOverlaySettings />
        </div>
    </Authenticated>
</main>

<Footer />

<style lang="scss">
    .container {
        height: 100%;
        display: flex;
        padding: 20px 250px;
        flex-direction: column;
        align-items: center;
        gap: 20px;
        align-self: stretch;
    }

    .main-content {
        width: 100%;
        height: 100%;

        display: flex;
        padding: 20px 100px;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 35px;
        align-self: stretch;
    }
</style>
