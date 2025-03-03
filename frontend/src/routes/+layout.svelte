<script>
	import '../app.css';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';

	// if (browser && !document.cookie.includes('auth_token')) {
	// 	goto('/');
	// }

	let { children } = $props();
</script>

{@render children?.()}