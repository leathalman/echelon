<script lang="ts">
	import type { WithElementRef, WithoutChildren } from 'bits-ui';
	import type { HTMLTextareaAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		value = $bindable(),
		class: className,
		...restProps
	}: WithoutChildren<WithElementRef<HTMLTextareaAttributes>> = $props();

	function updateHeight() {
		if (ref) {
			ref.style.height = '';
			ref.style.height = ref.scrollHeight + 'px';
		}
	}
</script>

<textarea
	bind:this={ref}
	class={cn(
		"outline-none bg-background min-h-[25px] rounded-md ",
		className
	)}
	bind:value
	{...restProps}
	style="resize: none;"
	rows="1"
	oninput={updateHeight}
></textarea>
