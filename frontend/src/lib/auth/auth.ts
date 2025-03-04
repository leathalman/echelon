export async function login(email: string, password: string) {
	const res = await fetch('http://localhost:8000/api/auth/login', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ email, password }),
		credentials: 'include'
	});

	if (res.ok) {
		await res.json();
	} else {
		throw new Error('Login failed');
	}
}

export async function logout() {
	await fetch('http://localhost:8000/api/auth/logout', {
		method: 'GET',
		credentials: 'include'
	});
}
