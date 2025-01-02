/** @typedef {import('../types.js').ITrack} ITrack  */

// Temporary type for playlist
// eslint-disable-next-line no-undef
export const currentPlaylist = $state({
    /** @type {ITrack[]} */
    tracks: [],
    trackIndex: -1,
});

/**
 * 
 * @param {object} playlist - Playlist object
 */
function setPlaylist(playlist) {
    currentPlaylist.tracks = playlist.tracks;
    currentPlaylist.trackIndex = 0;
}

/**
 * 
 * @returns {ITrack} - Next track in the playlist
 */
function getNextTrack() {
    currentPlaylist.trackIndex++;

    if (currentPlaylist.trackIndex >= currentPlaylist.tracks.length) {
        currentPlaylist.trackIndex = 0;
    }

    return currentPlaylist.tracks[currentPlaylist.trackIndex];
}

/**
 * 
 * @returns {ITrack} - Previous track in the playlist
 */
function getPrevTrack() {
    currentPlaylist.trackIndex--;

    if (currentPlaylist.trackIndex < 0) {
        currentPlaylist.trackIndex = currentPlaylist.tracks.length - 1;
    }

    return currentPlaylist.tracks[currentPlaylist.trackIndex];
}