import { z } from 'zod';

export const formSchema = z.object({
	text: z.string(),
	location: z.number(),
	contract: z.number()
});

export type FormSchema = typeof formSchema;
