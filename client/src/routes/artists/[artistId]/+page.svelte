<script>
	import Icon from '@iconify/svelte';

	import IconButton from '$lib/components/ui/IconButton.svelte';

	import { playTrack } from '$lib/stores/playerState.svelte';
	import { formatArtists } from '$lib/utils';

	let { data } = $props();
	let { success, tracks } = data;
</script>

<div>
	<div class="flex w-full items-center gap-4 px-8">
		<img src="/404.png" alt="Album Cover" class="w-full max-w-56 grow rounded-md" />
		<div class="flex flex-col gap-4">
			<h1 class="text-2xl font-bold">Artist Name</h1>
			<h4 class="text-xl font-semibold text-gray-700">Artist Type (Artist/Duo/Band)</h4>

			<div class="flex gap-4">
                <span>Progressive House</span>
                <span>X Albums</span>
				<span>{tracks.length} {tracks.length === 1 ? 'Song' : 'Songs'}</span>
				<span>XX:XX</span>
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
        <div class="flex items-center justify-between">
            <h2 class="px-2 py-2 text-xl font-semibold">Tracks</h2>
            
            <div>
                <button class="rounded-md p-2 text-base text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900">
                    <Icon icon="lucide:layout-grid" />
                </button>
                <button class="rounded-md p-2 text-base text-gray-800 transition-colors duration-300 hover:bg-gray-200 hover:text-gray-900" >
                    <Icon icon="lucide:list" />
                </button>
            </div>
        </div>

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
</div>
