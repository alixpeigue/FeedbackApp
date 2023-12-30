<script lang="ts">
    export let data;
    let { reports } = data;
</script>

<h1>Welcome to SvelteKit</h1>
{#each reports as report}
	<p>{report.text}</p>
{/each}
