<script>
    export let totalRows;
    export let perPage;
    export let currentPage = 1;

    $: totalPages = Math.ceil(totalRows / perPage);
    $: visiblePages = calculateVisiblePages(currentPage, totalPages);

    function calculateVisiblePages(currentPage, totalPages) {
        let pages = [1];
        for (let i = currentPage - 1; i <= currentPage + 1; i++) {
            if (i > 1 && i < totalPages) {
                pages.push(i);
            }
        }
        if (totalPages > 1) {
            pages.push(totalPages);
        }

        if (pages.length > 1 && pages[1] > 2) {
            pages.splice(1, 0, "dots");
        }
        if (
            pages.length > 1 &&
            pages[pages.length - 2] !== "dots" &&
            pages[pages.length - 2] + 1 < pages[pages.length - 1]
        ) {
            pages.splice(pages.length - 1, 0, "dots");
        }
        return pages;
    }
</script>

<div class="pagination">
    <button
        class="btn-nav left-btn"
        disabled={currentPage === 1 ? true : false}
        on:click={() => (currentPage -= 1)}
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="left-icon"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 19.5L8.25 12l7.5-7.5"
            />
        </svg>
    </button>
    <div class="page-numbers">
        {#each visiblePages as page}
            {#if page === "dots"}
                <span class="dots">...</span>
            {:else}
                <button
                    class="btn-page"
                    class:btn-selected={currentPage === page}
                    on:click={() => (currentPage = page)}>{page}</button
                >
            {/if}
        {/each}
    </div>
    <button
        class="btn-nav right-btn"
        disabled={currentPage === totalPages ? true : false}
        on:click={() => (currentPage += 1)}
    >
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="right-icon"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M8.25 4.5l7.5 7.5-7.5 7.5"
            />
        </svg>
    </button>
</div>

<style>
    button {
        font-family: "Inter", sans-serif;
        color: #343a40;
        line-height: 1;
    }

    button:disabled {
        background-color: #087f5b;
        color: white;
    }

    .pagination,
    .page-numbers {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
    }

    .btn-nav,
    .btn-page {
        border-radius: 50%;
        background-color: #fff;
        cursor: pointer;
    }

    .btn-nav {
        padding: 8px;
    }

    .btn-nav {
        width: 42px;
        height: 42px;
        border: 1.5px solid #087f5b;
        color: #087f5b;
    }

    .btn-nav:enabled:hover,
    .btn-page:enabled:hover {
        background-color: #087f5b;
        color: #fff;
    }

    .btn-page {
        border: none;
        width: 40px;
        height: 40px;
        font-size: 16px;
    }

    .btn-selected {
        background-color: #087f5b;
        color: #fff;
    }
</style>
