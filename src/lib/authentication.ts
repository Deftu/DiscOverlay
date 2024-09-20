import { get, writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const listeningForAuth = writable(false);

export const setup = writable(false);
export const authenticated = writable(false);
export const authUrl = writable<string | null>(null);
export const failedToOpenBrowser = writable(false);

export function runSetup() {
    if (get(setup)) {
        return;
    }

    invoke("setup_initial").then(() => {
        setup.set(true);
    });
}

export async function startAuth(clientId: string, clientSecret: string) {
    if (!get(listeningForAuth)) {
        listeningForAuth.set(true);

        await listen("authenticated", () => {
            authenticated.set(true);
        });

        await listen<string>("auth-url", (data) => {
            authUrl.set(data.payload);
        });

        await listen("failed-open-browser", () => {
            failedToOpenBrowser.set(true);
        });
    }

    await invoke("start_discord_auth", { clientId, clientSecret });
}
