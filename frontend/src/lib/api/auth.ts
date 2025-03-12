import { API_CONFIG } from './config';

export async function login(
	email: string,
	password: string
): Promise<{ success: boolean; error?: string }> {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/auth/login`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ email, password }),
			credentials: 'include'
		});

		// Handle non-2xx responses
		if (!response.ok) {
			const errorData = await response.json(); // assuming the API provides error details
			return { success: false, error: errorData.message || 'Login failed' };
		}

		// If successful, process the response or user data as needed
		await response.json();
		return { success: true };
	} catch (error) {
		// Catch any network or unexpected errors
		console.error('Error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'Unknown error occurred'
		};
	}
}

export async function logout() {
	await fetch(`${API_CONFIG.BASE_URL}/auth/logout`, {
		method: 'GET',
		credentials: 'include'
	});
}

export async function signup(
	email: string,
	password: string
): Promise<{ success: boolean; error?: string }> {
	try {
		const response = await fetch(`${API_CONFIG.BASE_URL}/auth/signup`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			credentials: 'include',
			body: JSON.stringify({
				email,
				password
			})
		});

		// Handle non-2xx responses
		if (!response.ok) {
			const errorData = await response.json();
			return { success: false, error: errorData.message || 'Signup failed' };
		}

		// If successful, process the response
		await response.json();
		return { success: true };
	} catch (error) {
		console.error('Error:', error);
		return {
			success: false,
			error: error instanceof Error ? error.message : 'Unknown error occurred'
		};
	}
}
