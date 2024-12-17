<script>
	let { data } = $props();
	let { success, tracks } = data;

	/**
	 * Formats the array of artists into a string
	 * @param {String[] | undefined} artists
	 * @returns {String} Formatted artsts string
	 */
	function formatArtists(artists) {
		if (!artists)
			return "";
		return artists.join(', ');
	}
</script>

<div>
	<h1 class="mb-4 text-2xl font-bold">Tracks</h1>
	<div class="space-y-2">
		{#if !success}
			<p>An error occurred while loading tracks...</p>
		{:else}
			{#each tracks as track}
				<div class="flex cursor-pointer items-center justify-between rounded p-2 hover:bg-gray-100">
					<!-- onClick={() => handleTrackClick(track)} -->
					<div>
						<h3 class="font-semibold">{track.title}</h3>
						<p class="text-sm text-gray-600">{formatArtists(track.artists)} - {track.album}</p>
					</div>
					<p class="text-sm text-gray-500">
						{Math.floor(track.duration / 60)}:{(track.duration % 60).toString().padStart(2, '0')}
					</p>
				</div>
			{/each}
		{/if}
	</div>
</div>
