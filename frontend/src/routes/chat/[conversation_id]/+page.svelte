<script lang="ts">
	import { page } from '$app/state';
	import { createStreamingCompletion, createMessage, createTitle, updateConversation } from '$lib/api/client';
	import { type Message, messages, newMessage } from '$lib/model/messages.svelte';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { marked } from 'marked';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { conversations } from '$lib/model/conversations.svelte';
	import { onMount } from 'svelte';

	let { data } = $props();
	let query = $state('');
	let streamingResponse = $state('');
	let isStreaming = $state(false);
	let textAreaHeight = $state(25);
	let markdownWidth = $state();

	const conversationId = $derived(parseInt(page.params.conversation_id));

	async function processStream(response: Response) {
		// Make sure we're getting a text event stream
		if (!response.body) {
			throw new Error('No response body');
		}

		// Set up SSE processing
		const reader = response.body.getReader();
		const decoder = new TextDecoder();

		try {
			while (true) {
				const { done, value } = await reader.read();

				if (done) {
					break;
				}

				// Decode the chunk (which is a Uint8Array)
				const text = decoder.decode(value, { stream: true });

				// Process the SSE format
				// SSE data comes in the format: "data: message\n\n"
				const lines = text.split('\n');

				for (const line of lines) {
					if (line.startsWith('data: ')) {
						// Extract just the message part
						const message = line.substring(6); // "data: ".length === 6

						// Check for special event types
						if (message && message !== '[DONE]') {
							streamingResponse += message;
						}
					} else if (line.startsWith('event: error')) {
						console.error('Stream error:', lines);
					}
				}
			}
		} finally {
			reader.releaseLock();
		}
	}

	async function handleSubmitQuery() {
		if (!query.trim()) return; // Don't submit empty queries

		try {
			isStreaming = true;
			streamingResponse = ''; // Reset the streaming response

			const userMessage: Message = {
				content: query,
				role: 'User'
			};

			// Update UI first
			messages.value.push(userMessage);

			newMessage.completionPending = true;
			await createMessage(data.authToken, conversationId, userMessage.content, userMessage.role);

			// Clear input
			query = '';

			// Start streaming
			const response = await createStreamingCompletion(
				data.authToken,
				messages.value,
				data.user.university
			);

			newMessage.completionPending = false;

			// Process the SSE stream
			await processStream(response);

			// Once streaming is complete, save the full response
			const assistantMessage: Message = {
				content: streamingResponse,
				role: 'Assistant'
			};

			isStreaming = false;
			messages.value.push(assistantMessage);

			// Save to backend
			await createMessage(data.authToken, conversationId, streamingResponse, 'Assistant');
		} catch (error) {
			console.error('Error in streaming:', error);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSubmitQuery();
		}
	}

	async function handleTitleCreation() {
		let conversation = conversations.value.find(conversation => conversation.id === conversationId);

		if (conversation) {
			if (conversation.title === "Untitled") {
				let title = await createTitle(data.authToken, messages.value);
				if (title !== "") {
					conversation.title = title
					await updateConversation(data.authToken, conversationId, title);
				}

				await updateConversation(data.authToken, conversationId, title);
			}
		}
	}

	onMount(async () => {
		await handleTitleCreation()
	});

	$effect(() => {
		// save messages into rune for use when updating ui directly
		messages.value = data.messages;
	});
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} class="flex flex-col w-[90%] md:max-w-156 space-y-8"
			 style="margin-bottom: {textAreaHeight + 80}px">
		{#each messages.value as message}
			{#if message.role === 'User'}
				<div class="flex w-full justify-end">
					<span class="text-md bg-violet-200 rounded-lg p-3">{message.content}</span>
				</div>
			{:else}
				<div class="prose">
					{@html marked(message.content)}
				</div>
			{/if}
		{/each}

		{#if newMessage.completionPending}
			<div class="space-y-2">
				<div class="flex flex-row">
					<Skeleton class="h-4 w-4/6" />
					<Skeleton class="h-4 w-1/6 ml-3" />
				</div>
				<Skeleton class="h-4 w-3/4" />
				<div class="flex flex-row">
					<Skeleton class="h-4 w-1/4" />
					<Skeleton class="h-4 w-1/2 ml-3" />
				</div>
			</div>
		{/if}

		{#if isStreaming}
			<div class="prose">
				{streamingResponse}
			</div>
		{/if}
	</div>
	<div class="fixed bottom-0 pb-6 bg-background" style="width: {markdownWidth}px">
		<div class="flex flex-row rounded-lg shadow-lg border justify-between bg-white">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					bind:value={query}
					class="w-full font-medium bg-white"
					disabled={isStreaming}
					onkeydown={handleKeydown}
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button
					class="w-8 h-8 my-2 mx-2"
					disabled={isStreaming}
					onclick={handleSubmitQuery}>
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>