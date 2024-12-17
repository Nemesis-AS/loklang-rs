<script>
	import { page } from '$app/state';

	import Icon from '@iconify/svelte';

	const links = [
		{ href: '/', label: 'Tracks', icon: 'lucide:music' },
		{ href: '/albums', label: 'Albums', icon: 'lucide:disc' },
		{ href: '/artists', label: 'Artists', icon: 'lucide:user' },
		{ href: '/favorites', label: 'Favorites', icon: 'lucide:heart' }
	];

	const playlists = [
		{
			id: 1,
			name: 'My Playlist',
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
		}
	];
</script>

<div class="hidden w-64 overflow-y-auto bg-gray-100 p-4 md:block">
	<nav class="space-y-2">
		{#each links as link}
			<a
				href={link.href}
				class={`flex items-center space-x-2 rounded px-2 py-1 hover:bg-gray-200 ${page.url.pathname === link.href && 'bg-gray-200 font-semibold'}`}
			>
				<Icon icon={link.icon} class="size-5" />
				<span>{link.label}</span>
			</a>
		{/each}

		<div class="pt-4">
			<h3 class="mb-2 font-semibold">Playlists</h3>
			{#each playlists as playlist}
				<a
					href={`/playlist/${playlist.id}`}
					class={`flex items-center space-x-2 rounded px-2 py-1 hover:bg-gray-200 ${page.url.pathname === `/playlist/${playlist.id}` && 'bg-gray-200 font-semibold'}`}
				>
					<Icon icon="lucide:list" class="size-5" />
					<span>{playlist.name}</span>
				</a>
			{/each}
		</div>
	</nav>
</div>
