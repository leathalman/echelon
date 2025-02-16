<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';

	let sidebar = useSidebar();
</script>

<div class="flex flex-row flex-growth justify-between p-2 h-14 ">
	<div class="flex flex-row items-center space-x-4">
		{#if !sidebar.open || sidebar.isMobile}
			<Sidebar.Trigger variant="outline" />
		{/if}
		<span class="text-lg font-semibold text-">EchelonGPT</span>
	</div>
</div>