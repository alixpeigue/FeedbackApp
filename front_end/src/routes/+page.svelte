<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Separator } from "$lib/components/ui/separator";
	import { ChevronDown, ChevronUp } from "lucide-svelte";
	import ComboBox from "$lib/components/custom/ComboBox.svelte";
	import ComboBoxDataType from "$lib/components/custom/ComboBox.svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	
    export let data;
    let { reports, clients, locations } = data;

	let client: ComboBoxDataType | undefined;
	let location: ComboBoxDataType | undefined;
	let searchValue: string | undefined = undefined;

	console.log(clients);

	const upvote = async (report) => {
		if (report.upvoted) {
			const res = await fetch(`http://localhost:3000/reports/${report.id}/upvotes`, {method: 'DELETE', credentials: 'include'});
			if (res.status === 200) {
				report.upvotes -= 1;
				report.upvoted = false;
				reports = reports;
			}		
		} else {
			console.log("Oui!!");
			const res = await fetch(`http://localhost:3000/reports/${report.id}/upvotes`, {method: 'PUT', credentials: 'include'});
			console.log(res.status);
			if(res.status === 201) {
				report.upvotes += 1
				report.upvoted = true;
				reports = reports;
			}
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

<Toaster />

<h1 class="text-4xl font-bold my-10">View Reports</h1>
<div class="flex gap-5 mb-10 flex-col lg:flex-row">
	<Input placeholder="search" bind:value={searchValue}/>
	<ComboBox list={clients} defaultValue="Search clients..." bind:selectedValue={client}/>
	<ComboBox list={locations} defaultValue="Search locations..." bind:selectedValue={location}/>
	<Button on:click={search}>Search</Button>
</div>
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
