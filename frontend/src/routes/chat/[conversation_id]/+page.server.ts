export function load({ locals }) {
	return {
		jwt: String(locals.jwt)
	};
}
