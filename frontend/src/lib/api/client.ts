import ConversationsResponse from '$lib/api/conversations';

export async function fetchMessages(conversationId: number) {
	try {
		const response = await fetch(
			`http://localhost:8000/api/conversations/${conversationId}/messages`
		);
		const data = await response.json();
		return data.messages;
	} catch (error) {
		console.error(`Failed to fetch messages: ${error}`);
		return [];
	}
}

// only returns active conversations
export async function fetchConversations() {
	try {
		const response = await fetch('http://localhost:8000/api/conversations');
		const data = await response.json();
		const conversationsResponse = new ConversationsResponse(data);
		conversationsResponse.conversations = conversationsResponse.getActiveConversations();
		return conversationsResponse.getConversationsSortedByDate();
	} catch (error) {
		console.error(`Failed to fetch conversations: ${error}`);
		return [];
	}
}

export async function createConversation() {
	try {
		const response = await fetch('http://localhost:8000/api/conversations', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
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

export async function createMessage(conversationId: number, content: string, role: string) {
	try {
		const response = await fetch(
			`http://localhost:8000/api/conversations/${conversationId}/messages`,
			{
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
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

export async function createCompletion(conversationId: number, query: string) {
	try {
		const response = await fetch(`http://localhost:8000/api/completions`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				messages: [
					{
						role: 'User',
						content: query
					}
				]
			})
		});

		const data = await response.json();
		
		return data.content;
	} catch (error) {
		console.error('Error:', error);
		return '';
	}
}
