<script lang="ts">
	import { goto } from "$app/navigation";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
</script>

<form method="POST" class="md:w-1/3 m-auto flex flex-col gap-5 h-screen justify-center">
	<h1 class="text-3xl font-bold text-center">Log In</h1>
	<div>
		<Input type="text" name="name" placeholder="Username"/>
	</div>
	<Button type="submit">Log In</Button>
</form>

