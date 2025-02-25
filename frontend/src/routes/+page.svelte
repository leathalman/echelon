<script lang="ts">
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { goto } from '$app/navigation';

	let query = $state('');

	async function createConversation() {
		try {
			const response = await fetch('http://localhost:8000/api/conversations', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					'title': 'Untitled'
				})
			});

			const data = await response.json();

			return data.conversation_id;
		} catch (error) {
			console.error('Error:', error);
			throw error;
		}
	}

	async function createMessage(conversationId: number, content: string, role: string) {
		try {
			const response = await fetch(`http://localhost:8000/api/conversations/${conversationId}/messages`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					'content': content,
					'role': role
				})
			});

			const data = await response.json();

			return data.conversation_id;
		} catch (error) {
			console.error('Error:', error);
			throw error;
		}
	}

	async function createCompletion(conversationId: number) {
		try {
			const response = await fetch(`http://localhost:8000/api/completions`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					'messages': [
						{
							'role': 'User',
							'content': query
						}
					]
				})
			});

			const data = await response.json();
			let res = await createMessage(conversationId, data.content, 'Assistant');
			console.log(res);
		} catch (error) {
			console.error('Error:', error);
			throw error;
		}
	}

	async function submitQuery() {
		let conversationId = await createConversation();
		console.log(conversationId);

		let messageId = await createMessage(conversationId, query, 'User');
		console.log(messageId);

		await goto(`/chat/${conversationId}`);

		await createCompletion(conversationId);
	}
</script>

<div class="flex flex-col basis-[75%] justify-center items-center space-y-5">
	<div class="w-[90%] md:max-w-156 items-start flex flex-col space-y-4">
		<img alt="Temp Icon" height="25" src="/temp_logo.svg" width="25" />
		<span class="text-lg">Good Morning, Harrison.</span>
	</div>
	<div
		class="w-[90%] md:max-w-156 flex flex-col rounded-lg
					shadow-lg border border-black">
		<TextareaPlain class="text-lg font-semibold mx-1 px-2 my-2"
									 placeholder="How can I help?" bind:value={query}></TextareaPlain>
		<div class="flex w-full justify-end items-end py-2 px-2">
			<Button class="w-9 h-9" onclick={submitQuery}>
				<ArrowRight />
			</Button>
		</div>
	</div>
</div>