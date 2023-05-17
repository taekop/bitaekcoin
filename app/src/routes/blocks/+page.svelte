<script>
    let blocks = [];

    import { onMount } from "svelte";

    onMount(async () => {
        fetch("http://0.0.0.0:8000", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: "2.0",
                method: "getBlocks",
            }),
        })
            .then((response) => response.json())
            .then((data) => {
                blocks = data.result;
            })
            .catch((error) => {
                console.log(error);
            });
    });
</script>

<div class="block-container">
    {#each blocks as block}
        <div class="card">
            <div class="block-card-header">Block #{block.header.height}</div>
            <div class="block-card-body">
                {block.transactions.length} transactions
            </div>
            <div class="block-card-footer">
                {new Date(block.header.timestamp * 1000)}
            </div>
        </div>
    {/each}
</div>

<style>
    .block-container {
        width: 100%;
    }

    .card {
        box-shadow: rgba(0, 0, 0, 0.2) 0px 3px 1px -2px,
            rgba(0, 0, 0, 0.14) 0px 2px 2px 0px,
            rgba(0, 0, 0, 0.12) 0px 1px 5px 0px;
        padding: 16px;
        padding-inline: 16px;
        margin: 16px;
    }

    .block-card-header {
        font-family: "Roboto", "Helvetica Neue", sans-serif;
        font-size: 25px;
        font-weight: 500;
        margin: 10px;
    }

    .block-card-body {
        font-size: 20px;
        margin: 10px;
    }

    .block-card-footer {
        color: rgb(115, 115, 115);
        margin: 10px;
    }
</style>
