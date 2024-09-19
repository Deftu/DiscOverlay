import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export async function listenForCustomCssUpdates(
    callback: () => void
): Promise<() => void> {
    return listen("custom-css-update", () => {
        callback();
    });
}

export async function obtainOwnId(): Promise<string> {
    return await invoke("obtain_own_id");
}
