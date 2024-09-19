import { get, writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

const listening = writable(false);

export const settings = writable<Settings | null>(null);

export type Settings = {
    voice: VoiceSettings;
};

export type VoiceSettings = {
    showVoiceChannelName: boolean;
    showSpeakingUsersOnly: boolean;
    showOwnUserFirst: boolean;
    showMutedUsers: boolean;
    showUsernames: boolean;
};

export type MessageSettings = {
    channelId: string;
    showTextChannelName: boolean;
};

export async function loadSettings() {
    if (!get(listening)) {
        await listen<Settings>("settings-updated", async (event) => {
            settings.set(event.payload);
        });
    }

    settings.set((await invoke("load_settings")) as Settings);
}

export async function saveSettings(settings: Settings) {
    return await invoke("save_settings", { settings });
}
