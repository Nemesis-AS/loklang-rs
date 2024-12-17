<script>
	import '../app.css';

	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Player from '$lib/components/layout/Player.svelte';
	import TrackInfoPanel from '$lib/components/layout/TrackInfoPanel.svelte';
	import LyricsPanel from '$lib/components/layout/LyricsPanel.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>Loklang</title>

	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
	<link
		href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="flex h-screen flex-col">
	<div class="flex flex-1 overflow-hidden">
		<Sidebar />
		<main class="flex-1 overflow-y-auto p-6">{@render children()}</main>
		<TrackInfoPanel />
		<!-- <LyricsPanel /> -->
	</div>
	<Player />
</div>