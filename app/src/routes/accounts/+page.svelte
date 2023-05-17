<script>
    let accounts = [
        {
            index: 0,
            balance: 1000,
        },
    ];
</script>

<div class="account-container">
    {#each accounts as account}
        <div class="card">
            <div class="account-card-header">Account #{account.index}</div>
            <div class="account-card-body">
                {account.balance / 1000000000} BTC ({account.balance} SAT)
            </div>
        </div>
    {/each}
</div>

<style>
    .account-container {
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

    .account-card-header {
        font-family: "Roboto", "Helvetica Neue", sans-serif;
        font-size: 25px;
        font-weight: 500;
        margin: 10px;
    }

    .account-card-body {
        color: rgb(115, 115, 115);
        margin: 10px;
    }
</style>
