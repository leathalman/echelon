<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import AppHeader from '$lib/components/app-header.svelte';

	let { children } = $props();
</script>

<ModeWatcher />

<Sidebar.Provider>
	<AppSidebar />
	<main class="flex flex-col flex-grow px-2 py-2">
		<AppHeader />
		{@render children?.()}
	</main>
</Sidebar.Provider>

<!--{@render children?.()}-->