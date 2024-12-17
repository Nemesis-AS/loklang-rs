import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
	try {
		const res = await fetch(`${PUBLIC_DEV_BASE_URL}/albums`);
		const json = await res.json();

		return {
			success: true,
			albums: json
		};
	} catch (err) {
		console.error('An error occurred while fetching albums!\n', err);
		return {
			success: false,
			message: 'An error occurred while fetching albums!'
		};
	}
};
