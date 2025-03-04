import { fetchConversations } from '$lib/api/client';

export async function load({ cookies, locals }) {
	const auth_token = String(locals.auth_token);
	const conversations = await fetchConversations(auth_token);
	return {
		auth_token,
		conversations
	};
}
