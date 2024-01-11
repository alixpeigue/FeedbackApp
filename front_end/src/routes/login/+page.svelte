<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { notification } from '$lib/notification_store';
	import Notification from '$lib/components/custom/Notification.svelte';
	export let data;
	export let form;
	let { redirect } = data;
	onMount(() => {
		if(form?.success) {
			notification.set("Successfully logged in");
			goto("/");
		} else {
			toast.error("An error happened");
		}
		if (redirect && form == null) {
			toast.info("You most login to access this page");
		}
	});

</script>

<Notification />
<form method="POST" class="m-auto flex h-screen flex-col justify-center gap-5 md:w-1/3">
	<h1 class="text-center text-3xl font-bold">Log In</h1>
	<div>
		<Input type="text" name="name" placeholder="Username" />
	</div>
	<Button type="submit">Log In</Button>
</form>
