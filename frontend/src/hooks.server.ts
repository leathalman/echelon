import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const authCookie = event.cookies.get('auth_token');
	const publicRoutes = ['/login', '/signup', '/'];

	if (!authCookie && !publicRoutes.includes(event.url.pathname)) {
		return Response.redirect(new URL('/', event.url), 303);
	}

	return resolve(event);
};
