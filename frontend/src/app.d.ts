import type { Conversation } from '$lib/api/conversations';

declare global {
	namespace App {
		interface Locals {
			auth_token: string | null;
			conversations: Conversation[];
		}
	}
}

export {};
