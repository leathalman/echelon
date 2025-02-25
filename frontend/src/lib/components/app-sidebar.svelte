<script lang="ts">
	import BookText from 'lucide-svelte/icons/book-text';
	import UserRound from 'lucide-svelte/icons/user-round';
	import Settings from 'lucide-svelte/icons/settings';

	import Search from 'lucide-svelte/icons/search';
	import CirclePlus from 'lucide-svelte/icons/circle-plus';

	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { MenuItem } from '$lib/components/ui/sidebar/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { onMount } from 'svelte';
	import ConversationsResponse, { type Conversation } from '$lib/api/conversations';


	// Menu items.
	const items = [
		{
			title: 'Academic Profile',
			url: '/maintenance',
			icon: BookText
		},
		{
			title: 'Account',
			url: '/maintenance',
			icon: UserRound
		},
		{
			title: 'Settings',
			url: '/maintenance',
			icon: Settings
		}
	];

	let conversations = $state<Conversation[]>([]);

	async function fetchConversations() {
		try {
			const response = await fetch('http://localhost:8000/api/conversations');
			const data = await response.json();
			const conversationsResponse = new ConversationsResponse(data);
			conversations = conversationsResponse.getConversationsSortedByDate() || [];
		} catch (err) {
			console.error(`Failed to fetch conversations: ${err}`);
		}
	}

	onMount(() => {
		fetchConversations();
	});
</script>

<Sidebar.Root class="pl-0.5">
	<Sidebar.Header>
		<div class="flex flex-row justify-between items-center px-1 py-2 h-14">
			<Sidebar.Trigger variant="outline" class="h-9 w-9" />
			<div class="flex flex-row space-x-3">
				<Button variant="outline" class="h-9 w-9">
					<Search />
				</Button>
				<Button variant="default" size="icon" class="h-9 w-9">
					<CirclePlus />
				</Button>
			</div>
		</div>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>General</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item (item.title)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href={item.url} {...props}>
										<item.icon />
										<span class="text-sm">{item.title}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Chat History</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each conversations as conversation}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href={`/chat/${conversation.id}`} {...props}>
										<span class="text-sm">{conversation.title}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<div class="flex flex-row space-x-3 px-1 py-2 rounded">
			<Avatar.Root>
				<Avatar.Image src="https://avatars.githubusercontent.com/u/11728008?v=4" alt="@shadcn" />
				<Avatar.Fallback>CN</Avatar.Fallback>
			</Avatar.Root>
			<div class="flex flex-col leading-tight">
				<Label class="font-semibold text-base">Harrison Leath</Label>
				<Label class="text-xs">hleath@me.com</Label>
			</div>
		</div>
	</Sidebar.Footer>
</Sidebar.Root>