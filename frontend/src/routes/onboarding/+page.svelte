<script lang="ts">
	import * as Card from '$lib/components/ui/card/index';
	import * as Form from '$lib/components/ui/form/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { formSchema, type FormSchema } from './onboarding_schema';
	import {
		type SuperValidated,
		type Infer,
		superForm
	} from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { login, signup } from '$lib/api/auth';
	import { goto } from '$app/navigation';
	import CircleAlert from 'lucide-svelte/icons/circle-alert';
	import { onMount } from 'svelte';
	import { updateUser } from '$lib/api/client';
	import Cookies from 'js-cookie';

	let { data }: { data: { form: SuperValidated<Infer<FormSchema>>, authToken: string | undefined } } =
		$props();

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, validateForm } = form;

	let signupFailed = $state(false);

	async function handleCompleteOnboarding() {
		const formValidation = await validateForm();

		if (formValidation.valid) {
			try {
				if (data.authToken) {
					const signupResult = await updateUser(data.authToken, $formData.student_id, $formData.first_name, $formData.last_name, $formData.university.toLowerCase().replaceAll(' ', '_'));

					if (signupResult.error) {
						signupFailed = true;
					} else {
						Cookies.set('onboarding_complete', true)
						await goto('/chat');
					}

				} else {
					signupFailed = true;
				}
			} catch (error) {
				console.error('Unexpected error during onboarding:', error);
			}
		}
	}
</script>

<div class="flex h-full w-full justify-center items-center bg-secondary">
	<Card.Root class="w-[380px]">
		<Card.Header class="flex">
			<Card.Title class="text-2xl">Welcome to Echelon</Card.Title>
			<Card.Description class="text-md">First things first, tell us a bit about yourself.</Card.Description>
		</Card.Header>
		<Card.Content>
			<form method="POST" use:enhance>
				<div class="grid grid-cols-2 gap-4">
					<Form.Field {form} name="first_name">
						<Form.Control>
							{#snippet children({ props })}
								<Form.Label>First Name</Form.Label>
								<Input {...props} bind:value={$formData.first_name} />
							{/snippet}
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Form.Field {form} name="last_name">
						<Form.Control>
							{#snippet children({ props })}
								<Form.Label>Last Name</Form.Label>
								<Input {...props} bind:value={$formData.last_name} />
							{/snippet}
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
				</div>
				<Form.Field {form} name="university">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label>University</Form.Label>
							<Select.Root
								type="single"
								bind:value={$formData.university}
								name={props.name}
							>
								<Select.Trigger {...props}>
									{$formData.university
										? $formData.university
										: "Choose your university"}
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="Texas Christian University" label="Texas Christian University" />
								</Select.Content>
							</Select.Root>
						{/snippet}
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>
				<Form.Field {form} name="student_id">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label>Student ID</Form.Label>
							<Input {...props} bind:value={$formData.student_id} />
						{/snippet}
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>
				{#if signupFailed}
					<Alert.Root variant="destructive" class="mt-6">
						<CircleAlert class="size-4" />
						<Alert.Title>Signup Error</Alert.Title>
						<Alert.Description
						>Unable to create your account. Please try again later.
						</Alert.Description>
					</Alert.Root>
				{/if}
				<Form.Button
					class="w-full mt-6" onclick={handleCompleteOnboarding}>Continue
				</Form.Button>
			</form>
		</Card.Content>
		<Card.Footer>
		</Card.Footer>
	</Card.Root>
</div>