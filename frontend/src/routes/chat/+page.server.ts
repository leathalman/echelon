import { fetchConversations } from '$lib/api/client';

export async function load({ cookies }) {
	const jwt = cookies.get('auth_token');
	const conversations = await fetchConversations(jwt);
	return {
		jwt,
		conversations
	};
}
