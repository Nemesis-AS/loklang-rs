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
	<div class="flex justify-end py-2 px-2">
		<IconButton>
			<Icon icon="lucide:x" class="size-5" />
		</IconButton>
	</div>

	<!-- onClick={toggleRightPanel} -->

	{#if !currentTrack}
		<p class="text-center text-gray-500">No track selected</p>
	{:else}
		<div class="text-center">
			<img
				src={currentTrack.image}
				alt={currentTrack.title}
				width={200}
				height={200}
				class="mx-auto mb-4 rounded"
			/>
			<h2 class="text-xl font-semibold">{currentTrack.title}</h2>
			<p class="text-gray-600">{currentTrack.artist}</p>
			<p class="text-gray-500">{currentTrack.album}</p>
			<p class="text-gray-500">
				Duration: {Math.floor(currentTrack.duration / 60)}:
				{(currentTrack.duration % 60).toString().padStart(2, '0')}
			</p>
		</div>
	{/if}
</div>
