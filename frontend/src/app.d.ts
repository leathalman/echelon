import type { User } from '$lib/model/user';

declare global {
	namespace App {
		interface Locals {
			jwt: string | null;
			user: User;
		}
	}
}

export {};
