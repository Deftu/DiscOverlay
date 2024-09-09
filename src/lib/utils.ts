import { invoke } from "@tauri-apps/api/tauri";

export async function obtainOwnId(): Promise<string> {
    return await invoke("obtain_own_id");
}
