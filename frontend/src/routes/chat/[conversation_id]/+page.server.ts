export function load({ locals }) {
	return {
		auth_token: String(locals.auth_token)
	};
}
