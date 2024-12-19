<script>
	import Icon from '@iconify/svelte';
	
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import { formatArtists } from '$lib/utils';

	import { playTrack } from '$lib/stores/playerState.svelte';

	let { data } = $props();
	let { success, tracks } = data;
</script>

<div>
	<h1 class="mb-4 text-2xl font-bold">Tracks</h1>
	<div class="space-y-2 divide-y divide-gray-200">
		{#if !success}
			<p>An error occurred while loading tracks...</p>
		{:else}
			{#each tracks as track}
				<div class="flex items-center justify-between rounded p-2">
					<div class="flex items-center gap-4">
						<IconButton onclick={() => playTrack(track)}>
							<Icon icon="lucide:play" class="size-5" />
						</IconButton>

						<div>
							<h3 class="font-semibold">{track.title}</h3>
							<p class="text-sm text-gray-600">{formatArtists(track.artists)} - {track.album}</p>
						</div>
					</div>
					<p class="text-sm text-gray-500">
						{Math.floor(track.duration / 60)}:{(track.duration % 60).toString().padStart(2, '0')}
					</p>
				</div>
			{/each}
		{/if}
	</div>
</div>
