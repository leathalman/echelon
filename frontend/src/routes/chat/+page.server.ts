// TODO: improve data flow with runes here maybe

export async function load({ locals }) {
	const jwt = String(locals.jwt);
	const user = locals.user;
	return {
		jwt,
		user
	};
}
