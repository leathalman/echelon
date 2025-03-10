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
	import { goto } from '$app/navigation';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { conversations } from '$lib/model/conversations.svelte';
	import {
		BadgeCheck,
		ChevronsUpDown,
		LogOut
	} from 'lucide-svelte';
	import { logout } from '$lib/api/auth';

	let { data } = $props();

	const items = [
		{
			title: 'Special Chat 1'
			// url: '/maintenance',
			// icon: BookText
		},
		{
			title: 'Special Chat 2'
			// url: '/maintenance',
			// icon: UserRound
		},
		{
			title: 'Special Chat 3'
			// url: '/maintenance',
			// icon: Settings
		}
	];

	function handleNewChat() {
		goto('/chat');
	}

	function handleLogout() {
		logout();
		goto('/');
	}
</script>

<Sidebar.Root class="pl-0.5">
	<Sidebar.Header>
		<div class="flex flex-row justify-between items-center px-1 py-2 h-14">
			<Sidebar.Trigger variant="outline" class="h-9 w-9" />
			<div class="flex flex-row space-x-3">
				<Button variant="outline" class="h-9 w-9">
					<Search />
				</Button>
				<Button variant="default" size="icon" class="h-9 w-9" onclick={handleNewChat}>
					<CirclePlus />
				</Button>
			</div>
		</div>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Starred</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each items as item (item.title)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								<span class="text-sm">{item.title}</span>
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
					{#each conversations.value as conversation}
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
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Sidebar.MenuButton
								size="lg"
								class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
								{...props}
							>
								<Avatar.Root class="h-8 w-8 rounded-lg">
									<!--									<Avatar.Image src={user.avatar} alt={user.name} />-->
									<Avatar.Fallback
										class="rounded-lg">{data.user.first_name.at(0)}{data.user.last_name.at(0)}</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-left text-sm leading-tight">
									<span class="truncate font-semibold">{data.user.first_name} {data.user.last_name}</span>
									<span class="truncate text-xs">{data.user.email}</span>
								</div>
								<ChevronsUpDown class="ml-auto size-4" />
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						class="w-[--bits-dropdown-menu-anchor-width] min-w-56 rounded-lg"
						side={data.isMobile ? "bottom" : "right"}
						align="end"
						sideOffset={4}
					>
						<DropdownMenu.Label class="p-0 font-normal">
							<div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
								<Avatar.Root class="h-8 w-8 rounded-lg">
									<!--									<Avatar.Image src={user.avatar} alt={user.name} />-->
									<Avatar.Fallback
										class="rounded-lg">{data.user.first_name.at(0)}{data.user.last_name.at(0)}</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-left text-sm leading-tight">
									<span class="truncate font-semibold">{data.user.first_name} {data.user.last_name}</span>
									<span class="truncate text-xs">{data.user.email}</span>
								</div>
							</div>
						</DropdownMenu.Label>
						<DropdownMenu.Separator />
						<DropdownMenu.Group>
							<DropdownMenu.Item>
								<BadgeCheck />
								Account
							</DropdownMenu.Item>
						</DropdownMenu.Group>
						<DropdownMenu.Separator />
						<DropdownMenu.Item onclick={handleLogout}>
							<LogOut />
							Log out
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
</Sidebar.Root>

