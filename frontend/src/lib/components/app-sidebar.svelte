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
	import { page } from '$app/state';

	let { data } = $props();
	let currentConversationId = $derived(page.params.conversation_id)

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
			<Sidebar.GroupLabel>Chat History</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each conversations.value as conversation}
						<Sidebar.MenuItem>
							{#if currentConversationId === conversation.id.toString()}
								<Sidebar.MenuButton class="bg-sidebar-accent text-sidebar-accent-foreground">
									{#snippet child({ props })}
											<a href={`/chat/${conversation.id}`} {...props}>
												<span class="text-sm">{conversation.title}</span>
											</a>
									{/snippet}
								</Sidebar.MenuButton>
							{:else}
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href={`/chat/${conversation.id}`} {...props}>
											<span class="text-sm">{conversation.title}</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							{/if}
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

