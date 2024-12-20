<script lang="ts">
	import Header from '$lib/components/app-header.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
</script>

<div class="flex flex-row w-full h-full bg-neutral-800 text-neutral-300">
	<div class="bg-neutral-900 w-72 hidden md:block">Left Side</div>
	<div class="flex flex-col w-full px-5 py-4">
		<Header />
		<div class="flex flex-col flex-grow h-screen justify-center items-center space-y-7 -mt-24">
			<div class="text-3xl text-neutral-100 font-bold">Good Morning, Harrison.</div>
			<Textarea class="bg-neutral-700 text-neutral-300 border-neutral-600 max-w-176 md:w-[80%]"
								placeholder="What can I help you plan today?" />
		</div>
	</div>
</div>