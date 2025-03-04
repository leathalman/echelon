// TODO: improve data flow with runes here maybe
import { fetchUser } from '$lib/api/client';
import { userState } from '$lib/state/user.svelte';
import type { User } from '$lib/model/user';

export async function load({ locals }) {
	const auth_token = String(locals.auth_token);
	const user: User = await fetchUser(auth_token);
	return {
		auth_token,
		user
	};
}
