import { z } from 'zod';

export const formSchema = z.object({
	email: z
		.string()
		.email('Please enter a valid email address')
		.refine((email) => email.endsWith('@tcu.edu'), {
			message: 'Email must use the @tcu.edu domain'
		}),
	password: z
		.string()
});

export type FormSchema = typeof formSchema;
