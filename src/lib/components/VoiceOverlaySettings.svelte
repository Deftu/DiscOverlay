<script lang="ts">
    import { focusOverlay, toggleOverlay, toggleOverlayDevtools } from "$lib/overlay";
    import { settings, saveSettings } from "$lib/settings";
    import { currentVoiceChannel } from "$lib/voice";

    import Toggle from "./Toggle.svelte";
</script>

<section class="settings-section">
    <div class="header">
        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32" fill="none">
            <path d="M16 4.60351C16 4.24989 15.8595 3.91075 15.6095 3.6607C15.3594 3.41066 15.0203 3.27018 14.6667 3.27018H14.5867C14.4016 3.26893 14.2183 3.30622 14.0484 3.37967C13.8786 3.45313 13.7258 3.56115 13.6 3.69685L7.89334 9.93685H4.00001C3.64638 9.93685 3.30724 10.0773 3.0572 10.3274C2.80715 10.5774 2.66667 10.9166 2.66667 11.2702V21.9368C2.66667 22.2905 2.80715 22.6296 3.0572 22.8797C3.30724 23.1297 3.64638 23.2702 4.00001 23.2702H7.89334L13.6 29.5102C13.7258 29.6459 13.8786 29.7539 14.0484 29.8274C14.2183 29.9008 14.4016 29.9381 14.5867 29.9368H14.6667C15.0203 29.9368 15.3594 29.7964 15.6095 29.5463C15.8595 29.2963 16 28.9571 16 28.6035V4.60351ZM20.1333 28.2702C19.36 28.4568 18.6667 27.8302 18.6667 27.0435V27.0035C18.6667 26.3368 19.16 25.7768 19.8 25.6035C21.7716 25.062 23.5108 23.888 24.7503 22.2621C25.9899 20.6361 26.6613 18.6481 26.6613 16.6035C26.6613 14.5589 25.9899 12.5709 24.7503 10.945C23.5108 9.31899 21.7716 8.14505 19.8 7.60351C19.4834 7.52735 19.2007 7.34888 18.9958 7.09578C18.7909 6.84267 18.6752 6.52905 18.6667 6.20351V6.16351C18.6667 5.36351 19.36 4.75018 20.1333 4.93685C22.7526 5.56743 25.0835 7.06109 26.7508 9.17736C28.418 11.2936 29.3246 13.9094 29.3246 16.6035C29.3246 19.2976 28.418 21.9134 26.7508 24.0297C25.0835 26.1459 22.7526 27.6396 20.1333 28.2702Z" fill="#FDFBF9"/>
            <path d="M20.2133 22.6168C19.4533 22.9902 18.6667 22.3502 18.6667 21.5102V21.3235C18.6667 20.7502 19.04 20.2568 19.5067 19.9635C20.0681 19.6009 20.5296 19.1034 20.8493 18.5164C21.1689 17.9295 21.3363 17.2718 21.3363 16.6035C21.3363 15.9352 21.1689 15.2775 20.8493 14.6906C20.5296 14.1036 20.0681 13.6061 19.5067 13.2435C19.04 12.9368 18.6667 12.4435 18.6667 11.8835V11.6968C18.6667 10.8568 19.4533 10.2302 20.2133 10.5902C21.3477 11.1331 22.3053 11.9859 22.9757 13.0499C23.646 14.114 24.0018 15.3459 24.0018 16.6035C24.0018 17.8611 23.646 19.0931 22.9757 20.1571C22.3053 21.2212 21.3477 22.0739 20.2133 22.6168Z" fill="#FDFBF9"/>
        </svg>

        <h1 class="dfg-header-1">Voice overlay {$currentVoiceChannel ? `(${$currentVoiceChannel.channelName})` : ''}</h1>
    </div>

    {#if $settings}
        <ul>
            <li class="settings-item">
                <h1 class="dfg-title-1">Show voice channel name?</h1>
                <Toggle bind:value={$settings.voice.showVoiceChannelName} />
            </li>
            <li class="settings-item">
                <h1 class="dfg-title-1">Show speaking users only?</h1>
                <Toggle bind:value={$settings.voice.showSpeakingUsersOnly} />
            </li>
            <li class="settings-item">
                <h1 class="dfg-title-1">Show muted users?</h1>
                <Toggle bind:value={$settings.voice.showMutedUsers} />
            </li>
            <li class="settings-item">
                <h1 class="dfg-title-1">Show usernames?</h1>
                <Toggle bind:value={$settings.voice.showUsernames} />
            </li>
            <li class="settings-item">
                <h1 class="dfg-title-1">Show me first?</h1>
                <Toggle bind:value={$settings.voice.showOwnUserFirst} />
            </li>
        </ul>

        <button class="dfg-button" onclick={async () => {
            await saveSettings($settings);
        }}>Save settings</button>
    {/if}

    <button class="dfg-button" onclick={async () => {
        await toggleOverlay("voice");
    }}>Toggle overlay</button>
    <button class="dfg-button" onclick={async () => {
        await focusOverlay("voice");
    }}>Focus overlay</button>
    <button class="dfg-button" onclick={async () => {
        await toggleOverlayDevtools("voice");
    }}>Open overlay devtools</button>
</section>

<style lang="scss">
    .settings-section {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 20px;
        align-self: stretch;

        .header {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        ul {
            display: flex;
            padding: 10px;
            flex-direction: column;
            justify-content: flex-end;
            align-items: flex-start;
            gap: 20px;
            align-self: stretch;

            .settings-item {
                display: flex;
                justify-content: space-between;
                align-items: center;
                align-self: stretch;
            }
        }
    }
</style>
