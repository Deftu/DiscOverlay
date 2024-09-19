import { invoke } from "@tauri-apps/api/core";

export async function openOverlay(id: string) {
    return invoke("open_overlay", { id });
}

export async function closeOverlay(id: string) {
    return invoke("close_overlay", { id });
}

export async function toggleOverlay(id: string) {
    return invoke("toggle_overlay", { id });
}

export async function focusOverlay(id: string) {
    return invoke("focus_overlay", { id });
}

export async function toggleOverlayDevtools(id: string) {
    return invoke("toggle_overlay_devtools", { id });
}

export async function openCustomCssPath() {
    return invoke("open_custom_css_path");
}
