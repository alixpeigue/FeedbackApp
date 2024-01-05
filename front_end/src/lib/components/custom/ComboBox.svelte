<script context="module" lang="ts">
    export type ComboBoxDataType = {
        value: string,
        label: string
    };
</script>

<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { tick } from "svelte";
	import * as Command from "$lib/components/ui/command";
  	import * as Popover from "$lib/components/ui/popover";
	import { cn } from "$lib/utils.js";
	import { Check, ChevronsUpDown } from "lucide-svelte";

    export let defaultValue: string = "Search"


    export let list: ComboBoxDataType[];
    
    let open = false;
    let value = "";
    
    export let selectedValue: ComboBoxDataType | undefined = undefined;
	$: selectedValue = list.find((f) => f.label === value)

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
    }
    
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<Button
	      	builders={[builder]}
	      	variant="outline"
	      	role="combobox"
	      	aria-expanded={open}
	      	class="w-full justify-between"
		>
      		{selectedValue?.label || defaultValue}
      		<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-[200px] p-0">
		<Command.Root>
  			<Command.Input placeholder={defaultValue} />
	      	<Command.Empty>No framework found.</Command.Empty>
	      	<Command.Group>
        		{#each list as element}
          			<Command.Item
            			value={element.label}
            			onSelect={(currentValue) => {
              				value = currentValue;
              				closeAndFocusTrigger(ids.trigger);
            			}}
          			>
	            		<Check
	              			class={cn(
	                			"mr-2 h-4 w-4",
	                			value !== element.value && "text-transparent"
	              			)}
	            		/>
	            		{element.label}
          			</Command.Item>
        		{/each}
  			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
