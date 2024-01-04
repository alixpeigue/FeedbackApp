import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';
import { parseString } from 'set-cookie-parser';

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();
        
        const res = await fetch("http://localhost:3000/login", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8',
            },
            body: new URLSearchParams({
                "name": data.get('name')?.toString() || " ",
            })
        })
        const new_cookie = res.headers.getSetCookie().at(0);
        if (new_cookie) {
            const {name, value, ...opts} = parseString(new_cookie);
            cookies.set(name, value, opts);
        } else {
            return fail(400);
        }
        return {success: true};
    }
} satisfies Actions;
