<script lang="ts">
	import '../../app.css';
	import { browser } from '$app/environment';
	import cookie from 'cookie';
	import { User } from '$lib/User';

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
					`/user/name?token=${usr.data.token}`
			).then((res) =>
				res.text().then((text) => {
					usr.data.name = name = text.replaceAll('"', '');
					window.sessionStorage.setItem(
						'usr',
						usr.toString()
					);
				})
			);
		}
	}
</script>

<slot />