
// src/hooks.server.js
import { backend_url } from '$lib/utils.js';
import { redirect, type Handle } from '@sveltejs/kit';
const unProtectedRoutes = ['/login'];
export const handle: Handle = async ({ event, resolve }) => {
    console.log(event.url.pathname);
    console.log(event.cookies.get('id'));
    if (!unProtectedRoutes.includes(event.url.pathname)) {
        const res = await fetch(`${backend_url()}/reports`, {
            headers: {
                Cookie: `id=${event.cookies.get('id')}` 
            }
        });
        if (res.status == 401) { // unauthorized
            redirect(303, '/login?redirected=true');
        }
    }
    
    
    return await resolve(event);
};
