<script lang="ts">
    import TitleBar from "$lib/components/TitleBar.svelte";
    import Footer from "$lib/components/Footer.svelte";

    import { type Settings, loadSettings, saveSettings } from "$lib/settings";
    import { loadDiscordConfig } from "$lib/discord_settings";
    import { runSetup, startAuth, listenForAuth, listenForAuthUrl, listenForFailedOpenBrowser } from "$lib/authentication";
    import { type VoiceChannelData, requestVoiceChannel } from "$lib/voice";
    import { openOverlay, closeOverlay, openCustomCssPath } from "$lib/overlay";

    import { onMount, onDestroy } from "svelte";
    import Toggle from "$lib/components/Toggle.svelte";

    let authenticated = $state(false);
    let clientId = $state("");
    let clientSecret = $state("");

    let settings: Settings | null = $state(null);

    let updateInterval: number | null = null;
    let authUrl: string | null = $state(null);
    let failedToOpenBrowser = $state(false);
    let voiceChannel: VoiceChannelData | null = $state(null);

    let authListenerCallback: () => void = () => {};
    let authUrlCallback: () => void = () => {};
    let failedOpenBrowserCallback: () => void = () => {};

    onMount(async () => {
        await updateSettings();
        await updateDiscordSettings();

        await runSetup();
        authListenerCallback = await listenForAuth(async (authState) => {
            authenticated = authState;
            await checkForVoiceChannel();
        });

        authUrlCallback = await listenForAuthUrl(async (url) => {
            authUrl = url;
        });

        failedOpenBrowserCallback = await listenForFailedOpenBrowser(async () => {
            failedToOpenBrowser = true;
        });

        if (authenticated) {
            await checkForVoiceChannel();
        }
    });

    onDestroy(async () => {
        authListenerCallback();
        authUrlCallback();
        failedOpenBrowserCallback();

        console.log("Destroying overlay page...");
        if (updateInterval !== null) {
            clearInterval(updateInterval);
            await toggleOverlay();
        }
    });

    async function updateSettings() {
        let loadedSettings = await loadSettings();
        if (loadedSettings !== null) {
            settings = loadedSettings;
        }
    }

    async function handleSave() {
        if (settings !== null) {
            await saveSettings(settings);
        }
    }

    async function updateDiscordSettings() {
        let settings = await loadDiscordConfig();
        if (settings !== null) {
            clientId = settings.client_id;
            clientSecret = settings.client_secret;
        }
    }

    async function checkForVoiceChannel() {
        voiceChannel = await requestVoiceChannel();
        await toggleOverlay();

        if (updateInterval !== null) {
            clearInterval(updateInterval);
        }

        updateInterval = setInterval(async () => {
            console.log("Checking for voice channel...");
            voiceChannel = await requestVoiceChannel();
            await toggleOverlay();
        }, 2500);
    }

    async function toggleOverlay() {
        if (voiceChannel !== null) {
            await openOverlay();
        } else {
            await closeOverlay();
        }
    }

    async function reloadOverlay() {
        if (voiceChannel === null) {
            return;
        }

        await closeOverlay();
        await openOverlay();
    }
</script>

<TitleBar />

<main class="dfg-content">
    {#if authenticated}
        {#if voiceChannel}
            <div class="dfg-main">
                <section class="dfg-voice-channel">
                    <h2 class="dfg-header-2">Current voice channel:</h2>
                    <h3 class="dfg-header-3">{voiceChannel.channel_name}</h3>
                </section>

                {#if settings}
                    <section class="dfg-settings-section">
                        <div class="dfg-settings">
                            <h1 class="dfg-header-1 dfg-settings-header">Visibility settings</h1>
                            <div class="dfg-setting">
                                <h1 class="dfg-header-1">Show voice channel name?</h1>
                                <Toggle bind:value={settings!!.visibility.show_voice_channel_name} />
                            </div>
                            <div class="dfg-setting">
                                <h1 class="dfg-header-1">Show speaking users only?</h1>
                                <Toggle bind:value={settings!!.visibility.show_speaking_users_only} />
                            </div>
                            <div class="dfg-setting">
                                <h1 class="dfg-header-1">Show muted users?</h1>
                                <Toggle bind:value={settings!!.visibility.show_muted_users} />
                            </div>
                            <div class="dfg-setting">
                                <h1 class="dfg-header-1">Show usernames?</h1>
                                <Toggle bind:value={settings!!.visibility.show_usernames} />
                            </div>
                            <div class="dfg-setting">
                                <h1 class="dfg-header-1">Show me first?</h1>
                                <Toggle bind:value={settings!!.visibility.show_own_user_first} />
                            </div>
                        </div>

                        <button class="dfg-button" onclick={handleSave}>Save</button>
                        <div class="dfg-setting-custom-css">
                            <button class="dfg-button" onclick={openCustomCssPath}>Open custom CSS location</button>
                            <button class="dfg-button" onclick={reloadOverlay}>Reload overlay</button>
                        </div>
                    </section>
                {/if}
            </div>
        {:else}
            <h1 class="dfg-attention-1">Not in a voice channel!</h1>
            <button class="dfg-button" onclick={checkForVoiceChannel}>Check again?</button>
        {/if}
    {:else}
        <div class="dfg-warning-note">
            <h2 class="dfg-header-2">Looks like this is your first time using DiscOverlay! You’re going to need to set up a Discord developer application and provide the app with it's client ID.</h2>
        </div>

        <form onsubmit={(e) => {
            e.preventDefault();
            startAuth(clientId, clientSecret);
        }}>
            <label class="dfg-body-1">
                Client ID
                <input type="text" id="clientId" bind:value={clientId} />
            </label>

            <label class="dfg-body-1" for="clientSecret">
                Client Secret
                <input type="password" id="clientSecret" bind:value={clientSecret} />
            </label>

            <button type="submit" class="dfg-button">Authenticate</button>
        </form>

        {#if authUrl !== null}
            <p class="dfg-body-1">If the browser didn't open, copy the link below and paste it into your browser:</p>
            <p class="dfg-body-1">{authUrl}</p>
        {/if}

        {#if failedToOpenBrowser}
            <p class="dfg-body-1">Failed to open browser. Please copy the link above and paste it into your browser:</p>
        {/if}
    {/if}
</main>

<Footer />

<style lang="scss">
    .dfg-button {
        display: inline-flex;
        padding: 10px 20px;
        justify-content: center;
        align-items: center;
        gap: 10px;

        border-radius: 5px;
        background: var(--dfg-background-2);
    }

    .dfg-content {
        height: 100%;
        display: flex;
        padding: 20px 250px;
        flex-direction: column;
        align-items: center;
        gap: 20px;
        align-self: stretch;

        .dfg-warning-note {
            border-radius: 5px;
            background: var(--dfg-warning);

            display: flex;
            padding: 10px 0px;
            justify-content: center;
            align-items: center;
            align-self: stretch;

            h2 {
                text-align: center;

                flex: 1 0 0;
                align-self: stretch;
            }
        }

        form {
            display: flex;
            flex-direction: column;
            gap: 20px;
            align-self: stretch;

            label {
                color: var(--dfg-text-faded);

                display: flex;
                flex-direction: column;
                justify-content: flex-end;
                align-items: flex-start;
                gap: 5px;
                align-self: stretch;
            }

            input {
                color: var(--dfg-text);
                border-radius: 5px;
                border: none;
                outline: none;
                background: var(--dfg-background-2);

                display: flex;
                padding: 10px 25px;
                align-items: center;
                align-self: stretch;
            }

            // button {
                // color: var(--dfg-text);
                // border-radius: 5px;
                // border: none;
                // outline: none;
                // background: var(--dfg-background-2);
                // cursor: pointer;

                // display: flex;
                // padding: 10px 20px;
                // justify-content: center;
                // align-items: center;
                // gap: 10px;
                // align-self: stretch;
            // }
        }
    }

    .dfg-main {
        width: 100%;
        height: 100%;

        .dfg-voice-channel {
            border-radius: 5px;
            background: var(--dfg-background-2);

            display: flex;
            padding: 10px 0px;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            align-self: stretch;

            h3 {
                color: var(--dfg-text-disabled);
            }
        }

        .dfg-settings-section {
            display: flex;
            padding: 10px;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            gap: 20px;
            align-self: stretch;

            .dfg-settings {
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                gap: 10px;
                align-self: stretch;

                .dfg-settings-header {
                    color: var(--dfg-text-faded);
                }

                .dfg-setting {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    align-self: stretch;
                }
            }

            .dfg-setting-custom-css {
                display: flex;
                justify-content: center;
                align-items: center;
                gap: 10px;
                align-self: stretch;
            }
        }
    }

    .dfg-button {
            color: var(--dfg-text);
            border-radius: 5px;
            border: none;
            outline: none;
            background: var(--dfg-background-2);
            cursor: pointer;

            display: flex;
            padding: 10px 20px;
            justify-content: center;
            align-items: center;
            gap: 10px;
            align-self: stretch;
    }
</style>
