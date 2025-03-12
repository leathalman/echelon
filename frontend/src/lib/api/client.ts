import type { Message } from '$lib/model/messages.svelte';
import { API_CONFIG } from './config';

export async function fetchUser(jwt: string) {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/users`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			}
		});
		const data = await response.json();
		return data.user;
	} catch (error) {
		console.error(`Failed to fetch user: ${error}`);
		return [];
	}
}

export async function fetchMessages(jwt: string, conversationId: number) {
	try {
		const response = await fetch(
			`${API_CONFIG.BASE_URL}/conversations/${conversationId}/messages`,
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

// only returns active conversationsSvelte
export async function fetchConversations(jwt: string) {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/conversations`, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			}
		});

		return await response.json();
	} catch (error) {
		console.error('Error:', error);
		return null;
	}
}

export async function createConversation(jwt: string) {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/conversations`, {
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
	jwt: string,
	conversationId: number,
	content: string,
	role: string
) {
	try {
		const response = await fetch(
			`${API_CONFIG.BASE_URL}/conversations/${conversationId}/messages`,
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

export async function createCompletion(jwt: string, messages: Message[], university: string) {
	try {
		const formattedMessages = messages.map((message) => ({
			content: message.content,
			role: message.role
		}));

		const response = await fetch(`${API_CONFIG.BASE_URL}/completions`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			},
			body: JSON.stringify({
				messages: formattedMessages,
				collection: university
			})
		});

		const data = await response.json();
		return data.content;
	} catch (error) {
		console.error('Error:', error);
		return '';
	}
}

export async function updateUser(jwt: string, studentId: string, firstName: string, lastName: string, university: string): Promise<{ success: boolean; error?: string }> {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/users`, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${jwt}`
			},
			body: JSON.stringify({
				student_id: studentId,
				first_name: firstName,
				last_name: lastName,
				university: university
			})
		});

		// Handle non-2xx responses properly
		if (!response.ok) {
			const errorData = await response.json(); // assuming the API provides error details
			return { success: false, error: errorData.message || 'User update failed' };
		}

		// If successful, return success
		await response.json();
		return { success: true };
	} catch (error) {
		// Catch any network or unexpected errors
		console.error('Error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'Unknown error occurred'
		};
	}
}