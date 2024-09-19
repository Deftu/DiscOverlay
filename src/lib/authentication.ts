import { get, writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const setup = writable(false);
export const authenticated = writable(false);

export async function runSetup() {
    if (get(setup)) {
        return;
    }

    setup.set(true);
    await invoke("setup_initial");
}

export async function startAuth(clientId: string, clientSecret: string) {
    await invoke("start_discord_auth", { clientId, clientSecret });
}

export async function listenForAuth(
    callback: (authState: boolean) => void
): Promise<() => void> {
    return listen("authenticated", () => {
        callback(true);
    });
}

export async function listenForAuthUrl(
    callback: (url: string) => void
): Promise<() => void> {
    return listen("auth-url", (data) => {
        const payload = data.payload as string;
        callback(payload);
    });
}

export async function listenForFailedOpenBrowser(callback: () => void) {
    return listen("failed-open-browser", () => {
        callback();
    });
}
