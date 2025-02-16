<script lang="ts">
	import temp from '/static/temp_logo.svg';
	import Plain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-5">
	<div class="w-[90%] md:max-w-156 items-start flex flex-col space-y-4">
		<img src={temp} width="25" height="25" alt="Temp Icon" />
		<span class="text-lg">Good Morning, Harrison.</span>
	</div>
	<div
		class="w-[90%] md:max-w-156 flex flex-col rounded-lg h-32
					shadow-lg border border-black">
		<Plain class="text-lg font-semibold mx-1 px-2 my-2 placeholder-opacity-5"
					 placeholder="How can I help?"></Plain>
		<div class="flex w-full h-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9">
				<ArrowRight class="h-4 w-4" />
			</Button>
		</div>
	</div>
</div>