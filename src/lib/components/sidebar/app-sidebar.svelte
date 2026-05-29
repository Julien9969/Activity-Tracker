<script lang="ts">
	import AudioWaveformIcon from "@lucide/svelte/icons/audio-waveform";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import CommandIcon from "@lucide/svelte/icons/command";
	import GalleryVerticalEndIcon from "@lucide/svelte/icons/gallery-vertical-end";
	import HouseIcon from "@lucide/svelte/icons/house";
	import Settings2Icon from "@lucide/svelte/icons/settings-2";
	import TimelineIcon from "@lucide/svelte/icons/timeline";
	import { page } from "$app/state";
	import NavMain from "./nav-main.svelte";
	import NavUser from "./nav-user.svelte";
	import TeamSwitcher from "./team-switcher.svelte";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import type { ComponentProps } from "svelte";
  	import Calendar from "$lib/components/Calendar.svelte";
	import { useSidebar } from "$lib/components/ui/sidebar/index.js";
    import { Calendar1Icon } from "@lucide/svelte";

  	const sidebar = useSidebar();

	const data = {
		user: {
			name: "shadcn",
			email: "m@example.com",
			avatar: "/avatars/shadcn.jpg",
		},
		teams: [
			{
				name: "Acme Inc",
				logo: GalleryVerticalEndIcon,
				plan: "Enterprise",
			},
			{
				name: "Acme Corp.",
				logo: AudioWaveformIcon,
				plan: "Startup",
			},
			{
				name: "Evil Corp.",
				logo: CommandIcon,
				plan: "Free",
			},
		],
		pages: [
			{
				title: "Dashboard",
				url: "/",
				icon: HouseIcon,
			},
			{
				title: "Timeline",
				url: "/timeline",
				icon: TimelineIcon,
			},
			{
				title: "Unused",
				url: "#",
				icon: BookOpenIcon,
			},
			{
				title: "Settings",
				url: "/settings",
				icon: Settings2Icon,
			},
		]
	};

	const pages = $derived(
		data.pages.map((item) => ({
			...item,
			isActive: item.url !== "#" && page.url.pathname === item.url,
		}))
	);

	let {
		ref = $bindable(null),
		collapsible = "icon",
		...restProps
	}: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
	<Sidebar.Header>
		<TeamSwitcher teams={data.teams} />
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain items={pages} />
		<Sidebar.Separator />
		{#if sidebar.state === "expanded"}
			<Calendar />
		{:else}
			<Sidebar.Group>
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton tooltipContent="Calendar">
							<Calendar1Icon />
							<span>Calendar</span>
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			</Sidebar.Group>
		{/if}
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser user={data.user} />
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
