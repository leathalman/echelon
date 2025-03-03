<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card';
	import { formSchema, type FormSchema } from '$lib/auth/schema';
	import {
		type SuperValidated,
		type Infer,
		superForm
	} from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { login } from '$lib/auth/auth';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';


	let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
		$props();

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance } = form;

	async function handleLogin() {
		try {
			await login($formData.email, $formData.password);
			goto('/chat');
		} catch (error) {
			console.error(error);
		}
	}
</script>

<div class="flex h-full w-full justify-center items-center bg-secondary">
	<Card.Root class="w-[380px]">
		<Card.Header class="flex">
			<Card.Title class="text-2xl">Welcome back to Echelon</Card.Title>
			<Card.Description class="text-md">Your AI Academic Advisor</Card.Description>
		</Card.Header>
		<Card.Content>
			<form method="POST" use:enhance>
				<Form.Field {form} name="email">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label>Email</Form.Label>
							<Input {...props} placeholder="" bind:value={$formData.email} />
						{/snippet}
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>
				<Form.Field {form} name="password" class="my-4">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label>Password</Form.Label>
							<Input {...props} placeholder="" type="password" bind:value={$formData.password} />
						{/snippet}
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>
				<Button href="/forgot-password" variant="link" class="m-0 p-0">Forgot Password?</Button>
				<Form.Button class="w-full mt-6" onclick={handleLogin}>Continue</Form.Button>
			</form>
		</Card.Content>
		<Card.Footer>
			<div class="flex flex-row w-full items-center justify-center mt-2 text-sm text-muted-foreground">
				<span class="mr-1">Don't have an account?</span>
				<Button href="/signup" variant="link" class="m-0 p-0">Sign up</Button>
			</div>
		</Card.Footer>
	</Card.Root>
</div>