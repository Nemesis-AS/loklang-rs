<script>
	import Icon from '@iconify/svelte';

	import IconButton from '$lib/components/ui/IconButton.svelte';
	import Slider from '$lib/components/ui/Slider.svelte';

	import { formatArtists } from '$lib/utils';

	import { layoutData } from '$lib/stores/LayoutData.svelte';
	import { playerState, togglePause, toggleMute } from '$lib/stores/playerState.svelte';
	import { PUBLIC_DEV_BASE_URL } from '$env/static/public';

	const currentTrack = $derived(playerState.currentTrack);
	const isTrackEmpty = $derived(Object.keys(currentTrack ?? {}).length === 0);

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

	function openTrackInfoPanel() {
		if (layoutData.lyricsPanel) layoutData.lyricsPanel = false;

		layoutData.trackInfoPanel = true;
	}

	function openLyricsPanel() {
		if (layoutData.trackInfoPanel) layoutData.trackInfoPanel = false;

		layoutData.lyricsPanel = true;
	}
</script>

<div class="flex h-20 items-center border-t bg-gray-100 px-4">
	<div class="flex flex-1 items-center">
		<button
			class="flex cursor-pointer items-center rounded p-2 hover:bg-gray-200"
			onclick={openTrackInfoPanel}
		>
			<img
				src={trackPicture ? trackPicture : '/404.png'}
				alt={!isTrackEmpty ? currentTrack?.title : 'No Track Selected'}
				width={50}
				height={50}
				class="mr-4 rounded"
			/>
			<div>
				<h3 class="font-semibold">{!isTrackEmpty ? currentTrack?.title : 'No Track Selected'}</h3>
				<p class="text-sm text-gray-600">
					{!isTrackEmpty ? formatArtists(currentTrack?.artists) : 'No Track Selected'}
				</p>
			</div>
		</button>
	</div>
	<div class="flex flex-1 flex-col items-center">
		<div class="flex items-center space-x-4">
			<IconButton>
				<Icon icon="lucide:skip-back" class="size-5" />
			</IconButton>

			<IconButton onclick={togglePause}>
				{#if playerState.paused || isTrackEmpty}
					<Icon icon="lucide:play" class="size-5" />
				{:else}
					<Icon icon="lucide:pause" class="size-5" />
				{/if}
			</IconButton>

			<IconButton>
				<Icon icon="lucide:skip-forward" class="size-5" />
			</IconButton>
		</div>
		<Slider class="mt-2 w-full max-w-screen-sm" value={0} max={100} step={1} />
	</div>
	<div class="flex flex-1 items-center justify-end space-x-4">
		<div class="flex items-center space-x-2">
			<IconButton onclick={toggleMute}>
				{#if playerState.muted}
					<Icon icon="lucide:volume-off" class="size-5" />
				{:else if playerState.volume === 0}
					<Icon icon="lucide:volume-x" class="size-5" />
				{:else if playerState.volume < 50}
					<Icon icon="lucide:volume-1" class="size-5" />
				{:else}
					<Icon icon="lucide:volume-2" class="size-5" />
				{/if}
			</IconButton>
			<Slider bind:value={playerState.volume} class="w-24" />
		</div>
		<IconButton onclick={openLyricsPanel}>
			<Icon icon="lucide:mic" class="size-5" />
		</IconButton>
		<IconButton>
			<Icon icon="lucide:maximize" class="size-5" />
		</IconButton>
		<!-- <Button variant="ghost" size="icon" onClick={toggleFullscreen}>
        <Maximize class="h-5 w-5" />
      </Button> -->
	</div>
</div>
