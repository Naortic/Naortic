<script lang="ts">
	import '../../app.css';
	import { browser } from '$app/environment';
	import cookie from 'cookie';
	import { User } from '$lib/User';
	import ArrowIcon from '../ArrowIcon.svelte';
	import LoadingIcon from '../LoadingIcon.svelte';
	import CrossIcon from '../CrossIcon.svelte';
	import TickIcon from '../TickIcon.svelte';

	export let signedIn = false;
	export let friends: string[] = [];
	export let icon = ArrowIcon;

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
					friends = usr.data.friends ?? [];
					window.sessionStorage.setItem(
						'usr',
						usr.toString()
					);
				})
			);
		} else {
			window.location.href = '/login';
		}
	}

	function addFriend() {
		let friendname = (
			document.getElementById(
				'friendInput'
			) as HTMLInputElement
		).value;

		let usr = User.from(
			window.sessionStorage.getItem('usr')!
		);

		if (usr.data.friends?.includes(friendname)) {
			(
				document.getElementById(
					'friendInput'
				) as HTMLInputElement
			).value = 'Friend already added!';
			setTimeout(() => {
				(
					document.getElementById(
						'friendInput'
					) as HTMLInputElement
				).value = '';
			}, 3000);

			return;
		}

		icon = LoadingIcon;
		fetch(
			import.meta.env.VITE_API_URL +
				`/user/friends/add?token=${usr.data.token}&friend=${friendname}`
		).then((res) =>
			res.text().then((text) => {
				usr.data.friends?.push(text);
				friends.push(text);
				window.sessionStorage.setItem(
					'usr',
					usr.toString()
				);
				icon = TickIcon;
				setTimeout(() => {
					icon = ArrowIcon;
				}, 2000);
			})
		);
	}

	function removeFriend(friend: string) {
		let usr = User.from(
			window.sessionStorage.getItem('usr')!
		);

		icon = LoadingIcon;

		fetch(
			import.meta.env.VITE_API_URL +
				`/user/friends/remove?token=${usr.data.token}&friend=${friend}`
		).then((res) =>
			res.text().then((text) => {
				usr.data.friends = usr.data.friends?.filter(
					(friend) => friend !== text
				);
				friends = friends.filter(
					(friend) => friend !== text
				);
				window.sessionStorage.setItem(
					'usr',
					usr.toString()
				);
				icon = TickIcon;
				setTimeout(() => {
					icon = ArrowIcon;
				}, 2000);
			})
		);
	}
</script>

<div class="bg-base-200 min-h-screen flex flex-row">
	<!-- FRIENDS BAR -->
	<div
		class="m-2 toast toast-top toast-start bg-base-300 rounded h-full max-h-full border-b-2"
	>
		<div class="flex">
			<input
				type="text"
				id="friendInput"
				placeholder="Add new friends"
				class="input my-1 bg-base-100 w-full max-w-xs"
			/>
			<button on:click={addFriend} class="btn mt-1 ml-1"
				><svelte:component this={icon} /></button
			>
		</div>
		{#each friends as friend}
			<div class="alert">
				<a href="/app/dm/{friend}"
					><div
						class="avatar online placeholder bg-base-100 rounded-full"
					>
						<div class="w-8 rounded-full">
							<span>{friend[0].toUpperCase()}</span>
						</div>
					</div>

					<span class="mr-auto">{friend}</span>
				</a>
				<button
					on:click={() => removeFriend(friend)}
					class="btn mt-1 ml-1"
					><svelte:component this={CrossIcon} /></button
				>
			</div>
		{/each}
	</div>
</div>

<slot />
