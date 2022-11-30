<script lang="ts">
	import { User } from '$lib/User';
	import ArrowIcon from '../../ArrowIcon.svelte';

	let show = 0;
	let email = '';
	let password = '';
	export let text = 'Logging in...';

	function detailsEntered() {
		email =
			(document.getElementById('email') as HTMLInputElement)
				.value ?? '';
		password =
			(
				document.getElementById(
					'password'
				) as HTMLInputElement
			).value ?? '';
		show++;
		fetch(
			import.meta.env.VITE_API_URL +
				`/login?email=${email}&name=${name}&password=${password}`
		).then((res) => {
			if (!res.ok) {
				return (text =
					'Login failed, please try again later...');
			}
			res.text().then((token) => {
				document.cookie = `token=${token} ;path=/ ;max-age=2592000 ;Secure`;
				window.sessionStorage.setItem(
					'usr',
					new User({ token }).toString()
				);
				text = 'Logged in successfully';
				setTimeout(() => {
					window.location.href = '/';
				}, 1000);
			});
		});
	}
</script>

<div class="hero min-h-screen bg-base-200">
	<div class="hero-content text-center">
		<div class="max-w-md">
			{#if show == 0}
				<h1 class="text-5xl font-bold">Log in</h1>
				<p class="py-6">Enter your details to continue.</p>
				<div class="flex flex-col">
					<input
						type="text"
						placeholder="Email"
						id="email"
						class="input input-bordered bg-primary w-full max-w-s mt-2 mx-auto"
					/>
					<input
						type="text"
						placeholder="Password"
						id="password"
						class="input input-bordered bg-primary w-full max-w-s mt-2 mx-auto"
					/>
					<button
						on:click={detailsEntered}
						class="mt-2 ml-auto btn btn-primary"
						><svelte:component this={ArrowIcon} /></button
					>
				</div>
			{/if}
			{#if show == 1}
				<div class="text-2xl">{text}</div>
			{/if}
		</div>
	</div>
</div>
