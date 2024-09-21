import type { Writable } from "svelte/store";
import { check, Update } from "@tauri-apps/plugin-updater";

export async function checkForUpdates(): Promise<Update | null> {
    console.log("Checking for updates...");
    const update = await check({
        timeout: 30 * 1000,
    });

    if (!update) {
        console.log("No updates found.");
        return null;
    }

    console.log(`Update found: ${update.currentVersion} to ${update.version}`);
    return update;
}

export async function performUpdate(
    update: Update,
    progress: Writable<number>
) {
    let total = 0;
    let downloaded = 0;

    await update.download((download) => {
        switch (download.event) {
            case "Started":
                if (download.data.contentLength !== undefined) {
                    total = download.data.contentLength;
                    console.log(`Downloading ${total} bytes...`);
                }

                break;
            case "Progress":
                downloaded += download.data.chunkLength;
                progress.set(Math.round((downloaded / total) * 100));
                console.log(
                    `Downloading update... ${downloaded} bytes of ${total}`
                );

                break;
            case "Finished":
                console.log("Update downloaded!");

                break;
        }
    });
}
