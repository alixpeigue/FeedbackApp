import type { PageLoad } from '../$types';

export const load: PageLoad = async ({ fetch }) => {
	return {
		locations: await (async () => {
			const res = await fetch('http://localhost:3000/locations');
			return (await res.json()).map((el) => {
				return { value: '' + el.id, label: el.name };
			});
		})(),
		contracts: await (async () => {
			const res = await fetch('http://localhost:3000/contracts');
			return (await res.json()).map((el) => {
				return { value: '' + el.id, label: el.description };
			});
		})()
	};
};
