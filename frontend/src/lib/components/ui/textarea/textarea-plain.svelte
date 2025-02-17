<script lang="ts">
	import type { WithElementRef, WithoutChildren } from 'bits-ui';
	import type { HTMLTextareaAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		value = $bindable(),
		height = $bindable(),
		class: className,
		...restProps
	}: WithoutChildren<WithElementRef<TextareaProps>> = $props();

	interface TextareaProps extends HTMLTextareaAttributes {
		height?: number;
	}

	function updateHeight() {
		if (ref) {
			ref.style.height = '';
			let newHeight = Math.min(ref.scrollHeight, 192);
			ref.style.height = newHeight + 'px';
			height = newHeight;
		}
	}
</script>

<textarea
	{...restProps}
	bind:this={ref}
	bind:value
	class={cn(
		"outline-none bg-background min-h-[25px]",
		className
	)}
	oninput={updateHeight}
	rows="1"
	style="resize: none;"
></textarea>
