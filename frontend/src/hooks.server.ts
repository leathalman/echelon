import type { Handle } from '@sveltejs/kit';
import { fetchUser } from '$lib/api/client';

export const handle: Handle = async ({ event, resolve }) => {
	const auth_token = String(event.cookies.get('auth_token'));
	const user = await fetchUser(auth_token);

	event.locals.jwt = auth_token;
	event.locals.user = user;

	console.log(`Auth token: ${auth_token}`);
	console.log(`User: ${user}`);

	const publicRoutes = ['/login', '/signup', '/'];

	// If not logged in and trying to access protected route, redirect to landing
	if (
		(!auth_token || auth_token === 'null' || auth_token === 'undefined') &&
		!publicRoutes.includes(event.url.pathname)
	) {
		console.log('rerouting to /');
		return Response.redirect(new URL('/', event.url), 303);
	}

	// If logged in and trying to access public routes, redirect to chat
	if (
		auth_token &&
		auth_token !== 'null' &&
		auth_token !== 'undefined' &&
		publicRoutes.includes(event.url.pathname)
	) {
		console.log('rerouting logged-in user to /chat');
		return Response.redirect(new URL('/chat', event.url), 303);
	}

	return resolve(event);
};
