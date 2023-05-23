<script>
    import { blockStore } from "$lib/stores/block.js";
    import Pagination from "$lib/components/Pagination.svelte";

    let blocks;
    blockStore.subscribe((value) => {
        blocks = value;
    });

    const perPage = 10;
    let currentPage;
    $: visibleBlocks = blocks.filter(
        (_, i) => (currentPage - 1) * perPage <= i && i < currentPage * perPage
    );
</script>

<div class="blocks-body">
    <div class="pagination-container">
        <Pagination totalRows={blocks.length} {perPage} bind:currentPage />
    </div>
    {#each visibleBlocks as block}
        <div class="block-card">
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
    .blocks-body {
        width: 100%;
        padding: 32px;
        box-sizing: border-box;
    }

    .block-card {
        box-shadow: rgba(0, 0, 0, 0.2) 0px 3px 1px -2px,
            rgba(0, 0, 0, 0.14) 0px 2px 2px 0px,
            rgba(0, 0, 0, 0.12) 0px 1px 5px 0px;
        padding: 16px;
        padding-inline: 16px;
        margin-top: 16px;
        margin-bottom: 16px;
    }

    .block-card:hover {
        background-color: #eee;
    }

    .block-card-header {
        font-family: "Roboto", "Helvetica Neue", sans-serif;
        font-size: 25px;
        font-weight: 500;
        margin: 10px;
    }

    .block-card-body {
        color: rgb(115, 115, 115);
        margin: 10px;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        font-size: 20px;
    }

    .block-card-footer {
        color: rgb(115, 115, 115);
        margin: 10px;
    }
</style>
