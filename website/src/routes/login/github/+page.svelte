<script lang="ts">
    import { browser } from "$app/environment";

    if (browser) {
        const code = new URLSearchParams(window.location.href.split("?")[1]).get('code')

        let token = fetch(import.meta.env.VITE_API_URL + `/login/github?code=${code}`)
        .then(res => res.text()
            .then(token => {
                document.cookie = `token=${token};`;
        }));
    }
</script>

<div>
    Logging in...
</div>