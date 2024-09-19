import { get, writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const listening = writable(false);
const disconnectSubscribed = writable(false);
const speakingStateListening = writable(false);
const voiceStateListening = writable(false);

export const currentVoiceChannel: Writable<VoiceChannelData | null> =
    writable(null);

export const speakingStates: Writable<{ [id: string]: boolean }> = writable({});

export const voiceStates: Writable<{ [id: string]: VoiceUserState }> = writable(
    {}
);

export type VoiceChannelUser = {
    id: string;
    name: string;
    mute: boolean;
    avatar: string | null;
};

export type VoiceChannelData = {
    id: string;
    channelName: string;
    users: VoiceChannelUser[];
};

export type VoiceUserState = {
    id: string;
    serverMute: boolean;
    serverDeaf: boolean;
    selfMute: boolean;
    selfDeaf: boolean;
};

export async function requestVoiceChannel() {
    await invoke("request_voice_channel");

    if (get(listening)) {
        return;
    }

    // Tell our global store that the listener is active
    listening.set(true);

    // Request voice channel updates every 500ms
    setInterval(async () => {
        await requestVoiceChannel();
    }, 500);

    // Listen for voice channel updates
    await listen<VoiceChannelData>("voice-channel", async (event) => {
        currentVoiceChannel.set(event.payload);
    });
}

export async function listenForDisconnect(
    callback: () => void
): Promise<() => void> {
    if (!get(disconnectSubscribed)) {
        await invoke("subscribe_voice_disconnect");
    }

    return listen("voice-connection-disconnected", () => {
        callback();
    });
}

export async function subscribeSpeakingState(
    channelId: string
): Promise<() => void> {
    await invoke("subscribe_speaking_state", { channelId });

    return () => {
        invoke("unsubscribe_speaking_state", { channelId });
    };
}

export async function listenForSpeakingState() {
    if (!get(speakingStateListening)) {
        speakingStateListening.set(true);

        await listen<string>("speaking-start", async (event) => {
            speakingStates.update((states) => ({
                ...states,
                [event.payload]: true,
            }));
        });

        await listen<string>("speaking-stop", async (event) => {
            speakingStates.update((states) => ({
                ...states,
                [event.payload]: false,
            }));
        });
    }
}

export async function listenForVoiceState() {
    if (!get(voiceStateListening)) {
        voiceStateListening.set(true);

        await listen<VoiceUserState>("voice-state-create", async (event) => {
            voiceStates.update((states) => ({
                ...states,
                [event.payload.id]: event.payload,
            }));
        });

        await listen<VoiceUserState>("voice-state-update", async (event) => {
            voiceStates.update((states) => ({
                ...states,
                [event.payload.id]: event.payload,
            }));
        });

        await listen<string>("voice-state-delete", async (event) => {
            const id = event.payload;

            // Remove the relevant speaking state
            speakingStates.update((states) => {
                const { [id]: _, ...rest } = states;
                return rest;
            });

            // Remove the voice state
            voiceStates.update((states) => {
                const { [id]: _, ...rest } = states;
                return rest;
            });
        });
    }
}
