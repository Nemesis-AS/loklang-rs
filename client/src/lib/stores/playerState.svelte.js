/**
 * @typedef {import('../types.js').ITrack} ITrack
 */

import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

// @todo! Add loop playlist option

// eslint-disable-next-line no-undef
export const playerState = $state({
	/** @type {ITrack | null} */
	currentTrack: null,
	volume: 50,
	muted: false,
	paused: false,
	currTime: 0,
	/** @type {HTMLAudioElement | null} */
	audioPlayer: null
});

/**
 * @param {number} value
 */
export function setVolume(value) {
	if (value < 0) value = 0;
	if (value > 1) value = 1;

	playerState.volume = value;

	if (!playerState.audioPlayer) return;
	playerState.audioPlayer.volume = value / 100;
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
	if (!playerState.audioPlayer) return;

	playerState.currentTrack = track;
	playerState.paused = false;
	
	// Set current playlist if not set
	// Set the proper trackIndex

	playerState.audioPlayer.src = `${PUBLIC_DEV_BASE_URL}/stream/${track.id}`;
	playerState.audioPlayer.load();
	playerState.audioPlayer.play();

	console.log(playerState);
}

/**
 * @param {number} value
 */
export function setCurrTime(value) {
	if (!playerState.currentTrack || !playerState.audioPlayer) return;

	// @todo Play next song here
	if (value > playerState.currentTrack.duration) value = playerState.currentTrack.duration;

	// @todo! Remove this, it is only present for DEBUGGING purposes
	console.log("Setting time to", value);
	playerState.audioPlayer.currentTime = value;
	playerState.currTime = value;
}

export function togglePause() {
	if (!playerState.audioPlayer) return;

	playerState.paused = !playerState.paused;

	if (playerState.paused) {
		playerState.audioPlayer.pause();
	} else {
		playerState.audioPlayer.play();
	}
}

export function toggleMute() {
	playerState.muted = !playerState.muted;

	if (!playerState.audioPlayer) return;
	if (playerState.muted) {
		playerState.audioPlayer.volume = 0;
	} else {
		playerState.audioPlayer.volume = playerState.volume / 100;
	}
}

export function initializeAudioPlayer() {
	playerState.audioPlayer = new Audio();
	playerState.audioPlayer.volume = playerState.volume / 100;
	playerState.audioPlayer.addEventListener('timeupdate', () => {
		playerState.currTime = playerState.audioPlayer ? playerState.audioPlayer.currentTime : 0;
	});
	playerState.audioPlayer.addEventListener('ended', () => {
		playerState.currTime = 0;
	});
}