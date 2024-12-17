import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
	try {
		const res = await fetch(`${PUBLIC_DEV_BASE_URL}/artists`);
		const json = await res.json();

		return {
			success: true,
			artists: json
		};
	} catch (err) {
		console.error('An error occurred while fetching artists!\n', err);
		return {
			success: false,
			message: 'An error occurred while fetching albums!'
		};
	}
};
