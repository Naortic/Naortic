<script>
	import GitHubIcon from "../GitHubIcon.svelte";
</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">Log in</h1>
            <p class="py-6">You can log in to your Naortic account with GitHub.</p>
                <div class="mt-2">
                    <a href={"https://github.com/login/oauth/authorize?client_id=" + import.meta.env.VITE_CLIENT_ID} class="btn btn-primary w-32">Github<svelte:component this={GitHubIcon} />
                    </a>
                </div>
        </div>
    </div>
</div>