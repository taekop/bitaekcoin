<script>
	import Sidebar from "$lib/components/Sidebar.svelte";
	import Toast from "$lib/components/Toast.svelte";
</script>

<div class="app">
	<div class="app-top">
		<Sidebar />

		<main>
			<slot />
		</main>
	</div>

	<footer>
		<p>
			visit <a href="https://github.com/taekop/bitaekcoin">bitaekcoin</a>
		</p>
	</footer>

	<Toast />
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
	}

	.app-top {
		display: flex;
		flex-direction: row;
	}

	main {
		width: 100%;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 12px;
	}

	footer a {
		color: black;
		text-decoration: none;
		font-weight: bold;
	}
</style>
