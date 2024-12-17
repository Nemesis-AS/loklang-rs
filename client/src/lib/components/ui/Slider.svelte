<script>
    let { value = 50, min = 0, max = 100, step = 1, class: className = "" } = $props();
  
    const handleInput = (/** @type {{ target: EventTarget; }} */ event) => {
      value = event.target.value;
    };
  </script>
  
  <div class={"flex items-center " + className}>
    <div class="relative w-full h-2 bg-gray-200 rounded-full">
      <div
        class="absolute h-2 bg-gray-900 rounded-full"
        style="width: {((value - min) / (max - min)) * 100}%"
      ></div>
      <input
        type="range"
        class="absolute inset-0 h-2 w-full appearance-none bg-transparent [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-gray-900 focus:outline-none"
        bind:value
        min={min}
        max={max}
        step={step}
        oninput={handleInput}
      />
    </div>
  </div>
  