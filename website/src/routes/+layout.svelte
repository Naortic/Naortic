<script>
    import "../app.css";
    import { browser } from "$app/environment";
    import cookie from "cookie";
	  import GitHubIcon from "./GitHubIcon.svelte";

    export let isSignedIn = false;
    export let name = ""

    if (browser) {

      (async () => {
        let token = cookie.parse(document.cookie).token;

        isSignedIn = typeof token != "undefined";

        console.log(token, isSignedIn);

        if (isSignedIn) {
          name = await (await fetch(import.meta.env.VITE_API_URL + `/user/name?token=${token}`)).text();
          name = name.replaceAll('"', '');
          window.sessionStorage.setItem("name", name);
        }
      })();
    }
      
</script>

<div class="navbar bg-base-100">
    <div class="navbar-start">
      <a href="/" class="btn btn-ghost normal-case text-xl">Naortic</a>
    </div>
    <div class="navbar-end">
      <a href="https://github.com/Naortic/Naortic"><svelte:component this={GitHubIcon} /></a>
      {#if isSignedIn}<div class="ml-2 btn btn-disabled">{name}</div>{/if}
      {#if !isSignedIn}<a href="/login" class="ml-2 btn">Login</a>{/if}
    </div>
  </div>
  
<slot />