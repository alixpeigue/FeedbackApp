<script lang="ts">
	import { goto } from '$app/navigation';
    import { backend_url } from '$lib/utils.js';
	import ComboBox, { type ComboBoxDataType } from '$lib/components/custom/ComboBox.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { notification } from '$lib/notification_store';
	import type { PageData } from './$types';
	export let data: PageData;
	let { locations, contracts } = data;
	let contract: ComboBoxDataType | undefined;
	let location: ComboBoxDataType | undefined;
	let text: string;
    let isSubmitClicked: boolean = false;

	const submit = async (e: SubmitEvent) => {
		const formData = new FormData(e.target as HTMLFormElement);
		if (contract === undefined || location === undefined) {
			return;
		}
        isSubmitClicked = true;
		const res = await fetch(`${backend_url()}/reports`, {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				text: formData.get('text'),
				contract: parseInt(contract?.value),
				location: parseInt(location?.value)
			})
		});
		if (res.ok) {
			notification.set("Successfully created report");
			goto('/');
		}
	};
</script>

<!--<ReportForm form={form} locations={locations} contracts={contracts}/>-->

<div class="mx-auto lg:w-2/3">
	<h1 class="my-10 text-xl">New report</h1>

	<form class="grid grid-cols-1 gap-5 lg:grid-cols-3" on:submit|preventDefault={submit}>
		<Textarea
			placeholder="Describe your report with as much details as you can"
			bind:value={text}
			class="lg:col-span-3"
			name="text"
		/>
		<ComboBox list={contracts} defaultValue="Select contract..." bind:selectedValue={contract} />
		<ComboBox list={locations} defaultValue="Select location..." bind:selectedValue={location} />
		<Button type="submit" disabled={isSubmitClicked}>Create report</Button>
	</form>
</div>
