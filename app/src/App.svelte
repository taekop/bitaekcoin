<script>
	import Accounts from "./Accounts.svelte";
	import Blocks from "./Blocks.svelte";
	import Sidebar from "./Sidebar.svelte";

	let menu = 1;
</script>

<div class="App">
	<Sidebar>
		<button on:click|preventDefault={() => (menu = 0)}>hello</button>
	</Sidebar>

	{#if menu === 0}
		<Blocks />
	{:else}
		<Accounts />
	{/if}
</div>

<style>
	.App {
		display: flex;
		flex-direction: row;
	}
</style>
