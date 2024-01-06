<script lang="ts">
	import ComboBox, { type ComboBoxDataType } from '$lib/components/custom/ComboBox.svelte';
	import * as Form from '$lib/components/ui/form';
	import { formSchema, type FormSchema } from './schema';
	import type { SuperValidated } from 'sveltekit-superforms';

	export let form: SuperValidated<FormSchema>;
	export let locations: ComboBoxDataType[];
	export let contracts: ComboBoxDataType[];
</script>

<Form.Root method="POST" {form} schema={formSchema} let:config>
	<Form.Field {config} name="text">
		<Form.Item>
			<Form.Label>Text</Form.Label>
			<Form.Input />
			<Form.Description>This is the description of the report</Form.Description>
			<Form.Validation />
		</Form.Item>
	</Form.Field>
	<Form.Field {config} name="location">
		<Form.Item>
			<Form.Label>Location</Form.Label>
			<ComboBox list={locations} />
			<Form.Validation />
		</Form.Item>
	</Form.Field>
	<Form.Field {config} name="contract">
		<Form.Item>
			<Form.Label>Contract</Form.Label>
			<ComboBox list={contracts} />
			<Form.Validation />
		</Form.Item>
	</Form.Field>
	<Form.Button>Submit</Form.Button>
</Form.Root>
