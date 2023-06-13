<script>
    import { mempoolStore } from "$lib/stores/mempool.js";
    import IconBase from "svelte-icons/components/IconBase.svelte";

    let mempool;
    mempoolStore.subscribe((value) => {
        mempool = value;
    });

    function transferAmount(tx) {
        return tx.outputs.reduce((s, o) => s + o.amount, 0);
    }
</script>

<div class="pending-container">
    {#each mempool.transactions as tx}
        <div class="pending">Transferring {transferAmount(tx)}...</div>
    {/each}
</div>

<style>
    .pending-container {
        position: absolute;
        height: 80%;
        margin: 30px;
        z-index: 100;
        display: flex;
        flex-direction: column-reverse;
    }

    .pending {
        width: 200px;
        height: 60px;
        color: #c2ffe9;
        background-color: #00e692;
        border-radius: 3px;
        font-weight: bold;
        margin: 30px;
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
