<script>
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
</script>

<div class="flex flex-col h-full w-full items-center justify-center bg-secondary">
	<Card.Root class="w-[380px]">
		<Card.Header class="flex">
			<Card.Title class="text-2xl">Welcome to Echelon</Card.Title>
			<Card.Description class="text-md">Your AI Academic Advisor</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="flex flex-col space-y-4">
				<Button href="/login">Login</Button>
				<Button href="/signup" variant="secondary">Sign Up</Button>
			</div>
		</Card.Content>
		<Card.Footer>
		</Card.Footer>
	</Card.Root>
</div>