<script lang="ts">
	import { browser } from '$app/environment';

    export let data;

    if (browser) {
        console.log(data)
    }
</script>
