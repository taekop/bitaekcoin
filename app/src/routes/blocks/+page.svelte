<script>
    let blocks = [
        {
            header: {
                bits: 520159231,
                merkle_root: [
                    249, 139, 248, 76, 30, 136, 139, 163, 48, 60, 4, 176, 188,
                    189, 199, 221, 96, 163, 121, 44, 255, 21, 94, 169, 163, 123,
                    121, 97, 63, 211, 111, 168,
                ],
                nonce: 100305,
                prev_block_hash: [
                    0, 0, 201, 22, 51, 191, 88, 229, 25, 108, 243, 148, 167,
                    232, 90, 50, 142, 243, 27, 111, 207, 209, 223, 131, 249,
                    217, 239, 119, 92, 220, 142, 251,
                ],
                timestamp: 1683678844,
                version: 1,
            },
            transactions: [
                {
                    flag: null,
                    inputs: [
                        {
                            output_index: 4294967295,
                            script_sig: [],
                            script_size: 0,
                            sequence: 0,
                            txid: [
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            ],
                        },
                    ],
                    lock_time: 0,
                    outputs: [
                        {
                            amount: 100000000,
                            script_pub_key: [
                                {
                                    PushBytes: {
                                        Bytes: [
                                            33,
                                            [
                                                3, 126, 150, 165, 114, 129, 64,
                                                22, 144, 193, 44, 53, 9, 242,
                                                231, 65, 65, 99, 85, 74, 112,
                                                41, 254, 40, 230, 231, 132, 219,
                                                175, 35, 72, 136, 45,
                                            ],
                                        ],
                                    },
                                },
                                {
                                    Opcode: 172,
                                },
                            ],
                            script_size: 35,
                        },
                    ],
                    version: 1,
                    witnesses: [],
                },
            ],
        },
    ];
</script>

<div class="block-container">
    {#each blocks as block}
        <div class="card">
            <div class="block-card-header">Block #0</div>
            <div class="block-card-body">
                {new Date(block.header.timestamp)}
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
    }
</style>
