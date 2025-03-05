import { z } from 'zod';

export const formSchema = z.object({
	first_name: z.string().min(2).max(50),
	last_name: z.string().min(2).max(50),
	university: z.string().min(3).max(100),
	student_id: z.string().min(2).max(50)
});

export type FormSchema = typeof formSchema;
