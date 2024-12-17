<script>
	import Icon from '@iconify/svelte';

	import IconButton from '$lib/components/ui/IconButton.svelte';
	import Slider from '$lib/components/ui/Slider.svelte';

	const currentTrack = {
		title: 'Track 1',
		artist: 'Artist 1',
		image: '/404.png'
	};
</script>

<div class="flex h-20 items-center border-t bg-gray-100 px-4">
	<div class="flex flex-1 items-center">
		<!-- {currentTrack && (
        <div
          class="flex items-center cursor-pointer hover:bg-gray-200 p-2 rounded"
          onClick={toggleRightPanel}
        >
          <Image
            src={currentTrack.image}
            alt={currentTrack.title}
            width={50}
            height={50}
            class="rounded mr-4"
          />
          <div>
            <h3 class="font-semibold">{currentTrack.title}</h3>
            <p class="text-sm text-gray-600">{currentTrack.artist}</p>
          </div>
        </div>
      )} -->
		<!-- onClick={toggleRightPanel} -->

		<button class="flex cursor-pointer items-center rounded p-2 hover:bg-gray-200">
			<img
				src={currentTrack.image}
				alt={currentTrack.title}
				width={50}
				height={50}
				class="mr-4 rounded"
			/>
			<div>
				<h3 class="font-semibold">{currentTrack.title}</h3>
				<p class="text-sm text-gray-600">{currentTrack.artist}</p>
			</div>
		</button>
	</div>
	<div class="flex flex-1 flex-col items-center">
		<div class="flex items-center space-x-4">
			<!-- <Button variant="ghost" size="icon">
          <SkipBack class="h-5 w-5" />
        </Button> -->

			<IconButton>
				<Icon icon="lucide:skip-back" class="size-5" />
			</IconButton>

			<IconButton>
				<Icon icon="lucide:play" class="size-5" />
			</IconButton>

			<IconButton>
				<Icon icon="lucide:skip-forward" class="size-5" />
			</IconButton>

			<!-- <Button
          variant="ghost"
          size="icon"
          onClick={() => setIsPlaying(!isPlaying)}
        >
          {isPlaying ? (
            <Pause class="h-5 w-5" />
          ) : (
            <Play class="h-5 w-5" />
          )}
        </Button> -->
			<!-- <Button variant="ghost" size="icon">
          <SkipForward class="h-5 w-5" />
        </Button> -->
		</div>
		<Slider class="mt-2 w-full max-w-screen-sm" value={0} max={100} step={1} />
		<!-- class="mt-2 w-64" -->

		<!-- <input
			type="range"
			name="seekBar"
			class="mt-2 w-full max-w-screen-md"
			value="0"
			max="100"
			step="1"
		/> -->
	</div>
	<div class="flex flex-1 items-center justify-end space-x-4">
		<div class="flex items-center space-x-2">
			<IconButton>
				<Icon icon="lucide:volume-x" class="size-5" />
			</IconButton>
			<!-- <input type="range" class="w-24" name="volumeSlider" value="100" max="100" min="0" step="1" /> -->
			<Slider class="w-24" />
			<!-- <Button variant="ghost" size="icon" onClick={toggleMute}>
          {isMuted ? (
            <VolumeX class="h-5 w-5" />
          ) : (
            <Volume2 class="h-5 w-5" />
          )}
        </Button>
        <Slider
          class="w-24"
          defaultValue={[100]}
          max={100}
          step={1}
          value={[volume * 100]}
          onValueChange={handleVolumeChange}
        /> -->
		</div>
		<IconButton>
			<Icon icon="lucide:mic" class="size-5" />
		</IconButton>
		<IconButton>
			<Icon icon="lucide:maximize" class="size-5" />
		</IconButton>
		<!-- <Button variant="ghost" size="icon" onClick={toggleLyrics}>
        <Mic class="h-5 w-5" />
      </Button>
      <Button variant="ghost" size="icon" onClick={toggleFullscreen}>
        <Maximize class="h-5 w-5" />
      </Button> -->
	</div>
</div>
