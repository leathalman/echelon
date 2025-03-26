<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import * as Card from '$lib/components/ui/card';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
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
	import CircleAlert from 'lucide-svelte/icons/circle-alert';
	import Cookies from 'js-cookie';


	let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
		$props();

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, validateForm } = form;

	let loginFailed = $state(false);
	let loading = $state(false);

	async function handleLogin() {
		const formValidation = await validateForm();

		if (formValidation.valid) {
			loading = true

			try {
				const loginResponse = await login($formData.email.toLowerCase(), $formData.password);
				if (loginResponse.success) {
					loginFailed = false;
					Cookies.set('onboarding_complete', true);
					await goto('/chat');
					loading = false
				} else {
					loginFailed = true;
					loading = false
				}
			} catch (error) {
				console.log(error);
				loginFailed = true;
				loading = false
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
				<div class="flex flex-col justify-start items-start space-y-4">
					<Button href="/forgot-password" variant="link" class="m-0 p-0">Forgot Password?</Button>
					{#if loginFailed}
						<Alert.Root variant="destructive">
							<CircleAlert class="size-4" />
							<Alert.Title>Authentication Error</Alert.Title>
							<Alert.Description
							>Your username or password was incorrect.
							</Alert.Description>
						</Alert.Root>
					{/if}
				</div>
				{#if loading}
					<Form.Button
						class="w-full mt-6" disabled>Loading...
					</Form.Button>
				{:else}
					<Form.Button
						class="w-full mt-6" onclick={handleLogin}>Continue
					</Form.Button>
				{/if}
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