<script>
    import "../app.css";
    import { browser } from "$app/environment";
    import cookie from "cookie";
	  import GitHubIcon from "./GitHubIcon.svelte";

    export let loggedin = false;
    export let name = ""

    if (browser) {
      let token = cookie.parse(document.cookie).token;

      loggedin = token != "";

      if (loggedin) {
        fetch(import.meta.env.VITE_API_URL + `/user/name?token=${token}`)
        .then(res => res.text()
        .then(text => { 
          name = text; 
          window.sessionStorage.setItem("login", name)
        }));
      }
    }
</script>

<div class="navbar bg-base-100">
    <div class="navbar-start">
      <a href="/" class="btn btn-ghost normal-case text-xl">Naortic</a>
    </div>
    <div class="navbar-end">
      <a href="https://github.com/Naortic/Naortic"><svelte:component this={GitHubIcon} /></a>
      {#if loggedin}<div class="ml-2 btn btn-disabled">{name.replaceAll('"', '')}</div>{/if}
      {#if !loggedin}<a href="/login" class="ml-2 btn">Login</a>{/if}
    </div>
  </div>
  
<slot />