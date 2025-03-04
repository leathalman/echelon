import type { Conversation } from '$lib/model/conversations';

declare global {
	namespace App {
		interface Locals {
			jwt: string | null;
		}
	}
}

export {};
