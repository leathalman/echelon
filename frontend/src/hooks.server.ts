import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const auth_token = String(event.cookies.get('auth_token'));
	event.locals.jwt = auth_token;

	const publicRoutes = ['/login', '/signup', '/'];

	// if not logged in, redirect all routes to base URL
	if (!auth_token && !publicRoutes.includes(event.url.pathname)) {
		return Response.redirect(new URL('/', event.url), 303);
	}

	// if logged in, redirect base URL to /chat
	if (auth_token && event.url.pathname === '/') {
		return Response.redirect(new URL('/chat', event.url), 303);
	}

	return resolve(event);
};
