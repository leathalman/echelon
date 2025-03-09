<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card';
	import { formSchema, type FormSchema } from './login_schema';
	import {
		type SuperValidated,
		type Infer,
		superForm
	} from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { login } from '$lib/api/auth';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { page } from '$app/state';


	let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
		$props();

	let showLoginFailed: boolean = $state(false);

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, errors } = form;

	// TODO: dont allow this unless both fields are correctly filled out
	// TODO: replace alert with something better
	async function handleLogin() {
		try {
			showLoginFailed = false;

			// Attempt the login API call
			await login($formData.email, $formData.password);

			// Redirect user upon successful login
			await goto('/chat');
		} catch (error: unknown) {
			// Log the error for debugging purposes
			console.error(error);

			// Handle different error scenarios
			if (error instanceof Response) {
				// Handle HTTP errors returned by the `login` API
				if (error.status === 401) {
					// Unauthorized: Incorrect email or password
					showLoginFailed = true;
				} else if (error.status >= 500) {
					// Server-side error
					alert('An error occurred on the server. Please try again later.');
				} else {
					// Other HTTP response scenarios
					alert('An unexpected error occurred. Please try again.');
				}
			} else {
				// Fallback for unknown errors, such as network or parse errors
				alert('A network error occurred. Please check your internet connection and try again.');
			}
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
				<div class="flex flex-col justify-start items-start">
					{#if showLoginFailed}
						<span class="text-sm text-destructive font-medium">Incorrect email or password</span>
					{/if}
					<Button href="/forgot-password" variant="link" class="m-0 p-0">Forgot Password?</Button>
				</div>
				<Form.Button
					class="w-full mt-6" onclick={handleLogin}>Continue
				</Form.Button>
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