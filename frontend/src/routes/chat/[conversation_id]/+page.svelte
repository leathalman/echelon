<script lang="ts">
	import { page } from '$app/state';
	import { createMessage, createStreamingCompletion, createTitle, updateConversation } from '$lib/api/client';
	import { type Message, newMessage } from '$lib/model/messages.svelte';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import { marked } from 'marked';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { conversations } from '$lib/model/conversations.svelte';
	import { afterNavigate } from '$app/navigation';
	import { createParser } from 'eventsource-parser';
	import { fade } from 'svelte/transition';
	import { onMount } from 'svelte';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';

	let { data } = $props();
	let query = $state('');
	let textAreaHeight = $state(60);
	let markdownWidth = $state();
	let chatContainer = $state<HTMLDivElement | null>(null);
	let inputContainerHeight = $state(0);
	let inputContainer = $state<HTMLDivElement | null>(null);

	let isStreaming = $state(false);
	let isAwaitingStream = $state(false);

	let streamingResponse = $state('');
	let currentMessages = $state({
		value: [] as Message[]
	});

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
			currentMessages.value, // Use local messages state
			data.user.university
		);


		if (!response.body) {
			throw new Error('No response body');
		}

		isAwaitingStream = false;
		isStreaming = true;
		streamingResponse = '';

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

		// Update both state arrays
		// currentMessages = [...currentMessages, assistantMessage];
		currentMessages.value.push(assistantMessage)

		scrollToBottom(); // Ensure we're scrolled to bottom when streaming completes

		isStreaming = false;

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

			// Update local state first (create a new array)
			// currentMessages = [...currentMessages, userMessage];
			currentMessages.value.push(userMessage)

			scrollToBottom(); // Scroll after adding user message

			isAwaitingStream = true;
			await createMessage(data.authToken, conversationId, userMessage.content, userMessage.role);

			// Clear input
			query = '';

			await handleStream();
		} catch (error) {
			console.error('Error in streaming:', error);
			isAwaitingStream = false;
			isStreaming = false;
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
			if (conversation.title === 'Untitled') {
				let title = await createTitle(data.authToken, currentMessages.value);
				if (title !== '') {
					conversation.title = title;
					await updateConversation(data.authToken, conversationId, title);
				}
			}
		}
	}

	// Initialize the local message state when data.messages changes
	$effect(() => {
		if (streamingResponse) {
			scrollToBottom();
		}

		if (inputContainer) {
			inputContainerHeight = inputContainer.offsetHeight;
		}

		$inspect(currentMessages)
	});

	onMount(() => {
		if (data.messages && (data.messages.length > currentMessages.value.length)) {
			currentMessages.value = [...data.messages];
		}

		scrollToBottom();
	});

	afterNavigate(async ({ from }) => {
		let previousPage = from?.url.pathname || '/';

		if (previousPage === '/chat' && newMessage.shouldStartCompletion) {
			console.log('Came from /chat and shouldStartCompletion');
			newMessage.shouldStartCompletion = false;
			isAwaitingStream = true
			await handleStream();
			await handleTitleCreation();
		}
		scrollToBottom();
	});
</script>

<div class="flex flex-col items-center pt-24" in:fade={{ duration: 200 }}>
	<div bind:clientWidth={markdownWidth} bind:this={chatContainer} class="flex flex-col w-[90%] md:max-w-156 space-y-8"
			 style="margin-bottom: {inputContainerHeight + 80}px" >
		{#each currentMessages.value as message}
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
		{#if isAwaitingStream}
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
				{@html marked(streamingResponse)}
			</div>
		{/if}
	</div>
	<div class="fixed bottom-0 pb-8 bg-background" style="width: {markdownWidth}px">
		<div
			bind:this={inputContainer}
			class="flex flex-col rounded-xl
      border-[0.5px] border-border focus-within:outline
      focus-within:outline-ring focus-within:outline-2
      focus-within:outline-offset-2 bg-white
      shadow-md shadow-gray-300"
		>
			<TextareaPlain
				bind:height={textAreaHeight}
				bind:value={query}
				class="text-md mt-4 ml-5 bg-white"
				disabled={isStreaming || newMessage.shouldStartCompletion || isAwaitingStream}
				onkeydown={handleKeydown}
				placeholder="Ask Echelon"></TextareaPlain>
			<div class="flex w-full justify-end items-end py-2 px-2">
				<Button class="w-9 h-9 rounded-xl" disabled={isStreaming || newMessage.shouldStartCompletion || isAwaitingStream} onclick={handleSubmitQuery}>
					<ArrowUp />
				</Button>
			</div>
		</div>
	</div>
</div>