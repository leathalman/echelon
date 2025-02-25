<script lang="ts">
	import { marked } from 'marked';
	import TextareaPlain from '$lib/components/ui/textarea/textarea-plain.svelte';

	import { Button } from '$lib/components/ui/button';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { page } from '$app/state';
	import { createMessage, fetchMessages } from '$lib/api/client';


	type Message = {
		id: number;
		conversation_id: number;
		content: string;
		role: 'User' | 'Assistant';
		created_at: string;
	};

	// state management
	let markdownWidth = $state();
	let textAreaHeight = $state(25);

	let messages = $state<Message[]>([]);
	let question = $state('');
	let loading = $state(true);
	let error = $state('');

	const conversationId = $derived(parseInt(page.params.conversation_id));

	$effect(() => {
		loadMessages(conversationId);
	});

	async function loadMessages(id: number) {
		if (!id || isNaN(id)) return;

		loading = true;
		error = '';

		try {
			messages = await fetchMessages(id);
		} catch (err) {
			error = 'Failed to load messages';
			console.error(err);
		} finally {
			loading = false;
		}

		console.log(messages);
	}

	// TODO: update method to actually worffk
	async function handleSendMessage() {
		if (!question.trim()) return;

		// Optimistically add user message to UI
		const userMessage: Message = {
			id: Date.now(), // temporary ID
			conversation_id: conversationId,
			content: question,
			role: 'User',
			created_at: new Date().toISOString()
		};

		messages = [...messages, userMessage];
		const currentQuestion = question;
		question = '';

		// Send to API
		const result = await createMessage(conversationId, currentQuestion, 'User');

		if (!result.success) {
			await loadMessages(conversationId);
		}
	}
</script>

<div class="flex flex-col items-center pt-24">
	<div bind:clientWidth={markdownWidth} style="margin-bottom: {textAreaHeight + 80}px"
			 class="flex flex-col w-[90%] md:max-w-156 space-y-8">
		{#each messages as message}
				{#if message.role === 'User'}
					<div class="flex w-full justify-end">
						<span class="text-md bg-violet-100 rounded-lg p-3">{message.content}</span>
					</div>
				{:else}
					<div class="prose">
						{@html marked(message.content)}
					</div>
				{/if}
		{/each}
	</div>
	<div style="width: {markdownWidth}px" class="fixed bottom-0 pb-6 bg-background">
		<div class="flex flex-row bg-background rounded-lg shadow-lg border justify-between">
			<div class="flex items-center mx-1 px-2 my-2 w-[90%]">
				<TextareaPlain
					bind:height={textAreaHeight}
					class="w-full font-medium"
					placeholder="What else would you like to know?" />
			</div>
			<div class="flex items-end">
				<Button class="w-8 h-8 my-2 mx-2">
					<ArrowRight />
				</Button>
			</div>
		</div>
	</div>
</div>
