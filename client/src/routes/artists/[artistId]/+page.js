import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch, params }) => {
	try {
		const res = await fetch(`${PUBLIC_DEV_BASE_URL}/artists/${params.artistId}`);
		const json = await res.json();

		return {
			success: true,
			tracks: json
		};
	} catch (err) {
		console.error('An error occurred while fetching artist!\n', err);
		return {
			success: false,
			message: 'An error occurred while fetching artist!'
		};
	}
};
