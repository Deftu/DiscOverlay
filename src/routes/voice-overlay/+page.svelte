<script lang="ts">
    import "../../styles.css";

    import { listenForCustomCssUpdates, obtainOwnId } from "$lib/utils";
    import { settings, loadSettings } from "$lib/settings";
    import { type VoiceChannelUser, currentVoiceChannel, speakingStates, voiceStates, requestVoiceChannel, listenForDisconnect, subscribeSpeakingState, listenForSpeakingState, listenForVoiceState } from "$lib/voice";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    const appWindow = getCurrentWebviewWindow()

    let ownId: string | null = $state(null);
    let oldChannelId: string | null = null;

    let users: Array<VoiceChannelUser> = $state([]);

    const renderedUsers = $derived.by(() => {
        return users
            // Filter out users based on settings
            .filter((user) => {
                if ($settings?.voice?.showSpeakingUsersOnly === true) {
                    return $speakingStates[user.id];
                }

                if ($settings?.voice?.showMutedUsers === false) {
                    return !$voiceStates[user.id]?.serverMute && !$voiceStates[user.id]?.selfMute;
                }

                return true;
            })
            // Sort users with ourselves first if enabled
            .sort((a, b) => {
                if ($settings?.voice?.showOwnUserFirst === true) {
                    if (a.id === ownId) {
                        return -1;
                    }

                    if (b.id === ownId) {
                        return 1;
                    }
                }

                return 0;
            });
    });

    let customCssListenerCallback = () => {};
    let settingsListenerCallback = () => {};
    let voiceListenerCallback = () => {};
    let disconnectListenerCallback = () => {};
    let speakingStateListenerCallback = () => {};

    onMount(async () => {
        // await appWindow.setIgnoreCursorEvents(true);

        loadSettings();

        obtainOwnId().then((id) => {
            ownId = id;
        });

        await setupListeners();
    });

    onDestroy(() => {
        // appWindow.setIgnoreCursorEvents(false);

        customCssListenerCallback();
        settingsListenerCallback();
        voiceListenerCallback();
        disconnectListenerCallback();
        speakingStateListenerCallback();
    });

    async function setupListeners() {
        customCssListenerCallback = await listenForCustomCssUpdates(() => {
            const cssLinks = document.querySelectorAll("link[rel='stylesheet'][id='dynamic-css']");
            cssLinks.forEach((element) => {
                const filename = element.getAttribute("data-filename");
                if (filename && element instanceof HTMLLinkElement) {
                    element.href = convertFileSrc(filename, "custom-css") + `?update=${Date.now()}`;
                }
            });
        });

        voiceListenerCallback = currentVoiceChannel.subscribe(async (channel) => {
            if (channel === null) {
                return;
            }

            if (oldChannelId !== channel.id) {
                speakingStateListenerCallback();
            }

            oldChannelId = channel.id;
            users = channel.users;
            speakingStateListenerCallback = await subscribeSpeakingState(channel.id);
        });

        disconnectListenerCallback = await listenForDisconnect(async () => {
            await appWindow.close();
        });

        await requestVoiceChannel(); // Set up listener so that the globally stored voice channel is updated
        await listenForSpeakingState();
        await listenForVoiceState();
    }

    function getAvatarUrl(user: VoiceChannelUser) {
        const avatar = user.avatar;
        return avatar
            ? `https://cdn.discordapp.com/avatars/${user.id}/${avatar}.png`
            : `https://cdn.discordapp.com/embed/avatars/${parseInt(user.id, 10) % 5}.png`;
    }
</script>

<svelte:head>
    <link id="dynamic-css" rel="stylesheet" href={convertFileSrc("global.css", "custom-css")} data-filename="global.css" />
    <link id="dynamic-css" rel="stylesheet" href={convertFileSrc("voice.css", "custom-css")} data-filename="voice.css" />
</svelte:head>

{#if $currentVoiceChannel !== null}
    <div class="overlay-container">
        {#if $settings?.voice?.showVoiceChannelName ?? true}
            <div class="channel-meta">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none">
                    <path d="M12 3.60352C12 3.3383 11.8946 3.08395 11.7071 2.89641C11.5196 2.70887 11.2652 2.60352 11 2.60352H10.94C10.8012 2.60258 10.6637 2.63054 10.5363 2.68564C10.4089 2.74073 10.2944 2.82174 10.2 2.92352L5.92 7.60352H3C2.73478 7.60352 2.48043 7.70887 2.29289 7.89641C2.10536 8.08395 2 8.3383 2 8.60352V16.6035C2 16.8687 2.10536 17.1231 2.29289 17.3106C2.48043 17.4982 2.73478 17.6035 3 17.6035H5.92L10.2 22.2835C10.2944 22.3853 10.4089 22.4663 10.5363 22.5214C10.6637 22.5765 10.8012 22.6045 10.94 22.6035H11C11.2652 22.6035 11.5196 22.4982 11.7071 22.3106C11.8946 22.1231 12 21.8687 12 21.6035V3.60352ZM15.1 21.3535C14.52 21.4935 14 21.0235 14 20.4335V20.4035C14 19.9035 14.37 19.4835 14.85 19.3535C16.3287 18.9474 17.6331 18.0669 18.5627 16.8474C19.4924 15.628 19.996 14.1369 19.996 12.6035C19.996 11.0701 19.4924 9.57907 18.5627 8.3596C17.6331 7.14013 16.3287 6.25967 14.85 5.85352C14.6125 5.79639 14.4005 5.66254 14.2469 5.47271C14.0932 5.28288 14.0064 5.04767 14 4.80352V4.77352C14 4.17352 14.52 3.71352 15.1 3.85352C17.0645 4.32646 18.8126 5.4467 20.0631 7.0339C21.3135 8.62111 21.9935 10.5829 21.9935 12.6035C21.9935 14.6241 21.3135 16.5859 20.0631 18.1731C18.8126 19.7603 17.0645 20.8806 15.1 21.3535Z" fill="currentColor"/>
                    <path d="M15.16 17.1135C14.59 17.3935 14 16.9135 14 16.2835V16.1435C14 15.7135 14.28 15.3435 14.63 15.1235C15.051 14.8516 15.3972 14.4784 15.6369 14.0382C15.8766 13.598 16.0022 13.1048 16.0022 12.6035C16.0022 12.1023 15.8766 11.609 15.6369 11.1688C15.3972 10.7286 15.051 10.3555 14.63 10.0835C14.28 9.85352 14 9.48352 14 9.06352V8.92352C14 8.29352 14.59 7.82352 15.16 8.09352C16.0108 8.50073 16.729 9.14028 17.2318 9.93832C17.7345 10.7363 18.0013 11.6603 18.0013 12.6035C18.0013 13.5467 17.7345 14.4707 17.2318 15.2687C16.729 16.0668 16.0108 16.7063 15.16 17.1135Z" fill="currentColor"/>
                </svg>

                <h1 class="dfg-header-2">{$currentVoiceChannel.channelName}</h1>
            </div>
        {/if}

        <div class="users">
            {#each renderedUsers as user (user.id)}
                <div class="user" class:speaking={$speakingStates[user.id]}>
                    <div class="avatar-container" class:disabled={$voiceStates[user.id]?.serverMute || $voiceStates[user.id]?.serverDeaf || $voiceStates[user.id]?.selfMute || $voiceStates[user.id]?.selfDeaf}>
                        <img
                            class="user-avatar"
                            src={getAvatarUrl(user)}
                            alt={user.name}
                        />
                    </div>

                    <div class="user-meta">
                        {#if $settings?.voice?.showUsernames ?? true}
                            <h1 class="dfg-title-1">
                                {user.name}
                            </h1>
                        {/if}

                        {#if $voiceStates[user.id]?.serverMute || $voiceStates[user.id]?.selfMute}
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" class:selfmuted={$voiceStates[user.id]?.selfMute} class:servermuted={$voiceStates[user.id]?.serverMute}>
                                <path d="M2.70002 23.3035L22.7 3.30352C22.8445 3.11094 22.9146 2.87274 22.8975 2.63263C22.8804 2.39252 22.7773 2.16662 22.6071 1.99641C22.4369 1.8262 22.211 1.7231 21.9709 1.70603C21.7308 1.68897 21.4926 1.75909 21.3 1.90352L1.30002 21.9035C1.18541 21.9895 1.0906 22.0991 1.02202 22.2249C0.953443 22.3507 0.912697 22.4897 0.90254 22.6326C0.892384 22.7755 0.913055 22.919 0.963154 23.0532C1.01325 23.1874 1.09161 23.3093 1.19292 23.4106C1.29423 23.5119 1.41612 23.5903 1.55035 23.6404C1.68458 23.6905 1.828 23.7112 1.97091 23.701C2.11383 23.6908 2.25289 23.6501 2.37868 23.5815C2.50447 23.5129 2.61406 23.4181 2.70002 23.3035ZM10.8 17.9235C10.59 18.1335 10.7 18.5035 11 18.5435V20.6035H9.00002C8.73481 20.6035 8.48045 20.7089 8.29292 20.8964C8.10538 21.0839 8.00002 21.3383 8.00002 21.6035C8.00002 21.8687 8.10538 22.1231 8.29292 22.3106C8.48045 22.4982 8.73481 22.6035 9.00002 22.6035H15C15.2652 22.6035 15.5196 22.4982 15.7071 22.3106C15.8947 22.1231 16 21.8687 16 21.6035C16 21.3383 15.8947 21.0839 15.7071 20.8964C15.5196 20.7089 15.2652 20.6035 15 20.6035H13V18.5435C14.9338 18.2999 16.712 17.3587 18.0009 15.8967C19.2898 14.4348 20.0007 12.5525 20 10.6035C20 10.3383 19.8947 10.0839 19.7071 9.89641C19.5196 9.70887 19.2652 9.60352 19 9.60352C18.7348 9.60352 18.4805 9.70887 18.2929 9.89641C18.1054 10.0839 18 10.3383 18 10.6035C18 12.0535 17.48 13.3935 16.62 14.4335L16.6 14.4535C16.0729 15.087 15.4207 15.605 14.6843 15.9749C13.9478 16.3449 13.143 16.5589 12.32 16.6035C12.1921 16.6099 12.071 16.6633 11.98 16.7535L10.8 17.9335V17.9235ZM15.36 5.12352C15.51 4.97352 15.55 4.74352 15.44 4.56352C14.9928 3.80985 14.3105 3.22391 13.4979 2.89574C12.6853 2.56757 11.7874 2.51534 10.9423 2.74708C10.0971 2.97881 9.35147 3.48169 8.8199 4.17842C8.28833 4.87515 8.00028 5.72716 8.00002 6.60352V10.6035C8.00002 10.9035 8.03002 11.1835 8.10002 11.4635C8.17002 11.8035 8.59002 11.8935 8.84002 11.6435L15.36 5.12352ZM5.06002 14.5835C5.22002 14.8635 5.59002 14.8935 5.81002 14.6735L6.56002 13.9235C6.72002 13.7635 6.75002 13.5235 6.64002 13.3135C6.21606 12.4733 5.99676 11.5447 6.00002 10.6035C6.00002 10.3383 5.89467 10.0839 5.70713 9.89641C5.51959 9.70887 5.26524 9.60352 5.00002 9.60352C4.73481 9.60352 4.48045 9.70887 4.29292 9.89641C4.10538 10.0839 4.00002 10.3383 4.00002 10.6035C4.00002 12.0535 4.39002 13.4135 5.06002 14.5835Z" fill="currentColor" fill-opacity="0.65"/>
                            </svg>
                        {/if}

                        {#if $voiceStates[user.id]?.serverDeaf || $voiceStates[user.id]?.selfDeaf}
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" class:selfdeaf={$voiceStates[user.id]?.selfDeaf} class:serverdeaf={$voiceStates[user.id]?.serverDeaf}>
                                <path d="M22.7 3.30351C22.8445 3.11094 22.9146 2.87273 22.8975 2.63262C22.8804 2.39251 22.7773 2.16661 22.6071 1.9964C22.4369 1.82619 22.211 1.72309 21.9709 1.70603C21.7308 1.68896 21.4926 1.75908 21.3 1.90351L1.30002 21.9035C1.18541 21.9895 1.0906 22.0991 1.02202 22.2249C0.953443 22.3506 0.912697 22.4897 0.90254 22.6326C0.892384 22.7755 0.913055 22.919 0.963154 23.0532C1.01325 23.1874 1.09161 23.3093 1.19292 23.4106C1.29423 23.5119 1.41612 23.5903 1.55035 23.6404C1.68458 23.6905 1.828 23.7112 1.97091 23.701C2.11383 23.6908 2.25289 23.6501 2.37868 23.5815C2.50447 23.5129 2.61406 23.4181 2.70002 23.3035L22.7 3.30351ZM17.06 3.54351C17.1146 3.49076 17.1559 3.42582 17.1806 3.35401C17.2052 3.2822 17.2124 3.20557 17.2017 3.13042C17.191 3.05526 17.1625 2.98373 17.1188 2.92168C17.0751 2.85964 17.0172 2.80886 16.95 2.77351C14.8835 1.73393 12.5418 1.37232 10.2579 1.74008C7.97407 2.10785 5.86427 3.18627 4.22852 4.82201C2.59278 6.45775 1.51436 8.56755 1.1466 10.8514C0.778833 13.1353 1.14045 15.477 2.18002 17.5435C2.32002 17.8435 2.71002 17.8935 2.94002 17.6635L6.14002 14.4635C6.39002 14.2135 6.29002 13.7835 5.94002 13.7035C5.60435 13.6353 5.26255 13.6018 4.92002 13.6035H3.05002C2.8728 12.0231 3.11749 10.424 3.75918 8.96892C4.40088 7.51385 5.41667 6.25478 6.70315 5.3199C7.98963 4.38502 9.50085 3.80771 11.083 3.64674C12.6651 3.48577 14.2616 3.74689 15.71 4.40351C15.91 4.49351 16.15 4.45351 16.3 4.30351L17.06 3.54351ZM20.2 8.88351C20.1562 8.78805 20.1422 8.68154 20.1601 8.578C20.1779 8.47447 20.2267 8.37877 20.3 8.30351L21.06 7.54351C21.1128 7.48891 21.1777 7.4476 21.2495 7.42298C21.3213 7.39835 21.398 7.39111 21.4731 7.40184C21.5483 7.41258 21.6198 7.44099 21.6819 7.48474C21.7439 7.52849 21.7947 7.58633 21.83 7.65351C23.1081 10.1929 23.3534 13.1285 22.5144 15.8448C21.6755 18.5611 19.8175 20.8471 17.33 22.2235C16.06 22.9335 14.6 22.4535 13.78 21.4835C13.3384 20.9606 13.0822 20.3062 13.0514 19.6225C13.0207 18.9387 13.2171 18.2639 13.61 17.7035L14.99 15.7335C15.4517 15.0749 16.0655 14.5373 16.7792 14.1665C17.493 13.7957 18.2857 13.6026 19.09 13.6035H20.95C21.125 11.9915 20.8661 10.3619 20.2 8.88351ZM10.1 18.5035C10.35 18.2535 10.75 18.3235 10.84 18.6435C10.9767 19.1315 10.9916 19.6456 10.8835 20.1407C10.7755 20.6358 10.5476 21.0969 10.22 21.4835C9.80619 21.9998 9.22562 22.3559 8.5779 22.4909C7.93018 22.626 7.25566 22.5314 6.67002 22.2235C6.64856 22.2118 6.63007 22.1953 6.61597 22.1754C6.60186 22.1554 6.59252 22.1325 6.58866 22.1083C6.58479 22.0842 6.58652 22.0595 6.59369 22.0361C6.60086 22.0127 6.61329 21.9913 6.63002 21.9735L10.11 18.4935L10.1 18.5035Z" fill="currentColor" fill-opacity="0.65"/>
                            </svg>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
{/if}

<style lang="scss">
    :global(html, body, .svelte-body, main) {
        background: transparent !important;
    }

    .overlay-container {
        width: fit-content;
        display: flex;
        padding: 15px;
        margin: 10px;
        flex-direction: column;
        align-items: flex-start;
        gap: 20px;

        border-radius: 5px;
        background: var(--dfg-background-2);
        color: var(--dfg-text);

        .channel-meta {
            display: flex;
            padding: 0px 10px;
            align-items: center;
            gap: 10px;

            svg {
                width: 24px;
                height: 24px;

                fill: var(--dfg-text);
            }
        }

        .users {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: flex-start;
            gap: 10px;
            align-self: stretch;

            .user {
                display: flex;
                justify-content: center;
                align-items: center;
                gap: 10px;

                .avatar-container,
                .avatar-container .user-avatar {
                    width: 52px;
                    height: 52px;
                    border-radius: 50px;
                }

                .disabled {
                    outline: 2.5px solid var(--dfg-danger);
                    outline-offset: 1px;

                    .user-avatar {
                        filter: grayscale(100%);
                    }
                }

                .user-meta {
                    display: flex;
                    align-items: center;
                    gap: 5px;

                    .selfmuted,
                    .selfdeaf {
                        color: var(--dfg-text-faded);
                    }

                    .servermuted,
                    .serverdeaf {
                        color: var(--dfg-danger);
                    }
                }
            }

            .speaking {
                .avatar-container {
                    outline: 2.5px solid var(--dfg-success);
                    outline-offset: 1px;
                }
            }
        }
    }
</style>
