<script lang="ts">
	import '../../app.css';
	import { browser } from '$app/environment';
	import cookie from 'cookie';
	import GitHubIcon from '../GitHubIcon.svelte';
	import { User } from '$lib/User';
	import ArrowIcon from '../ArrowIcon.svelte';

	export let signedIn = false;
	export let name = '';

	if (browser) {
		let usr = window.sessionStorage.getItem('usr')
			? User.from(window.sessionStorage.getItem('usr')!)
			: new User({
					token: cookie.parse(document.cookie).token
			  });

		signedIn = typeof usr.data.token != 'undefined';

		if (signedIn) {
			fetch(
				import.meta.env.VITE_API_URL +
					`/user?token=${usr.data.token}`
			).then((res) =>
				res.json().then((json) => {
					usr.data = json;
					usr.data.name = name = json.name.replaceAll(
						'"',
						''
					);
					window.sessionStorage.setItem(
						'usr',
						usr.toString()
					);
				})
			);
		}
	}
</script>

<div class="navbar bg-base-100">
	<div class="navbar-start">
		<a href="/" class="btn btn-ghost normal-case text-xl"
			>Naortic</a
		>
	</div>
	<div class="navbar-end">
		<a
			href="https://github.com/Naortic/Naortic"
			class="btn btn-ghost"
			><svelte:component this={GitHubIcon} /></a
		>
		{#if signedIn}<a href="/app" class="ml-2 btn">
				App<svelte:component this={ArrowIcon} />
			</a>{/if}
		{#if !signedIn}<a href="/login" class="ml-2 btn"
				>Login</a
			>{/if}
	</div>
</div>

<slot />
