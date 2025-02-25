<script lang="ts">
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';
	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { goto } from '$app/navigation';
	import { createConversation, createMessage } from '$lib/api/client';

	let query = $state('');

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