<script>
	let playlist = {
        id: 1,
        name: "My Playlist",
        tracks: [
            {
                id: '1',
                title: 'Track 1',
                artist: 'Artist 1',
                album: 'Album 1',
                duration: 180,
                image: '/404.png'
            },
            {
                id: '2',
                title: 'Track 2',
                artist: 'Artist 2',
                album: 'Album 2',
                duration: 210,
                image: '/404.png'
            },
            {
                id: '3',
                title: 'Track 3',
                artist: 'Artist 3',
                album: 'Album 3',
                duration: 195,
                image: '/404.png'
            }
        ]
    };
</script>

<div>
	<h1 class="mb-4 text-2xl font-bold">{playlist.name}</h1>
	{#if playlist.tracks.length === 0}
		<p>Empty playlist</p>
	{:else}
		<div class="space-y-2">
			{#each playlist.tracks as track}
				<div class="flex cursor-pointer items-center justify-between rounded p-2 hover:bg-gray-100">
					<div>
						<h3 class="font-semibold">{track.title}</h3>
						<p class="text-sm text-gray-600">{track.artist} - {track.album}</p>
					</div>
					<p class="text-sm text-gray-500">
						{Math.floor(track.duration / 60)}:{(track.duration % 60).toString().padStart(2, '0')}
					</p>
				</div>
			{/each}
		</div>
	{/if}
</div>
