<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Separator } from '$lib/components/ui/separator';
	import { ChevronUp } from 'lucide-svelte';
	import ComboBox from '$lib/components/custom/ComboBox.svelte';
	import ComboBoxDataType from '$lib/components/custom/ComboBox.svelte';
	import { toast } from 'svelte-sonner';
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { PUBLIC_SERVER_URL } from '$env/static/public';
	import { goto } from '$app/navigation';
	import Notification from '$lib/components/custom/Notification.svelte';

	export let data;
	let { reports, clients, locations, contracts } = data;

	let client: ComboBoxDataType | undefined;
	let location: ComboBoxDataType | undefined;
	let contract: ComboBoxDataType | undefined;
	let searchValue: string | undefined = undefined;

	console.log(clients);

	const upvote = async (report) => {
		const url = `${PUBLIC_SERVER_URL}/reports/${report.id}/upvotes`;
		if (report.upvoted) {
			const promise = fetch(url, { method: 'DELETE', credentials: 'include' });
			toast.promise(promise, {
				loading: 'Removing upvote...',
				success: (_) => {
					report.upvotes -= 1;
					report.upvoted = false;
					reports = reports;
					return 'Successfully removed upvote';
				},
				error: 'An error happened'
			});
		} else {
			const promise = fetch(url, { method: 'PUT', credentials: 'include' });
			toast.promise(promise, {
				loading: 'Upvoting...',
				success: (_) => {
					report.upvotes += 1;
					report.upvoted = true;
					reports = reports;
					return 'Successfully upvoted';
				},
				error: 'An error happened'
			});
		}
	};

	const invalid = async (report) => {
		const url = `${PUBLIC_SERVER_URL}/reports/${report.id}/invalid_votes`;
		const promise = fetch(url, { method: 'PUT', credentials: 'include'});
		toast.promise(promise, {
			loading: 'Marking as invalid...',
			success: "Successfully markes ad invalid",
			error: 'An error happened',
		});
		
	}

	const search = async () => {
		const res = await fetch(
			`${PUBLIC_SERVER_URL}/reports?` +
				new URLSearchParams(JSON.parse(JSON.stringify({ search: searchValue ? searchValue.split(' ').join('&') : undefined, client: client?.value.toString(), contract: contract?.value.toString(), location: location?.value.toString()}))),
			{ credentials: 'include' }
		);
		reports = await res.json();
	};

	const logout = async () => {
		const res = await fetch(`${PUBLIC_SERVER_URL}/logout`, {
			credentials: "include",
		});
		if(res.ok) {
			goto("/login?redirected=true");
		}
	}

</script>

<Notification />

<div class="flex items-center justify-between">
	<h1 class="my-10 text-4xl font-bold">View Reports</h1>
	<div class="flex gap-5">
		<Button on:click={logout} variant="outline">Log out</Button>
		<Button href="newreport">New report</Button>
	</div>
</div>
<!--<form class="flex gap-5 mb-10 flex-col lg:flex-row">-->
<form class="mb-10 grid grid-cols-1 gap-5 lg:grid-cols-4" on:submit|preventDefault={search}>
	<Input placeholder="search" bind:value={searchValue} class="lg:col-span-3" />
	<ComboBox list={clients} defaultValue="Search clients..." bind:selectedValue={client} />
	<ComboBox list={contracts} defaultValue="Search contracts..." bind:selectedValue={contract} />
	<ComboBox list={locations} defaultValue="Search locations..." bind:selectedValue={location} />
	<Button type="submit" class="lg:col-start-4 lg:row-start-1">Search</Button>
</form>
{#if reports.length != 0}
	<div class="flex flex-col gap-5">
		{#each reports as report}
			<div class="flex items-center gap-10">
				<div class="flex flex-col items-center">
					<button on:click={() => upvote(report)}>
						<ChevronUp strokeWidth="3" class={report.upvoted ? '' : 'opacity-30'} />
					</button>
					{report.upvotes}
				</div>
				<p>{report.text}</p>
				<DropdownMenu.Root>
				    <DropdownMenu.Trigger class="ml-auto text-xl">...</DropdownMenu.Trigger>
				    <DropdownMenu.Content>
				    	<DropdownMenu.Group>
				      		<DropdownMenu.Item on:click={()=>invalid(report)}>Mark as invalid report</DropdownMenu.Item>
				    	</DropdownMenu.Group>
				  	</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
			<Separator />
		{/each}
	</div>
{:else}
	<p class="text-center opacity-50">Nothing found</p>
{/if}
