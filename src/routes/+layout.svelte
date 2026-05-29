<script lang="ts">
	import './layout.css';
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import AppSidebar from "$lib/components/sidebar/app-sidebar.svelte";
	import TitleBar from "$lib/components/TitleBar.svelte";
 	import { ModeWatcher } from "mode-watcher";
	const { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar />
<Sidebar.Inset>
		<TitleBar />
		{@render children()}
	</Sidebar.Inset>
</Sidebar.Provider>
