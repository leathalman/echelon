<script lang="ts">
	import '../../app.css';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import AppHeader from '$lib/components/app-header.svelte';
	import { page } from '$app/state';

	let { children } = $props();
</script>

<Sidebar.Provider>
	<AppSidebar data={page.data} />
	<main class="flex flex-col flex-grow bg-background">
		<AppHeader />
		{@render children?.()}
	</main>
</Sidebar.Provider>