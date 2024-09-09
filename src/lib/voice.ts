import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export type VoiceChannelUser = {
    id: string;
    name: string;
    mute: boolean;
    avatar: string | null;
};

export type VoiceChannelData = {
    id: string;
    channel_name: string;
    users: VoiceChannelUser[];
};

export type VoiceUserState = {
    id: string;
    server_mute: boolean;
    server_deaf: boolean;
    self_mute: boolean;
    self_deaf: boolean;
};

export async function requestVoiceChannel(): Promise<VoiceChannelData> {
    await invoke("request_voice_channel");

    return new Promise((resolve) => {
        listen("voice-channel", (data) => {
            const payload = data.payload as VoiceChannelData;
            resolve(payload);
        });
    });
}

export async function listenForVoiceChannel(
    callback: (data: VoiceChannelData) => void
): Promise<() => void> {
    return listen("voice-channel", async (data) => {
        const payload = data.payload as VoiceChannelData;
        callback(payload);
    });
}

export async function subscribeSpeakingState(
    channelId: string
): Promise<() => void> {
    console.log("Subscribing to speaking state for channel", channelId);
    await invoke("subscribe_speaking_state", { channelId });

    return () => {
        invoke("unsubscribe_speaking_state", { channelId });
    };
}

export async function listenForSpeakingStart(
    callback: (userId: string) => void
): Promise<() => void> {
    return listen("speaking-start", async (data) => {
        const payload = data.payload as string;
        callback(payload);
    });
}

export async function listenForSpeakingStop(
    callback: (userId: string) => void
): Promise<() => void> {
    return listen("speaking-stop", async (data) => {
        const payload = data.payload as string;
        callback(payload);
    });
}

export async function listenForVoiceStateUpdate(
    callback: (data: VoiceUserState) => void
): Promise<() => void> {
    return listen("voice-state", async (data) => {
        const payload = data.payload as VoiceUserState;
        callback(payload);
    });
}
