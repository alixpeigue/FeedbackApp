import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { parseString } from 'set-cookie-parser';
import { backend_url } from '$lib/utils.js';

export const actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();

		const res = await fetch(`${backend_url()}/login`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'
			},
			body: new URLSearchParams({
				name: data.get('name')?.toString() || ' '
			})
		});
		const new_cookie = res.headers.getSetCookie().at(0);
		if (new_cookie) {
			const { name, value, ...opts } = parseString(new_cookie);
			cookies.set(name, value, opts);
		} else {
			return fail(400, {success: false});
		}
		return {success: true};
	}
} satisfies Actions;
