<script>
    import { onMount } from "svelte";
    import plus from "$lib/images/plus.png";

    let accounts = [];

    onMount(async () => {
        fetch("http://0.0.0.0:8000", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: "2.0",
                method: "getAccounts",
            }),
        })
            .then((response) => response.json())
            .then((data) => {
                accounts = data.result;
            })
            .catch((error) => {
                console.log(error);
            });
    });

    async function createAccount() {
        console.log(1);
        let res = await fetch("http://0.0.0.0:8000", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: "2.0",
                method: "createAccount",
            }),
        });
        console.log(res.json());
        console.log(2);
    }
</script>

<div class="accounts-body">
    <div class="new-account-container">
        <button class="new-account-btn" on:click={createAccount}>
            <img class="plus-img" src={plus} alt="" />
            New Account
        </button>
    </div>
    {#each accounts as account}
        <div class="account-card">
            <div class="account-card-header">Account #{account.index}</div>
            <div class="account-card-body">
                {account.balance / 1000000000} BTC ({account.balance} SAT)
            </div>
        </div>
    {/each}
</div>

<style>
    .accounts-body {
        width: 100%;
        padding: 32px;
        box-sizing: border-box;
    }

    .account-card {
        box-shadow: rgba(0, 0, 0, 0.2) 0px 3px 1px -2px,
            rgba(0, 0, 0, 0.14) 0px 2px 2px 0px,
            rgba(0, 0, 0, 0.12) 0px 1px 5px 0px;
        padding: 16px;
        padding-inline: 16px;
        margin-top: 16px;
        margin-bottom: 16px;
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
    .new-account-container {
        display: flex;
        flex-direction: row-reverse;
    }

    .plus-img {
        width: 25px;
        margin-right: 15px;
    }

    .new-account-btn {
        cursor: pointer;
        position: relative;
        outline: none;
        border: none;
        font-size: 18px;
        border-radius: 30px;
        padding: 18px 40px;
        font-weight: 500;
        line-height: 20px;
        background: #275efe;
        color: white;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .new-account-btn:before {
        content: "";
        background: linear-gradient(
            45deg,
            #ff0000,
            #ff7300,
            #fffb00,
            #48ff00,
            #00ffd5,
            #002bff,
            #7a00ff,
            #ff00c8,
            #ff0000
        );
        position: absolute;
        top: -2px;
        left: -2px;
        background-size: 400%;
        z-index: -1;
        filter: blur(5px);
        width: calc(100% + 4px);
        height: calc(100% + 4px);
        animation: glowing 20s linear infinite;
        opacity: 0;
        transition: opacity 0.3s ease-in-out;
        border-radius: 30px;
    }

    .new-account-btn:active {
        background-color: #002bff;
    }

    .new-account-btn:active:after {
        background: transparent;
    }

    .new-account-btn:hover:before {
        opacity: 1;
    }

    .new-account-btn:after {
        z-index: -1;
        content: "";
        position: absolute;
        width: 100%;
        height: 100%;
        left: 0;
        top: 0;
        border-radius: 30px;
    }

    @keyframes glowing {
        0% {
            background-position: 0 0;
        }
        50% {
            background-position: 400% 0;
        }
        100% {
            background-position: 0 0;
        }
    }
</style>
