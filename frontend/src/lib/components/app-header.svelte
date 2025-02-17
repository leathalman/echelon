<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';

	let sidebar = useSidebar();
</script>

<div class="fixed flex flex-row w-full items-center pl-4 pt-5 pb-3 bg-white">
	<div class="flex flex-row items-center space-x-3 min-h-9">
		{#if !sidebar.open || sidebar.isMobile}
			<Sidebar.Trigger variant="outline" />
		{/if}
		<span class="text-lg font-semibold">EchelonGPT</span>
	</div>
</div>