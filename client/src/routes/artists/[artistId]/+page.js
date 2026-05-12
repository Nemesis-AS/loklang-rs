import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch, params }) => {
	try {
		const artistRes = await fetch(`${PUBLIC_DEV_BASE_URL}/artists/${params.artistId}`);
		const artistJson = await artistRes.json();

		const trackRes = await fetch(`${PUBLIC_DEV_BASE_URL}/artists/${params.artistId}/songs`);
		const trackJson = await trackRes.json();

		return {
			success: true,
			artist: artistJson,
			tracks: trackJson,
		};
	} catch (err) {
		console.error('An error occurred while fetching artist!\n', err);
		return {
			success: false,
			message: 'An error occurred while fetching artist!'
		};
	}
};
