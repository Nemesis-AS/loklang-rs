<script>
	/**
	 * @typedef {import('$lib/types.js').ITrack} ITrack
	 */

	import Icon from '@iconify/svelte';

	import IconButton from '$lib/components/ui/IconButton.svelte';

	import { playTrack } from '$lib/stores/playerState.svelte';
	import { formatArtists, formatDuration } from '$lib/utils';

	let { data } = $props();
	let { success, album, tracks } = data;

	const aggregateLength = tracks.reduce(
		(/** @type {number} */ len, /** @type { ITrack } */ track) => track.duration + len,
		0
	);
</script>

<div>
	<div class="flex w-full items-center gap-4 px-8">
		<img src="/404.png" alt="Album Cover" class="w-full max-w-56 grow rounded-md" />
		<div class="flex flex-col gap-4">
			<h1 class="text-2xl font-bold">{album.title}</h1>
			<h4 class="text-xl font-semibold text-gray-700">Album Artist</h4>

			<div class="flex gap-4">
				<span>20XX</span>
				<span>Progressive House</span>
				<span>{tracks.length} {tracks.length === 1 ? 'Song' : 'Songs'}</span>
				<span>{formatDuration(aggregateLength)}</span>
			</div>

			<div class="flex items-center gap-4">
				<button
					class="aspect-square rounded-full px-4 py-2 text-2xl text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900"
				>
					<Icon icon="lucide:play" />
				</button>
				<button
					class="flex items-center gap-2 rounded-md px-4 py-2 text-base text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900"
				>
					<Icon icon="lucide:shuffle" class="text-lg" /> Shuffle Play
				</button>

				<button
					class="flex items-center gap-2 rounded-md px-4 py-2 text-base text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900"
				>
					<Icon icon="lucide:list-plus" class="text-lg" /> Add to (Queue)
				</button>

				<button
					class="flex items-center gap-2 rounded-md px-4 py-2 text-base text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900"
				>
					<Icon icon="lucide:pencil-line" class="text-lg" /> Edit
				</button>
			</div>
		</div>
	</div>

	<div class="px-6 pt-4">
		<h2 class="px-2 py-2 text-xl font-semibold">Tracks</h2>

		<div class="space-y-2 divide-y divide-gray-200">
			{#if !success}
				<p>An error occurred while loading tracks...</p>
			{:else if tracks.length > 0}
				{#each tracks as track}
					<div class="flex items-center justify-between rounded p-2">
						<div class="flex items-center gap-4">
							<IconButton onclick={() => playTrack(track)}>
								<Icon icon="lucide:play" class="size-5" />
							</IconButton>

							<div>
								<h3 class="font-semibold">{track.title}</h3>
								<p class="text-sm text-gray-600">{formatArtists(track.artists)}</p>
							</div>
						</div>
						<p class="text-sm text-gray-500">
							{formatDuration(track.duration)}
						</p>
					</div>
				{/each}
			{:else}
				<p>No tracks to display</p>
			{/if}
		</div>
	</div>
</div>
