<script lang="ts">
	import BookText from 'lucide-svelte/icons/book-text';
	import UserRound from 'lucide-svelte/icons/user-round';
	import Settings from 'lucide-svelte/icons/settings';

	import PanelLeft from 'lucide-svelte/icons/panel-left';
	import Search from 'lucide-svelte/icons/search';
	import CirclePlus from 'lucide-svelte/icons/circle-plus';

	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { MenuItem } from '$lib/components/ui/sidebar/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	// Menu items.
	const items = [
		{
			title: 'Academic Profile',
			url: '/academic-profile',
			icon: BookText
		},
		{
			title: 'Account',
			url: '/account',
			icon: UserRound
		},
		{
			title: 'Settings',
			url: '/settings',
			icon: Settings
		}
	];
	const chatHistory = [
		{
			title: 'Lorem ipsum dolor sit amet, consectetur adipiscing',
			url: '/chat-1'
		},
		{
			title: 'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex',
			url: '/chat-2'
		},
		{
			title: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur',
			url: '/chat-3'
		},
		{
			title: 'Est eros aenean sapien vestibulum odio lobortis. Augue ipsum conubia molestie posuere potenti donec.',
			url: '/chat-4'
		},
		{
			title: 'Molestie fames magna aptent vivamus nostra litora sapien condimentum. Sit curabitur sociosqu inceptos iaculis lacinia',
			url: '/chat-5'
		},
		{
			title: 'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex',
			url: '/chat-2'
		},
		{
			title: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur',
			url: '/chat-3'
		},
		{
			title: 'Est eros aenean sapien vestibulum odio lobortis. Augue ipsum conubia molestie posuere potenti donec.',
			url: '/chat-4'
		},
		{
			title: 'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex',
			url: '/chat-2'
		},
		{
			title: 'Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur',
			url: '/chat-3'
		},
		{
			title: 'Est eros aenean sapien vestibulum odio lobortis. Augue ipsum conubia molestie posuere potenti donec.',
			url: '/chat-4'
		},
		{
			title: 'Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex',
			url: '/chat-2'
		}
	];
</script>

<Sidebar.Root class="pl-0.5">
	<Sidebar.Header>
		<div class="flex flex-row justify-between items-center px-1 py-2 h-14">
			<Button variant="outline" size="icon">
				<Sidebar.Trigger />
			</Button>
			<div class="flex flex-row space-x-3">
				<Button variant="outline" size="icon">
					<Search size={10} />
				</Button>
				<Button variant="default" size="icon">
					<CirclePlus size={10} />
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
					{#each chatHistory as chat}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href={chat.url} {...props}>
										<span class="text-sm">{chat.title}</span>
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
				<Avatar.Image src="https://github.com/shadcn.png" alt="@shadcn" />
				<Avatar.Fallback>CN</Avatar.Fallback>
			</Avatar.Root>
			<div class="flex flex-col leading-tight">
				<Label class="font-semibold text-base">Harrison Leath</Label>
				<Label class="text-xs">hleath@me.com</Label>
			</div>
		</div>
	</Sidebar.Footer>
</Sidebar.Root>