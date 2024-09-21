<script lang="ts">
    let {
        children,
        clientId = "",
        clientSecret = ""
    }: {
        children: Snippet,
        clientId: string,
        clientSecret: string
    } = $props();

    import { setup, authenticated, authUrl, failedToOpenBrowser, runSetup, startAuth } from "$lib/authentication";
    import { onMount, type Snippet } from "svelte";

    onMount(async () => {
        runSetup();
    });
</script>

{#if $setup}
    {#if $authenticated}
        {@render children()}
    {:else}
        <div class="warning-note">
            <h2 class="dfg-header-2">Looks like this is your first time using DiscOverlay! You’re going to need to set up a Discord developer application and provide the app with it's client ID.</h2>
        </div>

        <div class="tutorial">
            <div class="tutorial-item">
                <h3 class="dfg-header-3">Navigate to the <a href="https://discord.com/developers" target="_blank" class="dfg-header-3">Discord developer portal</a> create a new developer application</h3>
                <img src="new-application.png" alt="New Application" />
            </div>

            <div class="tutorial-item">
                <h3 class="dfg-header-3">Create an application. This can be named whatever you like and can be within whatever team you would like. If you don’t know what the teams mean, just leave it on “Personal”</h3>
                <img src="create-application.png" alt="Create Application" />
            </div>

            <div class="tutorial-item">
                <h3 class="dfg-header-3">Once you’re taken to your new developer app’s page, navigate to the “OAuth2” tab</h3>
                <img src="navigate-oauth2.png" alt="Navigate to OAuth2 Tab" />
            </div>

            <div class="tutorial-item">
                <h3 class="dfg-header-3">Copy your Client ID and (reset and) Client Secret, then input both into the fields below</h3>
                <img src="copy-client-id-and-secret.png" alt="Copy Client ID and Secret" />
            </div>

            <div class="tutorial-item">
                <h3 class="dfg-header-3">Finally, click “Add Redirect” and input <p class="dfg-header-3 tutorial-highlighted">http://localhost:3053/redirect</p> into the redirect input, then save your changes</h3>
                <img src="add-oauth2-redirect.png" alt="Add Redirect" />
            </div>
        </div>

        <form onsubmit={(e) => {
            e.preventDefault();
            startAuth(clientId, clientSecret);
        }}>
            <label class="dfg-body-1">
                Client ID
                <input type="text" id="clientId" bind:value={clientId} />
            </label>

            <label class="dfg-body-1" for="clientSecret">
                Client Secret
                <input type="password" id="clientSecret" bind:value={clientSecret} />
            </label>

            <button type="submit" class="dfg-button">Authenticate</button>
        </form>

        {#if $authUrl !== null}
            <div class="warning-note">
                <p class="dfg-body-1">If the browser didn't open, copy the link below and paste it into your browser:</p>
                <p class="dfg-body-1">{$authUrl}</p>
            </div>
        {/if}

        {#if $failedToOpenBrowser}
            <div class="warning-note">
                <p class="dfg-body-1">Failed to open browser. Please copy the link above and paste it into your browser:</p>
            </div>
        {/if}
    {/if}
{:else}
    <div class="warning-note">
        <h2 class="dfg-header-2">Loading...</h2>
    </div>
{/if}

<style lang="scss">
    .warning-note {
        border-radius: 5px;
        background: var(--dfg-warning);

        display: flex;
        gap: 10px;
        flex-direction: column;
        padding: 10px 0px;
        justify-content: center;
        align-items: center;
        align-self: stretch;

        h2 {
            text-align: center;

            flex: 1 0 0;
            align-self: stretch;
        }

        p {
            max-width: 100%;
            max-width: 100%;
            overflow-wrap: anywhere;
            text-align: center;

            flex: 1 0 0;
            align-self: stretch;
        }
    }

    .tutorial {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 35px;
        align-self: stretch;

        .tutorial-item {
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            gap: 10px;
            align-self: stretch;

            h3 {
                align-self: stretch;

                a {
                    color: var(--dfg-primary-variant);
                }

                p {
                    display: inline;
                }
            }

            img {
                border-radius: 5px;
                align-self: stretch;
            }

            .tutorial-highlighted {
                color: var(--dfg-success);
            }
        }
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 20px;
        align-self: stretch;

        label {
            color: var(--dfg-text-faded);

            display: flex;
            flex-direction: column;
            justify-content: flex-end;
            align-items: flex-start;
            gap: 5px;
            align-self: stretch;
        }

        input {
            color: var(--dfg-text);
            border-radius: 5px;
            border: none;
            outline: none;
            background: var(--dfg-background-2);

            display: flex;
            padding: 10px 25px;
            align-items: center;
            align-self: stretch;
        }
    }
</style>
