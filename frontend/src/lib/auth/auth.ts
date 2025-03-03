import { clearAuthToken, getAuthToken, setAuthToken } from '$lib/auth/token';

export async function fetchWithAuth(endpoint: string, options: RequestInit = {}) {
	const token = getAuthToken() ?? sessionStorage.getItem('token'); // use session storage as backup

	const headers = new Headers(options.headers || {});

	if (token) {
		headers.set('Authorization', `Bearer ${token}`);
	}

	const res = await fetch(`http://localhost:8000/api${endpoint}`, {
		...options,
		headers,
		credentials: 'include'
	});

	if (!res.ok) {
		throw new Error(`API error: ${res.status}`);
	}

	return res.json();
}

export async function login(email: string, password: string) {
	const res = await fetch('http://localhost:8000/api/auth/login', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email, password }),
		credentials: 'include'
	});

	if (res.ok) {
		const data = await res.json();
		console.log(data);
		setAuthToken(data.token);
		sessionStorage.setItem('token', data.token);
	} else {
		throw new Error('Login failed');
	}
}

export async function logout() {
	await fetch('http://localhost:8000/api/auth/logout', {
		method: 'POST',
		credentials: 'include'
	});

	clearAuthToken();
	sessionStorage.removeItem('token');
}
