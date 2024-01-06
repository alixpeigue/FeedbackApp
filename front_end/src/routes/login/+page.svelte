<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
</script>

<form method="POST" class="m-auto flex h-screen flex-col justify-center gap-5 md:w-1/3">
	<h1 class="text-center text-3xl font-bold">Log In</h1>
	<div>
		<Input type="text" name="name" placeholder="Username" />
	</div>
	<Button type="submit">Log In</Button>
</form>
