<script>
    import { slide } from "svelte/transition";
    import IoIosSend from "svelte-icons/io/IoIosSend.svelte";

    import { accountStore } from "$lib/stores/account.js";
    import { notificationStore } from "$lib/stores/notification.js";
    import plus from "$lib/images/plus.png";

    let accounts;
    accountStore.subscribe((value) => {
        accounts = value;
    });

    let transferFrom;
    let transferTo;
    let transferAmount;

    async function createAccount() {
        let response = await fetch("http://0.0.0.0:8000", {
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
        let data = await response.json();
        if ("error" in data) {
            notificationStore.danger(data.error.message, 1000);
        } else {
            notificationStore.success(
                `Account #${data.result.index} created.`,
                1000
            );
        }
    }

    function renderTransfer(i) {
        transferTo = null;
        transferAmount = "";
        if (transferFrom === i) {
            transferFrom = null;
        } else {
            transferFrom = i;
        }
    }

    async function transfer() {
        let response = await fetch("http://0.0.0.0:8000", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: "2.0",
                method: "transfer",
                params: [transferFrom, transferTo, Number(transferAmount)],
            }),
        });
        let data = await response.json();
        if ("error" in data) {
            notificationStore.danger(data.error.message, 1000);
        } else {
            notificationStore.success(
                `Transaction sent! Send ${transferAmount} from ${transferFrom} to ${transferTo}.`,
                1000
            );
        }
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
        <div class="account-card" transition:slide|local>
            <div class="account-card-header">Account #{account.index}</div>
            <div class="account-card-body">
                {(account.balance / 1000000000).toFixed(9)} BTC ({account.balance}
                SAT)
                <button
                    class="transfer-btn"
                    on:click={renderTransfer(account.index)}
                    >Transfer<span class="arrow" /></button
                >
            </div>
        </div>
        {#if transferFrom === account.index}
            <div
                class="transfer-card"
                id="transfer-card-{account.index}"
                transition:slide
            >
                <div class="transfer-form">
                    <table>
                        <tr>
                            <th>From</th>
                            <th>To</th>
                            <th>Amount</th>
                        </tr>
                        <tr>
                            <td>
                                <div class="transfer-select">
                                    <select disabled bind:value={transferFrom}>
                                        <option value={account.index}>
                                            Account #{account.index}
                                        </option>
                                    </select>
                                </div>
                            </td>
                            <td>
                                <div class="transfer-select">
                                    <select bind:value={transferTo}>
                                        <option selected disabled
                                            >Choose an account to receive</option
                                        >
                                        {#each accounts as account}
                                            <option value={account.index}>
                                                Account #{account.index}
                                            </option>
                                        {/each}
                                    </select>
                                </div>
                            </td>
                            <td>
                                <div class="transfer-input">
                                    <input bind:value={transferAmount} />
                                </div>
                            </td>
                            <td>
                                <button
                                    class="transfer-submit"
                                    disabled={!parseInt(transferAmount) ||
                                        !parseInt(transferTo)}
                                    on:click={transfer}
                                >
                                    Submit!
                                    <i class="ico">
                                        <IoIosSend />
                                    </i>
                                </button>
                            </td>
                        </tr>
                    </table>
                </div>
            </div>
        {/if}
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

    .account-card:hover {
        background-color: #eee;
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
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
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

    .transfer-btn {
        display: flex;
        color: #c2ffe9;
        background-color: #00e692;
        padding: 10px 16px;
        border-radius: 20px;
        transition: all 0.3s ease;
        font-weight: bold;
        cursor: pointer;
        align-items: center;
        font-size: 14px;
        border: none;
    }

    .transfer-btn > .arrow {
        width: 6px;
        height: 6px;
        border-right: 2px solid #c2ffe9;
        border-bottom: 2px solid #c2ffe9;
        position: relative;
        transform: rotate(-45deg);
        margin: 0 6px;
        transition: all 0.3s ease;
    }

    .transfer-btn > .arrow::before {
        display: block;
        background-color: currentColor;
        width: 3px;
        transform-origin: bottom right;
        height: 2px;
        position: absolute;
        opacity: 0;
        bottom: calc(-2px / 2);
        transform: rotate(45deg);
        transition: all 0.3s ease;
        content: "";
        right: 0;
    }

    .transfer-btn:hover > .arrow {
        transform: rotate(-45deg) translate(4px, 4px);
        border-color: text-hover-color;
    }

    .transfer-btn:hover > .arrow::before {
        opacity: 1;
        width: 8px;
    }

    .transfer-btn:hover {
        background-color: #017a4f;
        color: #fff;
    }

    .transfer-form {
        display: flex;
        flex-direction: row;
    }

    select {
        -webkit-appearance: none;
        -moz-appearance: none;
        -ms-appearance: none;
        appearance: none;
        outline: 0;
        box-shadow: none;
        border: 0 !important;
        flex: 1;
        padding: 0 0.5em;
        cursor: pointer;
        font-size: 1em;
        font-family: "Open Sans", sans-serif;
    }

    select::-ms-expand {
        display: none;
    }

    .transfer-select {
        position: relative;
        display: flex;
        width: 8em;
        height: 2em;
        line-height: 3;
        overflow: hidden;
        border-radius: 0.25em;
        margin: 0.5em;
    }

    .transfer-select::after {
        content: "\25BC";
        position: absolute;
        top: 0;
        right: 0;
        font-size: 0.7em;
        padding: 0 0.7em;
        background: #a5a5a5;
        cursor: pointer;
        pointer-events: none;
        transition: 0.25s all ease;
    }
    .transfer-select:hover::after {
        color: #7af4a3;
    }

    .transfer-submit {
        font-size: 1em;
        padding: 10px 40px 10px 20px;
        background-color: #b4363f;
        color: #fff;
        border-radius: 50px;
        margin-left: 30px;
        border: 1px solid #b4363f;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .transfer-submit:enabled {
        transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
        background-color: #e94751;
        border: 1px solid #e94751;
    }

    .transfer-submit:enabled:hover {
        transform: scale(1.05);
        background-color: transparent;
        color: #e94751;
        border: 2px solid #e94751;
    }

    .transfer-submit:enabled:hover .ico {
        background-color: #e94751;
        color: white;
        transform: rotate(360deg);
    }

    .transfer-submit .ico {
        background-color: white;
        color: #e94751;
        width: 20px;
        height: 20px;
        padding: 2px;
        border-radius: 10px;
        position: absolute;
        margin-left: 90px;
    }

    .transfer-submit:enabled .ico {
        transition: all 0.5s;
    }
</style>
