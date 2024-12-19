<script>
	import Icon from '@iconify/svelte';
	import IconButton from '../ui/IconButton.svelte';

	import { PUBLIC_DEV_BASE_URL } from '$env/static/public';
	import { formatArtists } from '$lib/utils';

	// State
	import { layoutData } from '$lib/stores/LayoutData.svelte.js';
	import { playerState } from '$lib/stores/playerState.svelte';

	const currentTrack = $derived(playerState.currentTrack);
	// const isTrackEmpty = $derived(Object.keys(currentTrack ?? {}).length === 0);

	let trackPicture = $state('');

	$effect(() => {
		if (!currentTrack || currentTrack.pictures.length === 0) {
			trackPicture = '/404.png';
			return;
		}

		let picID = currentTrack.pictures[0].id;

		fetch(`${PUBLIC_DEV_BASE_URL}/picture/${picID}`)
			.then((res) => res.text())
			.then((data) => (trackPicture = data))
			.catch((err) => {
				console.log('An error occurred while fetching song image!');
				trackPicture = '/404.png';
			});
	});

	function hidePanel() {
		layoutData.trackInfoPanel = false;
	}
</script>

{#if layoutData.trackInfoPanel}
	<div class="relative w-80 bg-gray-100 p-4">
		<div class="flex justify-end px-2 py-2">
			<IconButton onclick={hidePanel}>
				<Icon icon="lucide:x" class="size-5" />
			</IconButton>
		</div>

		{#if !currentTrack}
			<p class="text-center text-gray-500">No track selected</p>
		{:else}
			<div class="text-center">
				<img
					src={trackPicture ? trackPicture : '/404.png'}
					alt={currentTrack.title}
					class="mx-auto mb-4 w-full rounded"
				/>
				<h2 class="text-xl font-semibold">{currentTrack.title}</h2>
				<p class="text-gray-600">{formatArtists(currentTrack.artists)}</p>
				<p class="text-gray-500">{currentTrack.album}</p>
				<p class="text-gray-500">
					Duration: {Math.floor(currentTrack.duration / 60)}:
					{(currentTrack.duration % 60).toString().padStart(2, '0')}
				</p>
			</div>
		{/if}
	</div>
{/if}
