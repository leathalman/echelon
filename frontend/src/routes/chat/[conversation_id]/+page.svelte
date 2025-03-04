<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createCompletion, createMessage, fetchMessages } from '$lib/api/client';
	import type { Message } from '$lib/model/messages';
	import { newChatState } from '$lib/state/new-chat.svelte.js';
	import { refreshConversations } from '$lib/state/conversations.svelte';
	import { refreshUser } from '$lib/state/user.svelte';

	let { data } = $props();

	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let messages: Message[] = $state([]);

	let query = $state('');

	let loading = $state(false);

	const conversationId = $derived(parseInt(page.params.conversation_id));
	let pollInterval: number;

	function pollForCompletion() {
		if (pollInterval) clearInterval(pollInterval);

		const isPending = newChatState.completionPending === true;
		if (!isPending) return;

		loading = true;

		pollInterval = setInterval(() => {

			try {
				if (newChatState.completionPending === false) {
					const completionResult = newChatState.completionResult;
					if (completionResult) {
						const assistantMessage: Message = {
							content: completionResult,
							role: 'Assistant'
						};
						messages = [...messages, assistantMessage];
					}

					const completionError = newChatState.completionError;
					if (completionError) {
						console.error('Completion error:', completionError);
					}

					clearInterval(pollInterval);
					loading = false;
				}
			} catch (error) {
				console.error('Error polling for completion status:', error);
			}
		}, 500) as unknown as number;

		// Set timeout to stop polling after 15 seconds
		setTimeout(() => {
			if (pollInterval) {
				clearInterval(pollInterval);
				loading = false;
			}
		}, 15000);
	}

	// TODO: lots of extra fetches here, user should be passed down... not fetched
	async function initializeChat() {
		await refreshUser(data.jwt)
		if (conversationId) {
			await refreshConversations(data.jwt)

			if (newChatState.completionPending) {
				const userMessage: Message = {
					content: newChatState.initialMessage,
					role: 'User'
				};
				messages = [userMessage];
			} else {
				await loadMessages(conversationId);
			}

			pollForCompletion();
		}
	}

	$effect(() => {
		if (conversationId) {
			initializeChat();
		}
	});

	async function loadMessages(id: number) {
		if (!id || isNaN(id)) return;
		try {
			messages = await fetchMessages(data.jwt, id);

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
				role: 'User'
			};

			const currentQuestion = query;
			query = '';

			messages = [...messages, userMessage];

			await createMessage(data.jwt, conversationId, currentQuestion, 'User');

			const completion = await createCompletion(data.jwt, messages);

			const assistantMessage: Message = {
				content: completion,
				role: 'Assistant'
			};

			messages = [...messages, assistantMessage];

			await createMessage(data.jwt, conversationId, completion, 'Assistant');
			await refreshConversations(data.jwt);
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
	<div bind:clientWidth={markdownWidth} class="flex flex-col w-[90%] md:max-w-156 space-y-8"
			 style="margin-bottom: {textAreaHeight + 80}px">
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
	<div class="fixed bottom-0 pb-6 bg-background" style="width: {markdownWidth}px">
		<div class="flex flex-row bg-background rounded-lg shadow-lg border justify-between">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					bind:value={query}
					class="w-full font-medium"
					onkeydown={handleKeydown}
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