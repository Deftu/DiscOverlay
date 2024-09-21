<script lang="ts">
    import "../styles.css";
    import "../app.css";

    let { children } = $props();

    // Disable context menu
    window.addEventListener("contextmenu", (e) => {
        e.preventDefault();
    });
</script>

{@render children()}
