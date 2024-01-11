<script lang="ts">
	import { notification } from "$lib/notification_store";
	import { onMount } from "svelte";
	import { toast } from "svelte-sonner";
	import { get } from "svelte/store";
	import { Toaster } from "../ui/sonner";

	onMount(() => {
		const notif = get(notification);
		if (notif != null) {
			notification.set(null)
			toast.info(notif);
		}
	})
</script>

<Toaster />
