import type { User } from '$lib/model/user';
import { fetchUser } from '$lib/api/client';

export const userState: User = $state({
	id: '',
	email: '',
	first_name: '',
	last_name: '',
	student_id: ''
});

export async function refreshUser(jwt: string) {
	try {
		const result = await fetchUser(jwt);

		userState.id = result.id;
		userState.email = result.email;
		userState.first_name = result.first_name;
		userState.last_name = result.last_name;
		userState.student_id = result.student_id;
	} catch (error) {
		console.error('Error refreshing userState:', error);
	}
}
