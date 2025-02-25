/**
 * @typedef ITrack
 * @type {object}
 * @property {string} id - Track ID.
 * @property {string} title - Track title.
 * @property {string[]} artists - Track artists
 * @property {string} album - Track album
 * @property {number} duration - Track playback duration.
 * @property {ITrackPicture[]} pictures - Track pictures
 */

/**
 * @typedef ITrackPicture
 * @type {object}
 * @property {string} id - Picture ID
 * @property {number[] | string[]} data - Picture Data
 * @property {string} descripton - Picture Description
 * @property {string} mime - Picture MIME type
 * @property {number} picture_type - Picture type for internal reference
 */

/**
 * @typedef IPlaylist
 * @type {object}
 * @property {string} id - Playlist ID
 * @property {string} title - Playlist title
 * @property {string} description - Playlist description
 * @property {ITrack[]} tracks - Playlist tracks
 * @property {string} cover - Playlist cover @todo! Change this type to ITrackPicture?
 * @property {number} duration - Playlist duration
 */

// Export to make this file into a module, this export is not being used anywhere
export const types = true;