/**
 * @typedef {import('../types.js').ITrack} ITrack
 */

// eslint-disable-next-line no-undef
export const playerState = $state({
	/** @type {ITrack | null} */
	currentTrack: null,
	volume: 50,
	muted: false,
	paused: false,
	currTime: 0,
});

// @todo Add a Track Interface/Type

/**
 * @param {number} value
 */
export function setVolume(value) {
	if (value < 0) value = 0;
	if (value > 1) value = 1;

	playerState.volume = value;
}

/**
 * @param {boolean} value
 */
export function setPaused(value) {
	playerState.paused = value;
}

/**
 * @param {ITrack} track
 */
export async function playTrack(track) {
	playerState.currentTrack = track;
	playerState.paused = false;

	console.log(playerState);
}

/**
 * @param {number} value
 */
export function setCurrTime(value) {
	if (!playerState.currentTrack) return;

    // @todo Play next song here
    if (value > playerState.currentTrack.duration)
        value = playerState.currentTrack.duration;

	playerState.currTime = value;
}

export function togglePause() {
	playerState.paused = !playerState.paused;
}

export function toggleMute() {
	playerState.muted = !playerState.muted;
}