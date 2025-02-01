import { PUBLIC_DEV_BASE_URL } from "$env/static/public";

/** @type {import('./$types').PageLoad} */
export const load = async ({ fetch }) => {
    try {
        const res = await fetch(`${PUBLIC_DEV_BASE_URL}/songs`);
        const json = await res.json();
        return {
            success: true,
            tracks: json
        };
    } catch (err) {
        console.error("An error occurred while fetching tracks!\n", err);
        return {
            success: false,
            message: "Could not fetch songs!"
        };
    }
};