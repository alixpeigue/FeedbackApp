import { backend_url } from '$lib/utils';
import type { PageLoad } from '../$types';

export const load: PageLoad = async ({ fetch }) => {
	return {
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
