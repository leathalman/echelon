<script lang="ts">
	import Header from '$lib/components/app-header.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
</script>

<div class="flex flex-row w-full h-full bg-custom text-foreground">
	<div class="bg-neutral-100 dark:bg-neutral-900 w-72 hidden md:block">Left Side</div>
	<div class="flex flex-col w-full px-5 py-4">
		<Header />
		<div class="flex flex-col basis-[80%] justify-center items-center space-y-7">
			<div class="text-3xl font-bold">Good Morning, Harrison.</div>
			<Textarea class="max-w-176 md:w-[80%]"
								placeholder="What can I help you plan today?" />

			<!--			bg-neutral-700 text-neutral-300 border-neutral-600 -->
		</div>
	</div>
</div>