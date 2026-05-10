<script lang="ts">
    import * as Menubar from "$lib/components/ui/menubar/index.js";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import ThemeModeToggle from "$lib/components/ThemeModeToggle.svelte";
    import { goto } from "$app/navigation";

    let bookmarks = $state(false);
    let fullUrls = $state(true);
    let profileRadioValue = $state("benoit");

    let autostartEnabled = $state(false);
    let autostartLoaded = $state(false);
    let lastSyncedAutostart = $state<boolean | null>(null);

    onMount(async () => {
        try {
            const status = await invoke<boolean>("get_autostart_status");
            autostartEnabled = status;
            lastSyncedAutostart = status;
        } catch (error) {
            console.error("Failed to get autostart status:", error);
        } finally {
            autostartLoaded = true;
        }
    });

    $effect(() => {
        if (!autostartLoaded || autostartEnabled === lastSyncedAutostart) {
            return;
        }

        const nextValue = autostartEnabled;
        void (async () => {
            try {
                const status = await invoke<boolean>("set_autostart", {
                    enabled: nextValue,
                });
                autostartEnabled = status;
                lastSyncedAutostart = status;
            } catch (error) {
                console.error("Failed to update autostart:", error);
                if (lastSyncedAutostart !== null) {
                    autostartEnabled = lastSyncedAutostart;
                }
            }
        })();
    });
    
</script>

<Menubar.Root>
    <Menubar.Menu>
        <Menubar.Trigger>File</Menubar.Trigger>
        <Menubar.Content>
            <Menubar.Item>
                New Tab <Menubar.Shortcut>⌘T</Menubar.Shortcut>
            </Menubar.Item>
            <Menubar.Item>
                New Window <Menubar.Shortcut>⌘N</Menubar.Shortcut>
            </Menubar.Item>
            <Menubar.Item>New Incognito Window</Menubar.Item>
            <Menubar.Separator />
            <Menubar.Sub>
                <Menubar.SubTrigger>Share</Menubar.SubTrigger>
                <Menubar.SubContent>
                    <Menubar.Item>Email link</Menubar.Item>
                    <Menubar.Item>Messages</Menubar.Item>
                    <Menubar.Item>Notes</Menubar.Item>
                </Menubar.SubContent>
            </Menubar.Sub>
            <Menubar.Separator />
            <Menubar.Item>
                Print... <Menubar.Shortcut>⌘P</Menubar.Shortcut>
            </Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>
    
    <Menubar.Menu>
        <Menubar.Trigger>Options</Menubar.Trigger>
        <Menubar.Content>
            <Menubar.CheckboxItem bind:checked={autostartEnabled}>
                Enable Autostart
            </Menubar.CheckboxItem>
            <Menubar.Separator />
            <Menubar.Item inset onclick={() => goto('/settings')}>Configuration</Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>

    <Menubar.Menu>
        <Menubar.Trigger>View</Menubar.Trigger>
        <Menubar.Content>
            <Menubar.CheckboxItem bind:checked={bookmarks}
                >Always Show Bookmarks Bar
            </Menubar.CheckboxItem>
            <Menubar.CheckboxItem bind:checked={fullUrls}>
                Always Show Full URLs
            </Menubar.CheckboxItem>
            <Menubar.CheckboxItem bind:checked={autostartEnabled}>
                Enable Autostart
            </Menubar.CheckboxItem>
            <Menubar.Separator />
            <Menubar.Item inset>
                Reload <Menubar.Shortcut>⌘R</Menubar.Shortcut>
            </Menubar.Item>
            <Menubar.Item inset>
                Force Reload <Menubar.Shortcut>⇧⌘R</Menubar.Shortcut>
            </Menubar.Item>
            <Menubar.Separator />
            <Menubar.Item inset>Toggle Fullscreen</Menubar.Item>
            <Menubar.Separator />
            <Menubar.Item inset>Hide Sidebar</Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>

    <Menubar.Menu>
        <Menubar.Trigger>Profiles</Menubar.Trigger>
        <Menubar.Content>
            <Menubar.RadioGroup bind:value={profileRadioValue}>
                <Menubar.RadioItem value="andy">Andy</Menubar.RadioItem>
                <Menubar.RadioItem value="benoit">Benoit</Menubar.RadioItem>
                <Menubar.RadioItem value="Luis">Luis</Menubar.RadioItem>
            </Menubar.RadioGroup>
            <Menubar.Separator />
            <Menubar.Item inset>Edit...</Menubar.Item>
            <Menubar.Separator />
            <Menubar.Item inset>Add Profile...</Menubar.Item>
        </Menubar.Content>
    </Menubar.Menu>
    <ThemeModeToggle class="ml-auto" />
</Menubar.Root>
