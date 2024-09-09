import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export type Settings = {
    visibility: VisibilitySettings;
};

export type VisibilitySettings = {
    show_voice_channel_name: boolean;
    show_speaking_users_only: boolean;
    show_own_user_first: boolean;
    show_muted_users: boolean;
    show_usernames: boolean;
};

export async function loadSettings() {
    return (await invoke("load_settings")) as Settings;
}

export async function saveSettings(settings: Settings) {
    return await invoke("save_settings", { settings });
}

export async function listenForSettings(
    callback: (settings: Settings) => void
): Promise<() => void> {
    return listen("settings-updated", async (settings) => {
        const payload = settings.payload as Settings;
        callback(payload);
    });
}
