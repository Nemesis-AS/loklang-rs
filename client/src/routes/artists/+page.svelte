<script>
	let { data } = $props();
	let { success, artists } = data;
</script>

<div>
	<h1 class="mb-4 text-2xl font-bold">Artists</h1>
	{#if !success}
		<p>An error occurred while loading artists...</p>
	{:else}
		{#each artists as artist}
			<div class="flex cursor-pointer items-center justify-between rounded p-2 hover:bg-gray-100">
				<!-- onClick={() => handleTrackClick(track)} -->
				<div>
					<a href="/artists/{artist}" class="font-semibold">{artist}</a>
					<!-- <p class="text-sm text-gray-600">{formatArtists(track.artists)} - {track.album}</p> -->
				</div>
				<!-- <p class="text-sm text-gray-500">
			{Math.floor(track.duration / 60)}:{(track.duration % 60).toString().padStart(2, '0')}
		</p> -->
			</div>
		{/each}
	{/if}
</div>
