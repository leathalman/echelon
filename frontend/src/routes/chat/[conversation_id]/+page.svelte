<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import ConversationsResponse from '$lib/api/conversations';
	import { onMount } from 'svelte';


	const { data } = $props();
	let conversation_id = data.chat_id;

	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	async function fetchMessages() {
	try {
	const response = await fetch(`http://localhost:8000/api/conversations/${conversation_id}/messages`);
	const data = await response.json();
	console.log(data)
} catch (err) {
	console.error(`Failed to fetch conversations: ${err}`);
}
}

onMount(() => {
	fetchMessages()
})
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} style="margin-bottom: {textAreaHeight + 80}px"
			 class="flex flex-col w-[90%] md:max-w-156 space-y-2">
		<span class="text-sm text-gray-600">what are the objectives of the holiday party?</span>
		<div class="prose prose-sm">
			<!--{@html marked(contents)}-->
		</div>
	</div>
	<div style="width: {markdownWidth}px" class="fixed bottom-0 pb-6 bg-background">
		<div class="flex flex-row bg-background rounded-lg shadow-lg border justify-between">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					class="w-full font-medium"
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button class="w-8 h-8 my-2 mx-2">
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>
