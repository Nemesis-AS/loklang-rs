<script>
	import Icon from '@iconify/svelte';

	import IconButton from '../ui/IconButton.svelte';

	const currentTrack = {
		title: 'Track 1',
		artist: 'Artist 1',
		album: 'Album 1',
		duration: 180,
		image: '/404.png'
	};
</script>

<div class="relative w-64 bg-gray-100 p-4">
	<!-- onClick={() => setShowLyrics(false)} -->
	<div class="flex justify-end px-2 py-2">
		<IconButton>
			<Icon icon="lucide:x" class="size-5" />
		</IconButton>
	</div>

	<h2 class="mb-4 text-xl font-semibold">Lyrics</h2>

	{#if !currentTrack}
		<p class="text-gray-500">No track selected</p>
	{:else}
		<div>
			<h3 class="mb-2 font-semibold">{currentTrack.title}</h3>
			<p class="mb-4 text-sm text-gray-600">by {currentTrack.artist}</p>
			<p class="whitespace-pre-line">
				<!-- This is where you'd typically fetch and display the actual lyrics -->
				This is a placeholder for the lyrics of {currentTrack.title}. Actual lyrics would be
				displayed here, usually fetched from an API or stored in your database. The lyrics would be
				formatted with line breaks and verses as needed.
			</p>
		</div>
	{/if}

	<!-- {currentTrack ? (
      <div>
        <h3 class="font-semibold mb-2">{currentTrack.title}</h3>
        <p class="text-sm text-gray-600 mb-4">by {currentTrack.artist}</p>
        <p class="whitespace-pre-line">
          {/* This is where you'd typically fetch and display the actual lyrics */}
          This is a placeholder for the lyrics of {currentTrack.title}.
          
          Actual lyrics would be displayed here, usually fetched from an API or stored in your database.
          
          The lyrics would be formatted with line breaks and verses as needed.
        </p>
      </div>
    ) : (
      <p class="text-gray-500">No track selected</p>
    )} -->
</div>
