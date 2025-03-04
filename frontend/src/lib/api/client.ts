import type { Message } from '$lib/api/messages';

export async function fetchMessages(jwt: string | undefined, conversationId: number) {
	try {
		const response = await fetch(
			`http://localhost:8000/api/conversations/${conversationId}/messages`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${jwt}`
				}
			}
		);
		const data = await response.json();
		return data.messages;
	} catch (error) {
		console.error(`Failed to fetch messages: ${error}`);
		return [];
	}
}

// only returns active conversations
export async function fetchConversations(jwt: string | undefined) {
	try {
		const response = await fetch('http://localhost:8000/api/conversations', {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			}
		});

		const data = await response.json();
		console.log(data);

		return data;
	} catch (error) {
		console.error('Error:', error);
		return null;
	}
}

export async function createConversation(jwt: string | undefined) {
	try {
		const response = await fetch('http://localhost:8000/api/conversations', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			},
			body: JSON.stringify({
				title: 'Untitled'
			})
		});

		const data = await response.json();
		return data.conversation_id;
	} catch (error) {
		console.error('Error:', error);
		return null;
	}
}

export async function createMessage(
	jwt: string | undefined,
	conversationId: number,
	content: string,
	role: string
) {
	try {
		const response = await fetch(
			`http://localhost:8000/api/conversations/${conversationId}/messages`,
			{
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${jwt}`
				},
				body: JSON.stringify({
					content: content,
					role: role
				})
			}
		);

		const data = await response.json();
		return data.conversation_id;
	} catch (error) {
		console.error('Error:', error);
		return null;
	}
}

export async function createCompletion(jwt: string | undefined, messages: Message[]) {
	try {
		const formattedMessages = messages.map((message) => ({
			content: message.content,
			role: message.role
		}));

		const response = await fetch(`http://localhost:8000/api/completions`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			},
			body: JSON.stringify({
				messages: formattedMessages
			})
		});

		const data = await response.json();
		return data.content;
	} catch (error) {
		console.error('Error:', error);
		return '';
	}
}
