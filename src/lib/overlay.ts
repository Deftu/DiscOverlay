import { invoke } from "@tauri-apps/api/tauri";

export async function openOverlay() {
    return invoke("open_overlay");
}

export async function closeOverlay() {
    return invoke("close_overlay");
}

export async function openCustomCssPath() {
    return invoke("open_custom_css_path");
}
