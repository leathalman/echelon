<script>
	import '../app.css';
	import { conversations } from '$lib/model/conversations.svelte';

	let { children, data } = $props();

	// save conversations into rune for use when updating ui directly
	conversations.value = data.conversations;
	console.log('Updated conversations from root layout');
</script>

{@render children?.()}