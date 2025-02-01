<script>
    let { class: className = "", children, onclick = null } = $props();
</script>

<button
	class={"flex h-10 w-10 items-center justify-center rounded-md border border-transparent bg-transparent text-sm font-medium text-gray-800 transition-colors hover:bg-gray-200 hover:text-gray-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-300 disabled:pointer-events-none disabled:opacity-50 " + className}
	aria-label="Icon button"
	{onclick}
>
	{@render children()}
</button>
