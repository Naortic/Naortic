<script lang="ts">
    import { browser } from "$app/environment";

    if (browser) {
        const code = new URLSearchParams(window.location.href.split("?")[1]).get('code')

        fetch(import.meta.env.VITE_API_URL + `/login/github?code=${code}`)
        .then(res => res.text()
            .then(token => {
                token = new URLSearchParams(token).get("access_token") ?? "";
                document.cookie = `token=${token};`;
                window.sessionStorage.setItem('token', token);
        }));
    }
</script>

<div>
    Logging in...
</div>