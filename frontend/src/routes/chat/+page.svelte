<script lang="ts">
	import TextareaPlain from '../../lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowUp from 'lucide-svelte/icons/arrow-up';
	import { goto } from '$app/navigation';
	import { type Message, messages } from '$lib/model/messages.svelte';
	import { conversations } from '$lib/model/conversations.svelte';
	import { createCompletion, createConversation, createMessage } from '$lib/api/client';
	import { newMessage } from '$lib/model/messages.svelte.js';

	let query = $state('');
	let { data } = $props();

	async function handleSubmitQuery() {
		if (!query.trim()) return; // don't submit empty queries

		const conversationId = await createConversation(data.authToken);

		// update frontend
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

				// create completion to get conversation title

				// update conversation with title

				return createMessage(data.authToken, conversationId, completion, 'Assistant');
			})
			.catch(error => {
				newMessage.completionPending = false;
				console.error('Error in completion:', error);
			});

		await goto(`/chat/${conversationId}`);
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

<div class="flex flex-col basis-[75%] justify-center items-center space-y-5 mt-10">
	<div class="w-[90%] max-w-[46rem] items-center flex flex-col mb-3">
		<span class="text-3xl font-semibold">Hello, {data.user.first_name}.</span>
	</div>
	<div
		class="w-[90%] md:max-w-[46rem] flex flex-col rounded-xl
      border-[0.5px] border-border focus-within:outline
      focus-within:outline-ring focus-within:outline-2
      focus-within:outline-offset-2 bg-white
      shadow-md shadow-gray-300
    ">
		<TextareaPlain
			bind:value={query}
			onkeydown={handleKeydown}
			class="text-md mt-4 ml-5 bg-white"
			placeholder="Ask Echelon (Press Enter to send, Shift+Enter for new line)"></TextareaPlain>
		<div class="flex w-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9 rounded-xl" onclick={handleSubmitQuery}>
				<ArrowUp />
			</Button>
		</div>
	</div>
</div>