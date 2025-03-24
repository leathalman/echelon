<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createCompletion, createMessage, fetchMessages } from '$lib/api/client';
	import { type Message, messages } from '$lib/model/messages.svelte';
	import { newMessage } from '$lib/model/messages.svelte.js';

	let { data } = $props();

	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let query = $state('');

	const conversationId = $derived(parseInt(page.params.conversation_id));

	$effect(() => {
		// save messages into rune for use when updating ui directly
		messages.value = data.messages;
	});

	async function handleSubmitQuery() {
		if (!query.trim()) return; // Don't submit empty queries

		newMessage.completionPending = true

		try {
			const userMessage: Message = {
				content: query,
				role: 'User',
			}

			// reset UI textbox
			query = ''

			// update UI
			messages.value.push(userMessage)

			await createMessage(data.authToken, conversationId, userMessage.content, userMessage.role);

			const completion = await createCompletion(data.authToken, messages.value, data.user.university);

			const assistantMessage: Message = {
				content: completion,
				role: 'Assistant'
			}

			// update UI
			messages.value.push(assistantMessage)

			await createMessage(data.authToken, conversationId, completion, 'Assistant');
		} catch (error) {
			console.error('Error sending message:', error)
		} finally {
			newMessage.completionPending = false
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (!event.shiftKey) {
				// Enter without shift - submit
				event.preventDefault(); // Prevent default newline
				handleSubmitQuery();
			}
			// With shift - let the default behavior (newline) happen
		}
	}
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
			<span class="text-md">Thinking...</span>
		{/if}
	</div>
	<div class="fixed bottom-0 pb-6 bg-background" style="width: {markdownWidth}px">
		<div class="flex flex-row rounded-lg shadow-lg border justify-between bg-white">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					bind:value={query}
					onkeydown={handleKeydown}
					class="w-full font-medium bg-white"
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