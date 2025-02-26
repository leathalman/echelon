<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createCompletion, createMessage, fetchMessages } from '$lib/api/client';
	import type { Message } from '$lib/api/messages';

	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let messages = $state<Message[]>([]);
	let query = $state('');

	let loading = $state(false);

	const conversationId = $derived(parseInt(page.params.conversation_id));
	let pollInterval: number;

	// TODO: only poll if there is a current completion request being processed...
	function pollForCompletion() {
		if (pollInterval) clearInterval(pollInterval);

		pollInterval = setInterval(async () => {
			try {
				const latestMessages: Message[] = await fetchMessages(conversationId);

				if (latestMessages.some(m => m.role === 'Assistant')) {
					messages = latestMessages;
					clearInterval(pollInterval);
				}
			} catch (error) {
				console.error('Error polling for messages:', error);
			}
		}, 1000) as unknown as number;

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

	async function handleSubmitQuery() {
		loading = true;
		if (!query.trim()) return;

		try {
			const userMessage: Message = {
				content: query,
				role: 'User',
			};

			const currentQuestion = query;
			query = '';

			messages = [...messages, userMessage];

			await createMessage(conversationId, currentQuestion, 'User');

			const completion = await createCompletion(messages);

			const assistantMessage: Message = {
				content: completion,
				role: 'Assistant',
			};

			loading = false;

			messages = [...messages, assistantMessage];

			await createMessage(conversationId, completion, 'Assistant');
		} catch (error) {
			console.error('Error sending message:', error);
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
