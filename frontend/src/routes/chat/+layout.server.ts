import type { LayoutServerLoad } from './$types';
import { fetchConversations, fetchUser } from '$lib/api/client';
import type { Conversation } from '$lib/model/conversations.svelte';
import type { User } from '$lib/model/user';

// layout.server needed because of cookies
export const load: LayoutServerLoad = async ({ cookies }) => {
	const authToken = cookies.get('auth_token');

	console.log('RELOADING FROM LAYOUT.SERVER.TS');

	if (authToken && authToken !== 'null' && authToken !== 'undefined') {
		try {
			const userResponse = await fetchUser(authToken);
			const conversationsResponse = await fetchConversations(authToken);

			// Ensure user is not null/undefined
			const user: User = userResponse || ({} as User);

			// Ensure conversations is not null/undefined
			const conversations: Conversation[] = conversationsResponse?.conversations
				? [...conversationsResponse.conversations].sort((a, b) => {
					return new Date(b.last_message_at).getTime() - new Date(a.last_message_at).getTime();
				})
				: [];

			return { authToken, user, conversations };
		} catch (error) {
			console.error('Error fetching:', error);
			// Return default values instead of error
			return {
				authToken,
				user: {} as User,
				conversations: [] as Conversation[]
			};
		}
	} else {
		// User not logged in - return empty objects/arrays instead of error
		return {
			authToken: '',
			user: {} as User,
			conversations: [] as Conversation[]
		};
	}
};
