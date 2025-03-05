import type { NewUser } from '$lib/state/new-user.svelte';

export async function login(
	email: string,
	password: string
): Promise<{ success: boolean; error?: string }> {
	try {
		const response = await fetch('http://localhost:8000/api/auth/login', {
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
	await fetch('http://localhost:8000/api/auth/logout', {
		method: 'GET',
		credentials: 'include'
	});
}

export async function signup(user: NewUser): Promise<{ success: boolean; error?: string }> {
	console.log(user);

	try {
		const response = await fetch('http://localhost:8000/api/auth/signup', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email: user.email,
				password: user.password,
				student_id: user.student_id,
				first_name: user.first_name,
				last_name: user.last_name,
				university: user.university
			})
		});

		// Handle non-2xx responses properly
		if (!response.ok) {
			const errorData = await response.json(); // assuming the API provides error details
			return { success: false, error: errorData.message || 'Signup failed' };
		}

		// If successful, return success
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
