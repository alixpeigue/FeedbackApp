
// src/hooks.server.js
import { PUBLIC_SERVER_URL } from '$env/static/public';
import { redirect, type Handle } from '@sveltejs/kit';
const unProtectedRoutes = ['/login'];
export const handle: Handle = async ({ event, resolve }) => {
    console.log(event.url.pathname);
    console.log(event.cookies.get('id'));
    if (!unProtectedRoutes.includes(event.url.pathname)) {
        const res = await fetch(`${PUBLIC_SERVER_URL}/reports`, {
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
