import { fetchConversations } from '$lib/api/client';
import type { Conversation } from '$lib/model/conversations';

export const conversationsState: Conversation[] = $state([]);

export async function refreshConversations(jwt: string) {
	try {
		const result = await fetchConversations(jwt);
		if (result && result.conversations) {
			// Sort conversations by last_message_at (newest first)
			const sortedConversations = [...result.conversations].sort((a, b) => {
				return new Date(b.last_message_at).getTime() - new Date(a.last_message_at).getTime();
			});

			conversationsState.length = 0;
			conversationsState.push(...sortedConversations);
		}
	} catch (error) {
		console.error('Error refreshing conversationsSvelte:', error);
	}
}
