import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch, params }) => {
	try {
		const res = await fetch(`${PUBLIC_DEV_BASE_URL}/albums/${params.albumId}`);
		const json = await res.json();

		const trackRes = await fetch(`${PUBLIC_DEV_BASE_URL}/albums/${params.albumId}/songs`);
		const trackJson = await trackRes.json();

		return {
			success: true,
			album: json,
			tracks: trackJson
		};
	} catch (err) {
		console.error('An error occurred while fetching album!\n', err);
		return {
			success: false,
			message: 'An error occurred while fetching album!'
		};
	}
};
