/**
 * @typedef {import('../types.js').ITrack} ITrack
 */

import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

// @todo! Add loop playlist option

// eslint-disable-next-line no-undef
export const playerState = $state({
	// /** @type {ITrack | null} */
	// currentTrack: null,
	volume: 50,
	muted: false,
	paused: false,
	currTime: 0,
	/** @type {HTMLAudioElement | null} */
	audioPlayer: null,

	/** @type {ITrack | null} */
	currentTrack: null,
	/** @type {ITrack[]} */
	tracks: [],
	/** @type {ITrack[]} */
	queue: [],
	/** @type {ITrack[]} */
	history: [],
	playMode: 'repeat-off'
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
	console.log('Setting time to', value);
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

/**
 *
 * @param {*} ctx
 * @param {ITrack} track Track to be played
 * @description This function is to be used from the UI when playing songs. It will also load in the playlist info along with playing the track
 */
export function playTracksWithContext(ctx, track) {
	// Context takes in the playlist ID
	// Playlist data would be fetched
	// Playlist ID: type:ID:sort
	// Splits the playlist, moves prev tracks to history and next tracks to tracks

	playTrack(track);
}

export function handlePrevious() {
	if (!playerState.audioPlayer) return;

	if (playerState.audioPlayer.currentTime > 5) {
		playerState.audioPlayer.currentTime = 0;
	} else {
		if (playerState.history.length === 0)
			if (playerState.currentTrack) playTrack(playerState.currentTrack);

		let prevTrack = playerState.history.shift();
		let currTrack = playerState.currentTrack;
		if (currTrack) playerState.tracks.unshift(currTrack);
		if (prevTrack) playTrack(prevTrack);
	}
}

export function handleNext() {
	if (!playerState.audioPlayer) return;

	if (playerState.queue.length > 0) {
		let currTrack = playerState.currentTrack;
		if (currTrack) playerState.history.unshift(currTrack);

		let nextTrack = playerState.queue.shift();
		if (nextTrack) playTrack(nextTrack);
	} else if (playerState.tracks.length > 0) {
		let currTrack = playerState.currentTrack;
		if (currTrack) playerState.history.unshift(currTrack);

		let nextTrack = playerState.tracks.shift();
		if (nextTrack) playTrack(nextTrack);
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
