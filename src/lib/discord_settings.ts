import { invoke } from "@tauri-apps/api/tauri";

export type DiscordConfig = {
    client_id: string;
    client_secret: string;
};

export async function loadDiscordConfig() {
    return (await invoke("load_config")) as DiscordConfig;
}
