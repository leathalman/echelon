import { z } from 'zod';

export const formSchema = z.object({
	email: z
		.string()
		.email('Please enter a valid email address')
		.min(5, 'Email must be at least 5 characters')
		.max(50, 'Email cannot exceed 50 characters'),
	password: z
		.string()
		.min(8, 'Password must be at least 8 characters')
		.max(50, 'Password cannot exceed 50 characters')
		.regex(
			/^(?=.*[A-Z])(?=.*\d)[A-Za-z\d!@#$%^&*(),.?":{}|<>]{8,}$/,
			'Password must contain at least one uppercase letter and one number'
		)
});

export type FormSchema = typeof formSchema;
