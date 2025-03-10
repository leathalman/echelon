<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import * as Card from '$lib/components/ui/card';
	import * as Alert from "$lib/components/ui/alert/index.js";
	import { Input } from '$lib/components/ui/input/index.js';
	import { formSchema, type FormSchema } from './signup_schema';
	import {
		type SuperValidated,
		type Infer,
		superForm
	} from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { Button } from '$lib/components/ui/button';
	import CircleAlert from "lucide-svelte/icons/circle-alert";
	import { signup } from '$lib/api/auth';
	import { goto } from '$app/navigation';
	import Cookies from 'js-cookie';

	let { data }: { data: { form: SuperValidated<Infer<FormSchema>>, signUpFailed: boolean } } =
		$props();

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, validateForm} = form;

	let signUpFailed = $state(false)

	async function handleSignup() {
		const formValidation = await validateForm();

		if (formValidation.valid) {
			const response = await signup($formData.email, $formData.password);
			if (response.error) {
				signUpFailed = true
			} else {
				signUpFailed = false
				Cookies.set('onboarding_complete', false)
				await goto("/onboarding")
			}
		}
	}
</script>

<div class="flex h-full w-full justify-center items-center bg-secondary">
	<Card.Root class="w-[380px]">
		<Card.Header class="flex">
			<Card.Title class="text-2xl">Get started with Echelon</Card.Title>
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
					{#if signUpFailed}
						<Alert.Root variant="destructive" class="mt-8">
							<CircleAlert class="size-4" />
							<Alert.Title>Authentication Error</Alert.Title>
							<Alert.Description
							>Unable to complete sign up. Please try again later.</Alert.Description>
						</Alert.Root>
					{/if}
				<Form.Button class="w-full mt-6" onclick={handleSignup}>
					Continue
				</Form.Button>
			</form>
		</Card.Content>
		<Card.Footer>
			<div class="flex flex-row w-full items-center justify-center mt-2 text-sm text-muted-foreground">
				<span class="mr-1">Already have an account?</span>
				<Button href="/login" variant="link" class="m-0 p-0">Log in</Button>
			</div>
		</Card.Footer>
	</Card.Root>
</div>