import { writable } from "svelte/store"

export const blockStore = writable([], function start(set) {
    const interval = setInterval(async () => {
        let response = await fetch("http://0.0.0.0:8000", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                id: 1,
                jsonrpc: "2.0",
                method: "getBlocks",
            }),
        });
        let data = await response.json();
        if ("error" in data) {
            console.log(data.error.message);
        } else {
            set(data.result);
        }
    }, 100);

    return function stop() {
        clearInterval(interval);
    };
});
