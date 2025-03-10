import type { Handle } from '@sveltejs/kit';

// responsible for intercepting requests based on user auth status
export const handle: Handle = async ({ event, resolve }) => {
	const auth_token = event.cookies.get('auth_token');
	const onboarding_complete = event.cookies.get('onboarding_complete');

	const publicRoutes = ['/', '/login', '/signup'];
	const onboardingRoute = '/onboarding';
	const isPublicRoute = publicRoutes.includes(event.url.pathname);
	const isOnboardingRoute = event.url.pathname === onboardingRoute;

	// If not logged in and trying to access protected route, redirect to landing
	if ((!auth_token || auth_token === 'null' || auth_token === 'undefined') &&
		!isPublicRoute && !isOnboardingRoute) {
		console.log("Redirected from hooks -> attempted to access private route");
		return new Response(null, {
			status: 303,
			headers: { Location: '/' }
		});
	}

	// If logged in but onboarding not complete
	if (auth_token && auth_token !== 'null' && auth_token !== 'undefined') {
		// Case 1: If user has auth token but hasn't completed onboarding and tries to access a
		// route other than onboarding, redirect to onboarding
		if ((!onboarding_complete || onboarding_complete === 'false') &&
			!isOnboardingRoute) {
			console.log("Redirected from hooks -> onboarding not complete");
			return new Response(null, {
				status: 303,
				headers: { Location: '/onboarding' }
			});
		}

		// Case 2: If user has auth token, has completed onboarding, and is trying to
		// access public routes or onboarding, redirect to chat
		if (onboarding_complete === 'true' && (isPublicRoute || isOnboardingRoute)) {
			console.log("Redirected from hooks -> already logged in, bypass public routes");
			return new Response(null, {
				status: 303,
				headers: { Location: '/chat' }
			});
		}
	}

	return resolve(event);
};