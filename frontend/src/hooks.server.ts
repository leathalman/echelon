import type { Handle } from '@sveltejs/kit';
import { getAuthToken, setAuthToken } from '$lib/auth/token';

export const handle: Handle = async ({ event, resolve }) => {
	const authCookie = event.cookies.get('auth_token');
	const publicRoutes = ['/login', '/signup', '/'];

	// if not logged in, redirect all routes to base URL
	if (!authCookie && !publicRoutes.includes(event.url.pathname)) {
		return Response.redirect(new URL('/', event.url), 303);
	}

	// if logged in, redirect base URL to /chat
	if (authCookie && event.url.pathname === '/') {
		return Response.redirect(new URL('/chat', event.url), 303);
	}

	return resolve(event);
};
