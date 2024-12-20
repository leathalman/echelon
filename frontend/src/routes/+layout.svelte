<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import '../app.css';

	let { children } = $props();
</script>

{@render children?.()}

<!--<Sidebar.Provider>-->
<!--	<AppSidebar />-->
<!--	<main class="w-full m-2">-->
<!--		<Sidebar.Trigger />-->
<!--		{@render children?.()}-->
<!--	</main>-->
<!--</Sidebar.Provider>-->