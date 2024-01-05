<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Separator } from "$lib/components/ui/separator";
	import { ChevronDown, ChevronUp } from "lucide-svelte";
	import ComboBox from "$lib/components/custom/ComboBox.svelte";
	import ComboBoxDataType from "$lib/components/custom/ComboBox.svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import { toast } from "svelte-sonner";
	
    export let data;
    let { reports, clients, locations, contracts } = data;

	let client: ComboBoxDataType | undefined;
	let location: ComboBoxDataType | undefined;
	let contract: ComboBoxDataType | undefined;
	let searchValue: string | undefined = undefined;

	console.log(clients);

	const upvote = async (report) => {
		const url = `http://localhost:3000/reports/${report.id}/upvotes`;
		if (report.upvoted) {
			const promise = fetch(url, {method: 'DELETE', credentials: 'include'});
			toast.promise(promise, {
				loading: 'Removing upvote...',
				success: (_) => {
					report.upvotes -= 1
					report.upvoted = false;
					reports = reports;
					return "Successfully removed upvote";
				},
				error: "An error happened",
			});
		} else {
			const promise = fetch(url, {method: 'PUT', credentials: 'include'});
			toast.promise(promise, {
				loading: 'Upvoting...',
				success: (_) => {
					report.upvotes += 1
					report.upvoted = true;
					reports = reports;
					return "Successfully upvoted";
				},
				error: "An error happened",
			});
		}
	} 

	const search = async () => {
		const res = await fetch(`http://localhost:3000/reports?` + 
			new URLSearchParams({search: searchValue || "" , client: "" + (client?.id || "1")}),
			{credentials: 'include'}
		);
		reports = await res.json();
	}

</script>

<Toaster/>

<div class="flex items-center justify-between">
<h1 class="text-4xl font-bold my-10">View Reports</h1>
<Button>New report</Button>
</div>
<!--<form class="flex gap-5 mb-10 flex-col lg:flex-row">-->
<form class="grid lg:grid-cols-4 grid-cols-1 gap-5 mb-10">
	<Input placeholder="search" bind:value={searchValue} class="lg:col-span-3"/>
	<ComboBox list={clients} defaultValue="Search clients..." bind:selectedValue={client}/>
	<ComboBox list={contracts} defaultValue="Search contracts..." bind:selectedValue={contract}/>
	<ComboBox list={locations} defaultValue="Search locations..." bind:selectedValue={location}/>
	<Button on:click={search} class="lg:row-start-1 lg:col-start-4">Search</Button>
</form>
{#if reports.length != 0}
<div class="flex flex-col gap-5">
	{#each reports as report}
		<div class="flex items-center gap-10">
			<div class="flex flex-col items-center">
				<button on:click={() => upvote(report)}>
					<ChevronUp strokeWidth="3" class="{report.upvoted ? '' : 'opacity-30'}" />
				</button>				
					{report.upvotes}
				<ChevronDown strokeWidth="3" class="opacity-30"/>
			</div>
			<p>{report.text}</p>
		</div>
		<Separator/>
	{/each}
</div>
{:else}
	<p class="text-center opacity-50">Nothing found</p>
{/if}
