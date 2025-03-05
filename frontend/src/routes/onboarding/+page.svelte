<script lang="ts">
	import { type NewUser, newUserState } from '$lib/state/new-user.svelte';
	import * as Card from '$lib/components/ui/card/index';
	import * as Form from '$lib/components/ui/form/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
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

	let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
		$props();

	const form = superForm(data.form, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance } = form;

	// TODO: dont allow this unless all fields are correctly filled out
	// TODO: remove alerts for something better
	async function handleCompleteOnboarding() {
		try {
			// Populate new user state (signup payload)
			newUserState.first_name = $formData.first_name;
			newUserState.last_name = $formData.last_name;
			newUserState.university = $formData.university.toLowerCase().replaceAll(' ', '_');
			newUserState.student_id = $formData.student_id;

			// Ensure signup completes successfully
			const signupResult = await signup(newUserState);
			if (!signupResult || signupResult.error) {
				// Handle signup error
				console.error('Signup failed:', signupResult?.error || 'Unknown error occurred');
				alert('Signup failed. Please try again.');
				return; // Exit early if signup fails
			}

			// Ensure login completes successfully
			const loginResult = await login(newUserState.email, newUserState.password);
			if (!loginResult || loginResult.error) {
				// Handle login error
				console.error('Login failed:', loginResult?.error || 'Unknown error occurred');
				alert('Login failed. Please try again.');
				return; // Exit early if login fails
			}

			// Navigate to the chat page only after successful signup and login
			await goto('/chat');
		} catch (error) {
			// Handle unexpected errors
			console.error('Unexpected error during onboarding:', error);
			alert('Something went wrong. Please try again later.');
		}
	}

	$effect(() => {
		$inspect(newUserState);
	});
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
				<Form.Button
					disabled={true}
					class="w-full mt-6" onclick={handleCompleteOnboarding}>Continue
				</Form.Button>
			</form>
		</Card.Content>
		<Card.Footer>
		</Card.Footer>
	</Card.Root>
</div>