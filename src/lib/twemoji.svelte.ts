import api from "@discordapp/twemoji";

export function twemoji(node: HTMLElement) {
    api.parse(node);

    return {
        update(node: HTMLElement) {
            api.parse(node);
        },
    };
}
