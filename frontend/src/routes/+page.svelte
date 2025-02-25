<script lang="ts">
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { goto } from '$app/navigation';
	import { createCompletion, createConversation, createMessage } from '$lib/api/client';

	let query = $state('');

	async function submitQuery() {
		if (!query.trim()) return;

		try {
			// Create a new conversation with default title
			const conversationId = await createConversation();
			console.log('Created conversation:', conversationId);

			// Save the user's query as a message in the conversation
			const result = await createMessage(conversationId, query, 'User');
			console.log('Created message:', result);

			// Start the completion request but don't wait for it to finish
			createCompletion(query)
				.then(completion => {
					// Save the assistant's response
					return createMessage(conversationId, completion, 'Assistant');
				})
				.then(result => {
					console.log('Assistant message created:', result);
				})
				.catch(error => {
					console.error('Error in completion:', error);
				});

			// Navigate to the chat page immediately
			await goto(`/chat/${conversationId}`);
		} catch (error) {
			console.error('Error in submitQuery:', error);
			// You might want to show an error notification here
		}
	}

	// Handle Enter key to submit
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			submitQuery();
		}
	}
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-5">
	<div class="w-[90%] md:max-w-156 items-start flex flex-col space-y-4">
		<img alt="Temp Icon" height="25" src="/temp_logo.svg" width="25" />
		<span class="text-lg">Good Morning, Harrison.</span>
	</div>
	<div
		class="w-[90%] md:max-w-156 flex flex-col rounded-lg
					shadow-lg border border-black">
		<TextareaPlain class="text-lg font-semibold mx-1 px-2 my-2"
									 placeholder="How can I help?" bind:value={query}
									 onkeydown={handleKeydown}></TextareaPlain>
		<div class="flex w-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9" onclick={submitQuery}>
				<ArrowRight />
			</Button>
		</div>
	</div>
</div>