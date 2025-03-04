import type { Conversation } from '$lib/model/conversations';

declare global {
	namespace App {
		interface Locals {
			auth_token: string | null;
		}
	}
}

export {};
