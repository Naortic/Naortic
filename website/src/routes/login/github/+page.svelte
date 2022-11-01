<script lang="ts">
	import { browser } from '$app/environment';
	import { User } from '$lib/User';

	if (browser) {
		const code = new URLSearchParams(
			window.location.href.split('?')[1]
		).get('code');

		fetch(
			import.meta.env.VITE_API_URL +
				`/login/github?code=${code}`
		).then((res) =>
			res.text().then((token) => {
				if (new URLSearchParams(token).has('access_token')) {
					token = new URLSearchParams(token).get('access_token')!;
				} else {
					return;
				}
				document.cookie = `token=${token} ;path=/ ;max-age=2592000 ;Secure`;
				window.sessionStorage.setItem(
					'usr',
					new User({ token }).toString()
				);
			})
		);
	}
</script>

<div>Logging in...</div>
