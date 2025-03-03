import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const authCookie = event.cookies.get('auth_token');

	if (!authCookie && event.url.pathname !== '/login') {
		return Response.redirect(new URL('/login', event.url), 303);
	}

	return resolve(event);
};
