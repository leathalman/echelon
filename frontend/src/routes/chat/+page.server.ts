import { fetchConversations } from '$lib/api/client';

export async function load({ locals }) {
	const auth_token = String(locals.auth_token);
	return {
		auth_token
	};
}
