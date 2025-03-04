import { z } from 'zod';

export const formSchema = z.object({
	email: z
		.string()
		.email('Please enter a valid email address')
		.min(5, 'Email must be at least 5 characters')
		.max(50, 'Email cannot exceed 50 characters')
		.refine((email) => email.endsWith('@tcu.edu'), {
			message: 'Email must use the @tcu.edu domain'
		}),
	password: z.string()
});

export type FormSchema = typeof formSchema;
