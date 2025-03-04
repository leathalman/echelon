<script lang="ts">
	import TextareaPlain from '../../lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { createCompletion, createConversation, createMessage } from '$lib/api/client';
	import type { Message } from '$lib/api/messages';

	let query = $state('');
	let { data } = $props();

	async function handleSubmitQuery() {
		if (!query.trim()) return;

		try {
			const jwt = data.auth_token;
			const conversationId = await createConversation(jwt);

			const { refreshConversations } = await import('$lib/state/conversations.svelte');

			await refreshConversations(jwt);

			await createMessage(jwt, conversationId, query, 'User');

			let message: Message = {
				role: 'User',
				content: query
			};

			const messages: Message[] = [message];

			// Store the query and completion status in sessionStorage
			if (browser) {
				sessionStorage.setItem('initialMessage', query);
				sessionStorage.setItem('completionPending', 'true');
			}

			// Start the completion process but don't wait for it
			createCompletion(jwt, messages)
				.then(completion => {
					// Mark completion as finished in session storage
					if (browser) {
						sessionStorage.setItem('completionPending', 'false');
						sessionStorage.setItem('completionResult', completion);
					}
					return createMessage(jwt, conversationId, completion, 'Assistant');
				})
				.catch(error => {
					// Mark completion as failed
					if (browser) {
						sessionStorage.setItem('completionPending', 'false');
						sessionStorage.setItem('completionError', error.message);
					}
					console.error('Error in completion:', error);
				});

			// Navigate to conversation page
			await goto(`/chat/${conversationId}`);
		} catch (error) {
			console.error('Error in submitQuery:', error);
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			handleSubmitQuery();
		}
	}
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-5">
	<div class="w-[90%] md:max-w-156 items-start flex flex-col space-y-4">
		<img alt="Temp Icon" height="25" src="/temp_logo.svg" width="25" />
		<span class="text-lg">Good afternoon, Harrison.</span>
	</div>
	<div
		class="w-[90%] md:max-w-156 flex flex-col rounded-lg
					border border-border focus-within:outline
					focus-within:outline-ring focus-within:outline-2
					focus-within:outline-offset-2 bg-background
					shadow-lg
		">
		<TextareaPlain class="text-lg font-semibold mx-1 px-2 my-2"
									 placeholder="How can I help?" bind:value={query}
									 onkeydown={handleKeydown}></TextareaPlain>
		<div class="flex w-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9 rounded-full">
				<ArrowRight />
			</Button>
		</div>
	</div>
</div>