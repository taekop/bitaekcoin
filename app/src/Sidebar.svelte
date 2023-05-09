<script>
    import Logo from "./Logo.svelte";
</script>

<div>
    <header>
        <Logo style="padding: 1px;" />
        <div>BitaekCoin</div>
    </header>
    <slot />
</div>

<style>
    div {
        width: 300px;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: start;
    }

    header {
        border-bottom: 1px solid black;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        text-align: center;
    }

    header > div {
        display: flex;
        justify-content: center;
        font-size: xx-large;
        font-style: italic;
    }
</style>
