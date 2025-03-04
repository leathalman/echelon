import { fetchConversations } from '$lib/api/client';
import type { Conversation } from '$lib/api/conversations';

export const conversations = $state<Conversation[]>([]);

export async function refreshConversations(jwt: string): Promise<Conversation[]> {
	try {
		const result = await fetchConversations(jwt);
		if (result && result.conversations) {
			// Sort conversations by last_message_at (newest first)
			const sortedConversations = [...result.conversations].sort((a, b) => {
				return new Date(b.last_message_at).getTime() - new Date(a.last_message_at).getTime();
			});

			conversations.length = 0;
			conversations.push(...sortedConversations);
			return sortedConversations;
		}
		return [];
	} catch (error) {
		console.error('Error refreshing conversationsSvelte:', error);
		return [];
	}
}
