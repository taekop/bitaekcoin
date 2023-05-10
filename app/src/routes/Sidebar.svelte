<script>
	import { page } from "$app/stores";

	import account from "$lib/images/account.svg";
	import block from "$lib/images/block.svg";
	import logo from "$lib/images/logo.svg";
</script>

<header>
	<nav>
		<a href="/">
			<div class="home">
				<img class="logo-img" src={logo} alt="" />
				<p>Bitaekcoin</p>
			</div>
		</a>
		<a href="/blocks">
			<div class="route">
				<img class="route-img" src={block} alt="" />
				<p>Blocks</p>
			</div>
		</a>
		<a href="/accounts">
			<div class="route">
				<img class="route-img" src={account} alt="" />
				<p>Accounts</p>
			</div>
		</a>
	</nav>
</header>

<style>
	header {
		width: 300px;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: start;
	}

	nav {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	a {
		color: black;
		text-decoration: none;
	}

	div {
		padding: 10px;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
	}

	.home {
		border-bottom: 1px solid black;
		font-size: 30px;
	}

	.route {
		border-bottom: 1px solid gray;
		font-size: 20px;
	}

	.logo-img {
		width: 60px;
	}

	.route-img {
		width: 45px;
	}
</style>
