<script lang="ts">
	import Header from '$lib/components/app-header.svelte';
	import { Textarea } from '$lib/components/ui/textarea';
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-7">
	<div class="text-3xl font-bold">Good Morning, Harrison.</div>
	<Textarea class="max-w-176 sm:w-[80%]"
						placeholder="What can I help you plan today?" />
</div>