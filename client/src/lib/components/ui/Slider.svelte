<script>
	let { value = $bindable(50), min = 0, max = 100, step = 1, class: className = '', oninput = null } = $props();
</script>

<div class={'flex items-center ' + className}>
	<div class="relative h-2 w-full rounded-full bg-gray-200">
		<div
			class="absolute h-2 rounded-full bg-gray-900"
			style="width: {((value - min) / (max - min)) * 100}%"
		></div>
		<input
			type="range"
			class="absolute inset-0 h-2 w-full appearance-none bg-transparent focus:outline-none [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-gray-900"
			bind:value
			{min}
			{max}
			{step}
			{oninput}
		/>
	</div>
</div>
