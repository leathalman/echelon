import '$app/environment';

let token: string | undefined = undefined;

export function getAuthToken() {
	return token;
}

export function setAuthToken(newToken: string | undefined) {
	token = newToken;
}

export function clearAuthToken() {
	token = undefined;
}
