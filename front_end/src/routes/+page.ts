import type { PageLoad } from './$types';
import { backend_url } from '$lib/utils';

export const load: PageLoad = async ({ fetch }) => {
	return {
		reports: await (async () => {
			const res = await fetch(`${backend_url()}/reports`, {credentials: 'include'});
			return await res.json();
		})(),
		clients: await (async () => {
			const res = await fetch(`${backend_url()}/clients`, {credentials: 'include'});
			return (await res.json()).map((el) => {
				return { value: '' + el.id, label: el.name };
			});
		})(),
		locations: await (async () => {
			const res = await fetch(`${backend_url()}/locations`, {credentials: 'include'});
			return (await res.json()).map((el) => {
				return { value: '' + el.id, label: el.name };
			});
		})(),
		contracts: await (async () => {
			const res = await fetch(`${backend_url()}/contracts`, {credentials: 'include'});
			return (await res.json()).map((el) => {
				return { value: '' + el.id, label: el.description };
			});
		})()
	};
};
