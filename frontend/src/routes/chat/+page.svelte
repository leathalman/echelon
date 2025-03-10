<script lang="ts">
	import TextareaPlain from '../../lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { goto } from '$app/navigation';
	import { type Message, messages } from '$lib/model/messages.svelte';
	import { conversations } from '$lib/model/conversations.svelte';
	import { createCompletion, createConversation, createMessage } from '$lib/api/client';
	import { newMessage } from '$lib/model/messages.svelte.js';

	let query = $state('');
	let { data } = $props();

	async function handleSubmitQuery() {
		const conversationId = await createConversation(data.authToken);

		conversations.value.unshift({
			id: conversationId,
			last_message_at: '',
			owner_id: data.user.id,
			status: 'Active',
			title: 'Untitled'
		});

		await createMessage(data.authToken, conversationId, query, 'User');

		newMessage.completionPending = true;
		newMessage.content = query;

		createCompletion(data.authToken, [{
			role: 'User',
			content: query
		}] as Message[], data.user.university)
			.then(completion => {
				newMessage.completionPending = false;
				newMessage.completion = completion;

				messages.value.push({
					role: 'Assistant',
					content: completion
				});

				return createMessage(data.authToken, conversationId, completion, 'Assistant');
			})
			.catch(error => {
				newMessage.completionPending = false;
				console.error('Error in completion:', error);
			});

		await goto(`/chat/${conversationId}`);
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
		<TextareaPlain bind:value={query}
									 class="text-lg font-semibold mx-1 px-2 my-2"
									 placeholder="How can I help?"></TextareaPlain>
		<div class="flex w-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9 rounded-full" onclick={handleSubmitQuery}>
				<ArrowRight />
			</Button>
		</div>
	</div>
</div>