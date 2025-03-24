import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { API_CONFIG } from '$lib/api/config';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	const { authToken } = await parent();
	const conversationId = params.conversation_id; // Get conversationId from URL params

	try {
		const response = await fetch(
			`${API_CONFIG.BASE_URL}/conversations/${conversationId}/messages`,
			{
				method: 'GET',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${authToken}`
				}
			}
		);
		const data = await response.json();

		return {
			messages: data.messages
		};
	} catch (err) {
		console.error('Error fetching messages:', err);
		throw error(500, 'Failed to fetch messages');
	}
};
