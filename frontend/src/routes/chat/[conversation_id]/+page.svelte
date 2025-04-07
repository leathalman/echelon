<script lang="ts">
	// Add other imports from your existing file
	import { page } from '$app/state';
	import { createMessage, createStreamingCompletion, createTitle, updateConversation } from '$lib/api/client';
	import { type Message, messages, newMessage } from '$lib/model/messages.svelte';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { marked } from 'marked';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { conversations } from '$lib/model/conversations.svelte';
	import { afterNavigate } from '$app/navigation';
	import { createParser } from 'eventsource-parser';
	import { onMount } from 'svelte';

	let { data } = $props();
	let query = $state('');
	let streamingResponse = $state('');
	let textAreaHeight = $state(25);
	let markdownWidth = $state();
	let chatContainer = $state<HTMLDivElement | null>(null);

	const conversationId = $derived(parseInt(page.params.conversation_id));

	// Function to scroll to the bottom of the chat container
	function scrollToBottom() {
		if (chatContainer) {
			// Use setTimeout to ensure this happens after the DOM updates
			setTimeout(() => {
				window.scrollTo({
					top: document.body.scrollHeight,
					behavior: 'smooth'
				});
			}, 100);
		}
	}

	function createSSEParser(onMessage: (message: string) => void, onError?: (error: string) => void) {
		return createParser({
			onEvent(event) {
				if (event.event === 'error') {
					onError?.(event.data);
					return;
				}
				if (event.event === 'message') {
					onMessage(event.data);
					return;
				}
				onMessage(event.data);
			},
			onError(err) {
				onError?.(err.message);
			}
		});
	}

	async function handleStream() {
		const response = await createStreamingCompletion(
			data.authToken,
			messages.value,
			data.user.university
		);

		newMessage.isAwaitingStream = false;
		newMessage.isStreaming = true;
		streamingResponse = '';

		if (!response.body) {
			throw new Error('No response body');
		}

		const reader = response.body.getReader();
		const decoder = new TextDecoder();

		const parser = createSSEParser(
			(message) => {
				if (message && message !== '[DONE]') {
					streamingResponse += message;
					scrollToBottom(); // Scroll to bottom on each chunk
				}
			},
			(error) => {
				console.error('Stream error:', error);
			}
		);

		try {
			while (true) {
				const { done, value } = await reader.read();

				if (done) {
					break;
				}

				const chunk = decoder.decode(value, { stream: true });
				parser.feed(chunk);
			}
		} finally {
			reader.releaseLock();
		}

		const assistantMessage: Message = {
			content: streamingResponse,
			role: 'Assistant'
		};

		messages.value.push(assistantMessage);
		scrollToBottom(); // Ensure we're scrolled to bottom when streaming completes

		newMessage.isStreaming = false;

		await createMessage(data.authToken, conversationId, streamingResponse, 'Assistant');
	}

	async function handleSubmitQuery() {
		if (!query.trim()) return; // Don't submit empty queries

		try {
			streamingResponse = ''; // Reset the streaming response

			const userMessage: Message = {
				content: query,
				role: 'User'
			};

			// Update UI first
			messages.value.push(userMessage);
			scrollToBottom(); // Scroll after adding user message

			newMessage.isAwaitingStream = true;
			await createMessage(data.authToken, conversationId, userMessage.content, userMessage.role);

			// Clear input
			query = '';

			await handleStream();
		} catch (error) {
			console.error('Error in streaming:', error);
			newMessage.isAwaitingStream = false;
			newMessage.isStreaming = false;
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
					conversation.title = title;
					await updateConversation(data.authToken, conversationId, title);
				}
			}
		}
	}

	$effect(() => {
		// save messages into rune for use when updating ui directly
		messages.value = data.messages;
		// Scroll to bottom when messages change
		scrollToBottom();
	});

	// Also scroll when streaming response changes
	$effect(() => {
		if (streamingResponse) {
			scrollToBottom();
		}
	});

	// Scroll to bottom when page loads
	onMount(() => {
		scrollToBottom();
	});

	afterNavigate(async ({ from }) => {
		let previousPage = from?.url.pathname || '/';

		if (previousPage === "/chat" && newMessage.shouldStartCompletion) {
			console.log("Came from /chat and shouldStartCompletion");
			await handleStream();
			await handleTitleCreation();
		}

		// Ensure we're scrolled to bottom after navigation
		scrollToBottom();
	});
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} bind:this={chatContainer} class="flex flex-col w-[90%] md:max-w-156 space-y-8"
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
		{#if newMessage.isAwaitingStream}
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
		{#if newMessage.isStreaming}
			<div class="prose">
				{@html marked(streamingResponse)}
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
					disabled={newMessage.isStreaming}
					onkeydown={handleKeydown}
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button
					class="w-8 h-8 my-2 mx-2"
					disabled={newMessage.isStreaming}
					onclick={handleSubmitQuery}>
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>