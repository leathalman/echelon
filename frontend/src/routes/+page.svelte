<script lang="ts">
	import Header from '$lib/components/app-header.svelte';
	import TextareaNoFocus from '$lib/components/textarea-no-focus.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-7">
	<span class="text-3xl font-bold text-center">Good Morning, Harrison.</span>
	<div
		class="box w-[80%] md:max-w-176 h-28 flex flex-col items-center rounded-md border focus-within:ring-ring focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2">
		<TextareaNoFocus class="w-full"
										 placeholder="What can I help you plan today?" />
		<div class="flex w-full justify-end p-2">
			<Button class="disabled" variant="default" size="icon">
				<ArrowRight size={10} />
			</Button>
		</div>
	</div>
</div>