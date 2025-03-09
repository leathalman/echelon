import type { Handle } from '@sveltejs/kit';

// responsible for intercepting requests based on user auth status
export const handle: Handle = async ({ event, resolve }) => {
	const auth_token = event.cookies.get('auth_token');

	const publicRoutes = ['/', '/login', '/signup', '/onboarding'];
	const isPublicRoute = publicRoutes.includes(event.url.pathname);

	// If not logged in and trying to access protected route, redirect to landing
	if ((!auth_token || auth_token === 'null' || auth_token === 'undefined') && !isPublicRoute) {
		return new Response(null, {
			status: 303,
			headers: { Location: '/' }
		});
	}

	// If logged in and trying to access public routes, redirect to chat
	if (auth_token && auth_token !== 'null' && auth_token !== 'undefined' && isPublicRoute) {
		return new Response(null, {
			status: 303,
			headers: { Location: '/chat' }
		});
	}

	return resolve(event);
};
