<script lang="ts">
  import { Switch } from "$lib/components/ui/switch";
  import { Slider } from "$lib/components/ui/slider";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Badge } from "$lib/components/ui/badge";
  import * as Card from "$lib/components/ui/card";
  import {
    Briefcase,
    Flag,
    Gamepad2,
    ListChecks,
    Plus,
    CirclePlus,
    Shield,
    SlidersVertical,
    X,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let activeWindowTracking = $state(true);
  let idleTimeout = $state([5]);
  let localStorageOnly = $state(true);
  let dataEncryption = $state(true);
  let hardwareAcceleration = $state(false);

  let excludedApps = $state(["Spotify.exe", "Calculator.exe"]);
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

  function removeApp(app: string) {
    excludedApps = excludedApps.filter((a) => a !== app);
  }
</script>

<main class="min-h-screen bg-background p-4">
  <div
    class="mx-auto mt-4 w-full max-w-screen-2xl flex flex-col gap-6 lg:flex-row"
  >
    <!-- Main Content -->
    <div class="flex-1 space-y-6">
      <header class="space-y-1">
        <h1 class="text-2xl font-semibold">Settings</h1>
        <p class="text-sm text-muted-foreground">
          Configure your tracking preferences, privacy options, and application
          behavior.
        </p>
      </header>

      <div class="grid gap-6 lg:grid-cols-2">
        <!-- Tracking Rules Panel -->
        <Card.Root>
          <Card.Header>
            <Card.Title class="flex items-center gap-2">
              <ListChecks class="h-4 w-4 text-primary" />
              Tracking Rules
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-4 text-sm">
            <!-- Setting Item -->
            <div class="flex items-start justify-between gap-4">
              <p class="flex-1">
                <span class="font-semibold block">Active Window Tracking</span>
                <span class="text-muted-foreground block"
                  >Log time only for the application currently in focus.</span
                >
              </p>
              <Switch bind:checked={activeWindowTracking} />
            </div>

            <!-- Setting Item -->
            <div class="space-y-2">
              <div class="flex items-start justify-between gap-4">
                <p class="flex-1">
                  <span class="font-semibold block">Idle Timeout</span>
                  <span class="text-muted-foreground block"
                    >Stop tracking after a period of no mouse or keyboard input.</span
                  >
                </p>
                <Badge variant="secondary">{idleTimeout[0]} min</Badge>
              </div>
              <Slider
                type="multiple"
                bind:value={idleTimeout}
                max={60}
                min={1}
                step={1}
              />
              <div class="flex justify-between text-xs text-muted-foreground">
                <span>1m</span>
                <span>15m</span>
                <span>30m</span>
                <span>60m</span>
              </div>
            </div>

            <!-- Setting Item -->
            <div class="space-y-2">
              <p>
                <span class="font-semibold block">Excluded Applications</span>
                <span class="text-muted-foreground block"
                  >Applications listed here will not be tracked.</span
                >
              </p>
              <div class="flex flex-wrap gap-2 rounded-md border p-3">
                {#each excludedApps as app}
                  <Badge variant="outline" class="gap-1">
                    {app}
                    <Button
                      variant="ghost"
                      size="icon"
                      onclick={() => removeApp(app)}
                      class="h-4 w-4 text-muted-foreground hover:text-destructive"
                    >
                      <X class="h-3.5 w-3.5" />
                    </Button>
                  </Badge>
                {/each}
                <Button variant="outline" size="sm">
                  <Plus class="h-4 w-4" />
                  Add App
                </Button>
              </div>
            </div>
          </Card.Content>
        </Card.Root>

        <!-- Privacy Panel -->
        <Card.Root>
          <Card.Header>
            <Card.Title class="flex items-center gap-2">
              <Shield class="h-4 w-4 text-amber-300" />
              Privacy & Data
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-4 text-sm">
            <div class="flex items-start justify-between gap-4">
              <p class="flex-1">
                <span class="font-semibold block">Local Storage Only</span>
                <span class="text-muted-foreground block"
                  >Keep all tracking data strictly on this device.</span
                >
              </p>
              <Switch bind:checked={localStorageOnly} />
            </div>
            <div class="flex items-start justify-between gap-4">
              <p class="flex-1">
                <span class="font-semibold block">Data Encryption</span>
                <span class="text-muted-foreground block"
                  >Encrypt local database using AES-256.</span
                >
              </p>
              <Switch bind:checked={dataEncryption} />
            </div>
            <div class="space-y-2 border-t pt-3">
              <h3 class="font-semibold">Data Management</h3>
              <div class="flex gap-2">
                <Button variant="outline" class="flex-1">Export Data</Button>
                <Button variant="outline" class="flex-1 text-destructive"
                  >Clear History</Button
                >
              </div>
            </div>
          </Card.Content>
        </Card.Root>

        <!-- Goals Panel -->
        <Card.Root>
          <Card.Header>
            <Card.Title class="flex items-center gap-2">
              <Flag class="h-4 w-4 text-secondary" />
              Daily Goals & Limits
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-3 text-sm">
            <div
              class="flex items-center justify-between gap-4 rounded-md border p-3"
            >
              <div class="flex items-center gap-3">
                <span
                  class="grid h-9 w-9 place-items-center rounded-full bg-primary/20 text-primary"
                >
                  <Briefcase class="h-4 w-4" />
                </span>
                <p>
                  <span class="font-semibold block">Productive Work</span>
                  <span class="text-muted-foreground block"
                    >Target minimum active time</span
                  >
                </p>
              </div>
              <div class="flex items-center gap-2">
                <Input class="w-20 text-center" value="06:00" />
                <span class="text-muted-foreground">hrs</span>
              </div>
            </div>

            <div
              class="flex items-center justify-between gap-4 rounded-md border p-3"
            >
              <div class="flex items-center gap-3">
                <span
                  class="grid h-9 w-9 place-items-center rounded-full bg-destructive/20 text-destructive"
                >
                  <Gamepad2 class="h-4 w-4" />
                </span>
                <p>
                  <span class="font-semibold block">Entertainment Limit</span>
                  <span class="text-muted-foreground block"
                    >Maximum allowed time</span
                  >
                </p>
              </div>
              <div class="flex items-center gap-2">
                <Input class="w-20 text-center" value="02:00" />
                <span class="text-muted-foreground">hrs</span>
              </div>
            </div>

            <Button variant="outline">
              <CirclePlus class="h-4 w-4" />
              Create New Goal
            </Button>
          </Card.Content>
        </Card.Root>

        <!-- General Panel -->
        <Card.Root>
          <Card.Header>
            <Card.Title class="flex items-center gap-2">
              <SlidersVertical class="h-4 w-4 text-muted-foreground" />
              General
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-4 text-sm">
            <div class="flex items-start justify-between gap-4">
              <p class="flex-1">
                <span class="font-semibold block">Launch on Startup</span>
                <span class="text-muted-foreground block"
                  >Start tracking silently when OS boots.</span
                >
              </p>
              <Switch bind:checked={autostartEnabled} />
            </div>
            <div class="flex items-start justify-between gap-4">
              <p class="flex-1">
                <span class="font-semibold block">Hardware Acceleration</span>
                <span class="text-muted-foreground block"
                  >Use GPU for smoother UI rendering.</span
                >
              </p>
              <Switch bind:checked={hardwareAcceleration} />
            </div>
          </Card.Content>
        </Card.Root>

        <!-- Save Actions -->
        <div class="flex justify-end gap-2 lg:col-span-2">
          <Button variant="ghost">Discard</Button>
          <Button>Save Changes</Button>
        </div>
      </div>
    </div>
  </div>
</main>
