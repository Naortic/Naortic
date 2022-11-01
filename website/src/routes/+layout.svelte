<script>
	import '../app.css';
	import { browser } from '$app/environment';
	import cookie from 'cookie';
	import GitHubIcon from './GitHubIcon.svelte';
	import { User } from '$lib/User';

	export let isSignedIn = false;
	export let name = '';

	if (browser) {
		let usr = window.sessionStorage.getItem('usr')
			? User.from(
					window.sessionStorage.getItem('usr') ?? ''
			  )
			: new User({
					token: cookie.parse(document.cookie).token
			  });

		if (typeof usr.data.token != 'undefined') {
			fetch(
				import.meta.env.VITE_API_URL +
					`/user/name?token=${usr.data.token}`
			).then((res) =>
				res.text().then((text) => {
					usr.data.name = text.replaceAll('"', '');
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
		<a href="https://github.com/Naortic/Naortic"
			><svelte:component this={GitHubIcon} /></a
		>
		{#if isSignedIn}<div class="ml-2 btn btn-disabled">
				{name}
			</div>{/if}
		{#if !isSignedIn}<a href="/login" class="ml-2 btn"
				>Login</a
			>{/if}
	</div>
</div>

<slot />
