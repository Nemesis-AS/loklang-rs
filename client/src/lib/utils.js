/**
 * Formats the array of artists into a string
 * @param {String[] | undefined} artists
 * @returns {String} Formatted artsts string
 */
export function formatArtists(artists) {
	if (!artists) return '';
	return artists.join(', ');
}
