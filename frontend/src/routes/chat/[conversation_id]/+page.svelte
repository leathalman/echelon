<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createCompletion, createMessage, fetchMessages } from '$lib/api/client';
	import type { Message } from '$lib/api/messages';
	import { browser } from '$app/environment';

	let { data } = $props()

	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let messages = $state<Message[]>([]);
	let query = $state('');

	let loading = $state(false);

	const conversationId = $derived(parseInt(page.params.conversation_id));
	let pollInterval: number;

	// Poll storage for completion status if pending
	function pollForCompletion() {
		if (pollInterval) clearInterval(pollInterval);
		
		if (!browser) return;

		// Check if completion is pending
		const isPending = sessionStorage.getItem('completionPending') === 'true';
		if (!isPending) return;

		loading = true;
		
		pollInterval = setInterval(() => {
			if (!browser) return;
			
			try {
				// Check if completion finished
				if (sessionStorage.getItem('completionPending') === 'false') {
					// Get completion result
					const completionResult = sessionStorage.getItem('completionResult');
					if (completionResult) {
						const assistantMessage: Message = {
							content: completionResult,
							role: 'Assistant',
						};
						messages = [...messages, assistantMessage];
						
						// Clear storage values
						sessionStorage.removeItem('completionResult');
					}
					
					// Check for error
					const completionError = sessionStorage.getItem('completionError');
					if (completionError) {
						console.error('Completion error:', completionError);
						sessionStorage.removeItem('completionError');
					}
					
					// Clear pending flag and stop polling
					sessionStorage.removeItem('completionPending');
					clearInterval(pollInterval);
					loading = false;
				}
			} catch (error) {
				console.error('Error polling for completion status:', error);
			}
		}, 500) as unknown as number;

		// Set timeout to stop polling after 30 seconds
		setTimeout(() => {
			if (pollInterval) {
				clearInterval(pollInterval);
				loading = false;
			}
		}, 30000);
	}

	$effect(() => {
		if (conversationId && browser) {
			// Try to load initial message from sessionStorage
			const initialMessage = sessionStorage.getItem('initialMessage');
			if (initialMessage) {
				const userMessage: Message = {
					content: initialMessage,
					role: 'User',
				};
				messages = [userMessage];
				sessionStorage.removeItem('initialMessage');
			} else {
				// Load existing messages if there's no initial message
				loadMessages(conversationId);
			}
			
			// Check if we should start polling for completion
			pollForCompletion();
		}
	});

	async function loadMessages(id: number) {
		if (!id || isNaN(id)) return;
		try {
			const fetchedMessages = await fetchMessages(data.auth_token, id);

			if (messages.length === 0) {
				messages = fetchedMessages;
			}
		} catch (err) {
			console.error('Error fetching messages:', err);
		}
	}

	async function handleSubmitQuery() {
		if (!query.trim()) return;
		loading = true;

		try {
			const userMessage: Message = {
				content: query,
				role: 'User',
			};

			const currentQuestion = query;
			query = '';

			messages = [...messages, userMessage];

			await createMessage(data.auth_token, conversationId, currentQuestion, 'User');

			// Add all current messages for context
			const completion = await createCompletion(data.auth_token, messages);

			const assistantMessage: Message = {
				content: completion,
				role: 'Assistant',
			};

			messages = [...messages, assistantMessage];

			await createMessage(data.auth_token, conversationId, completion, 'Assistant');
		} catch (error) {
			console.error('Error sending message:', error);
		} finally {
			loading = false;
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSubmitQuery();
		}
	}
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} style="margin-bottom: {textAreaHeight + 80}px"
			 class="flex flex-col w-[90%] md:max-w-156 space-y-8">
		{#each messages as message}
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
		{#if loading}
			<span class="text-md">Thinking...</span>
		{/if}
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
				<Button class="w-8 h-8 my-2 mx-2" onclick={handleSubmitQuery}>
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>