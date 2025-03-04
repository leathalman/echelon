// TODO: improve data flow with runes here maybe
import { fetchUser } from '$lib/api/client';
import type { User } from '$lib/model/user';

export async function load({ locals }) {
	const jwt = String(locals.jwt);
	const user: User = await fetchUser(jwt);
	return {
		jwt,
		user
	};
}
