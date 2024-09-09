import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

let setup = false;

export async function runSetup() {
    if (setup) {
        return;
    }

    setup = true;
    await invoke("setup_initial");
}

export async function startAuth(clientId: string, clientSecret: string) {
    await invoke("start_discord_auth", { clientId, clientSecret });
}

export async function listenForAuth(
    callback: (authState: boolean) => void
): Promise<() => void> {
    return listen("authenticated", () => {
        console.log("Authenticated");
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
