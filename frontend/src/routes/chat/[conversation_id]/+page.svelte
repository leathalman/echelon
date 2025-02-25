<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createCompletion, createMessage, fetchMessages } from '$lib/api/client';
	import type { Message } from '$lib/api/messages';

	// state management
	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let messages = $state<Message[]>([]);
	let query = $state('');

	const conversationId = $derived(parseInt(page.params.conversation_id));
	let pollInterval: number;

	function pollForCompletion() {
		// Clear any existing poll interval
		if (pollInterval) clearInterval(pollInterval);

		// Check every 2 seconds for new messages
		pollInterval = setInterval(async () => {
			try {
				const latestMessages: Message[] = await fetchMessages(conversationId);

				// Check if we have a new assistant message
				if (latestMessages.some(m => m.role === 'Assistant')) {
					// We got a response, update the messages and stop polling
					messages = latestMessages;
					clearInterval(pollInterval);
				}
			} catch (error) {
				console.error('Error polling for messages:', error);
			}
		}, 1000) as unknown as number;

		// Stop polling after 30 seconds to prevent endless polling
		setTimeout(() => {
			if (pollInterval) {
				clearInterval(pollInterval);
			}
		}, 30000);
	}

	$effect(() => {
		loadMessages(conversationId);
		pollForCompletion();
	});

	async function loadMessages(id: number) {
		if (!id || isNaN(id)) return;
		try {
			messages = await fetchMessages(id);
		} catch (err) {
			console.error(err);
		}
	}

	async function handleSendMessage() {
		if (!query.trim()) return;

		try {
			// Save the user's query to state first for immediate display
			const userMessage: Message = {
				conversation_id: conversationId,
				content: query,
				role: 'User',
			};

			// Store current query and clear input
			const currentQuestion = query;
			query = '';

			// Add user message to UI
			messages = [...messages, userMessage];

			// Save user message to backend
			await createMessage(conversationId, currentQuestion, 'User');

			// Get AI response
			const completion = await createCompletion(currentQuestion);

			// Add AI response to UI
			const assistantMessage: Message = {
				conversation_id: conversationId,
				content: completion,
				role: 'Assistant',
			};

			messages = [...messages, assistantMessage];

			// Save assistant message to backend
			await createMessage(conversationId, completion, 'Assistant');
		} catch (error) {
			console.error('Error sending message:', error);
			// You could add error handling UI here
		}
	}

	// Handle Enter key to submit
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSendMessage();
		}
	}
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} style="margin-bottom: {textAreaHeight + 80}px"
			 class="flex flex-col w-[90%] md:max-w-156 space-y-8">
		{#each messages as message}
				{#if message.role === 'User'}
					<div class="flex w-full justify-end">
						<span class="text-md bg-violet-100 rounded-lg p-3">{message.content}</span>
					</div>
				{:else}
					<div class="prose">
						{@html marked(message.content)}
					</div>
				{/if}
		{/each}
	</div>
	<div style="width: {markdownWidth}px" class="fixed bottom-0 pb-6 bg-background">
		<div class="flex flex-row bg-background rounded-lg shadow-lg border justify-between">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					bind:value={query}
					onkeydown={handleKeydown}
					class="w-full font-medium"
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button class="w-8 h-8 my-2 mx-2" onclick={handleSendMessage}>
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>
