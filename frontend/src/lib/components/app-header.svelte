<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import DarkModeSwitch from './dark-mode-switch.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
</script>

<div class="flex flex-row flex-growth justify-between items-center p-2 h-14">
	<div class="flex flex-col ">
		<div class="text-xs font-light">Student</div>
		<div class="text-lg font-semibold leading-5">EchelonGPT</div>
	</div>
	<DarkModeSwitch />
</div>