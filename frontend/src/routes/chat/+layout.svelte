<script lang="ts">
	import '../../app.css';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import AppHeader from '$lib/components/app-header.svelte';

	let { children } = $props();
</script>

<Sidebar.Provider>
	<AppSidebar />
	<main class="flex flex-col flex-grow">
		<AppHeader />
		{@render children?.()}
	</main>
</Sidebar.Provider>