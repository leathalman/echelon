<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import DarkModeSwitch from './dark-mode-switch.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { Button } from '$lib/components/ui/button';
	import { useSidebar } from '$lib/components/ui/sidebar/context.svelte.js';

	let sidebar = useSidebar();
</script>

<div class="flex flex-row flex-growth justify-between p-2 h-14">
	<div class="flex flex-row items-center space-x-4">
		{#if !sidebar.open || sidebar.isMobile}
			<Button variant="outline" size="icon">
				<Sidebar.Trigger />
			</Button>
		{/if}
		<span class="text-lg font-semibold">EchelonGPT</span>
	</div>
	<DarkModeSwitch />
</div>