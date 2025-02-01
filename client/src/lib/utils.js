/**
 * Formats the array of artists into a string
 * @param {String[] | undefined} artists
 * @returns {String} Formatted artsts string
 */
export function formatArtists(artists) {
	if (!artists) return '';
	return artists.join(', ');
}

/**
 * Formats the duration in seconds into a string
 * @param {number} duration
 * @returns {string} Formatted duration string
 */
export function formatDuration(duration) {
	const minutes = Math.floor(duration / 60);
	const seconds = Math.floor(duration % 60);

	return `${minutes}:${seconds.toString().padStart(2, '0')}`;
}
