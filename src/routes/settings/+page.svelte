<script lang="ts">
  import MenuBar from "$lib/components/MenuBar.svelte";
  import { Switch } from "$lib/components/ui/switch";
  import { Slider } from "$lib/components/ui/slider";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Badge } from "$lib/components/ui/badge";
  import * as Card from "$lib/components/ui/card";
  import * as ToggleGroup from "$lib/components/ui/toggle-group";

  let activeWindowTracking = $state(true);
  let idleTimeout = $state([5]);
  let localStorageOnly = $state(true);
  let dataEncryption = $state(true);
  let launchOnStartup = $state(true);
  let hardwareAcceleration = $state(false);
  let themePreference = $state("system");

  let excludedApps = $state(["Spotify.exe", "Calculator.exe"]);

  function removeApp(app: string) {
    excludedApps = excludedApps.filter((a) => a !== app);
  }
</script>

<main class="min-h-screen bg-background p-4">
  <MenuBar />
  <div class="mx-auto mt-4 w-full max-w-screen-2xl space-y-6 lg:flex lg:gap-6 lg:space-y-0">
    <!-- SideNavBar -->
    <aside class="hidden lg:block lg:w-60 lg:shrink-0">
      <div class="mb-3 text-xs font-semibold text-muted-foreground uppercase">Pro Edition</div>
      <nav class="grid gap-2">
        <Button variant="ghost" class="justify-start gap-2" href="/">
          <span class="material-symbols-outlined" data-icon="dashboard">dashboard</span>
          Dashboard
        </Button>
        <Button variant="secondary" class="justify-start gap-2" href="/settings">
          <span class="material-symbols-outlined" data-icon="settings" style="font-variation-settings: 'FILL' 1;">settings</span>
          Settings
        </Button>
      </nav>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 space-y-6">
      <header class="space-y-1">
        <h1 class="text-2xl font-semibold">Settings</h1>
        <p class="text-sm text-muted-foreground">Configure your tracking preferences, privacy options, and application behavior.</p>
      </header>

      <div class="space-y-6 lg:flex lg:gap-6 lg:space-y-0">
        <!-- Left Column: Tracking & Goals -->
        <div class="flex-1 space-y-6">
          <!-- Tracking Rules Panel -->
          <Card.Root>
            <Card.Header>
              <div class="flex items-center gap-2">
                <span class="material-symbols-outlined text-primary" data-icon="rule">rule</span>
                <Card.Title>Tracking Rules</Card.Title>
              </div>
            </Card.Header>
            <Card.Content class="space-y-4">
                <!-- Setting Item -->
                <div class="flex items-start justify-between gap-4">
                  <div>
                    <h3 class="text-sm font-semibold">Active Window Tracking</h3>
                    <p class="text-sm text-muted-foreground">Log time only for the application currently in focus.</p>
                  </div>
                  <Switch bind:checked={activeWindowTracking} />
                </div>

                <!-- Setting Item -->
                <div class="space-y-2">
                  <div class="flex items-start justify-between gap-4">
                    <div>
                      <h3 class="text-sm font-semibold">Idle Timeout</h3>
                      <p class="text-sm text-muted-foreground">Stop tracking after a period of no mouse or keyboard input.</p>
                    </div>
                    <Badge variant="secondary">{idleTimeout[0]} min</Badge>
                  </div>
                  <Slider bind:value={idleTimeout} max={60} min={1} step={1} />
                  <div class="flex justify-between text-xs text-muted-foreground">
                    <span>1m</span>
                    <span>15m</span>
                    <span>30m</span>
                    <span>60m</span>
                  </div>
                </div>

                <!-- Setting Item -->
                <div class="space-y-2">
                  <div>
                    <h3 class="text-sm font-semibold">Excluded Applications</h3>
                    <p class="text-sm text-muted-foreground">Applications listed here will not be tracked.</p>
                  </div>
                  <div class="flex flex-wrap gap-2 rounded-md border p-3">
                    {#each excludedApps as app}
                      <Badge variant="outline" class="gap-1">
                        {app}
                        <Button variant="ghost" size="icon" onclick={() => removeApp(app)} class="h-4 w-4 text-muted-foreground hover:text-destructive">
                          <span class="material-symbols-outlined" data-icon="close">close</span>
                        </Button>
                      </Badge>
                    {/each}
                    <Button variant="outline" size="sm">
                      <span class="material-symbols-outlined" data-icon="add">add</span>
                      Add App
                    </Button>
                  </div>
                </div>
            </Card.Content>
          </Card.Root>

          <!-- Goals Panel -->
          <Card.Root>
            <Card.Header>
              <div class="flex items-center gap-2">
                <span class="material-symbols-outlined text-secondary" data-icon="flag">flag</span>
                <Card.Title>Daily Goals & Limits</Card.Title>
              </div>
            </Card.Header>
            <Card.Content class="space-y-3">
              <div class="flex items-center justify-between gap-4 rounded-md border p-3">
                <div class="flex items-center gap-3">
                  <div class="grid h-9 w-9 place-items-center rounded-full bg-primary/20 text-primary">
                    <span class="material-symbols-outlined" data-icon="work">work</span>
                  </div>
                  <div>
                    <h4 class="text-sm font-semibold">Productive Work</h4>
                    <p class="text-sm text-muted-foreground">Target minimum active time</p>
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <Input class="w-20 text-center" value="06:00" />
                  <span class="text-sm text-muted-foreground">hrs</span>
                </div>
              </div>

              <div class="flex items-center justify-between gap-4 rounded-md border p-3">
                <div class="flex items-center gap-3">
                  <div class="grid h-9 w-9 place-items-center rounded-full bg-destructive/20 text-destructive">
                    <span class="material-symbols-outlined" data-icon="sports_esports">sports_esports</span>
                  </div>
                  <div>
                    <h4 class="text-sm font-semibold">Entertainment Limit</h4>
                    <p class="text-sm text-muted-foreground">Maximum allowed time</p>
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <Input class="w-20 text-center" value="02:00" />
                  <span class="text-sm text-muted-foreground">hrs</span>
                </div>
              </div>

              <Button variant="outline">
                <span class="material-symbols-outlined" data-icon="add_circle">add_circle</span>
                Create New Goal
              </Button>
            </Card.Content>
          </Card.Root>
        </div>

        <!-- Right Column: Privacy & General -->
        <div class="space-y-6 lg:w-96 lg:shrink-0">
          <!-- Privacy Panel -->
          <Card.Root>
            <Card.Header>
              <div class="flex items-center gap-2">
                <span class="material-symbols-outlined text-amber-300" data-icon="shield">shield</span>
                <Card.Title>Privacy & Data</Card.Title>
              </div>
            </Card.Header>
            <Card.Content class="space-y-4">
              <div class="flex items-start justify-between gap-4">
                <div>
                  <h3 class="text-sm font-semibold">Local Storage Only</h3>
                  <p class="text-sm text-muted-foreground">Keep all tracking data strictly on this device.</p>
                </div>
                <Switch bind:checked={localStorageOnly} />
              </div>
              <div class="flex items-start justify-between gap-4">
                <div>
                  <h3 class="text-sm font-semibold">Data Encryption</h3>
                  <p class="text-sm text-muted-foreground">Encrypt local database using AES-256.</p>
                </div>
                <Switch bind:checked={dataEncryption} />
              </div>
              <div class="space-y-2 border-t pt-3">
                <h3 class="text-sm font-semibold">Data Management</h3>
                <div class="flex gap-2">
                  <Button variant="outline" class="flex-1">Export Data</Button>
                  <Button variant="outline" class="flex-1 text-destructive">Clear History</Button>
                </div>
              </div>
            </Card.Content>
          </Card.Root>

          <!-- General Panel -->
          <Card.Root>
            <Card.Header>
              <div class="flex items-center gap-2">
                <span class="material-symbols-outlined text-muted-foreground" data-icon="tune">tune</span>
                <Card.Title>General</Card.Title>
              </div>
            </Card.Header>
            <Card.Content class="space-y-4">
              <div class="space-y-2">
                <label class="text-sm font-semibold">Theme Preference</label>
                <ToggleGroup.Root type="single" bind:value={themePreference} class="gap-2" variant="outline">
                  <ToggleGroup.Item value="light" class="h-16 w-24 flex-col gap-1">
                    <span class="material-symbols-outlined text-lg" data-icon="light_mode">light_mode</span>
                    <span class="text-xs">Light</span>
                  </ToggleGroup.Item>
                  <ToggleGroup.Item value="dark" class="h-16 w-24 flex-col gap-1">
                    <span class="material-symbols-outlined text-lg" data-icon="dark_mode">dark_mode</span>
                    <span class="text-xs">Dark</span>
                  </ToggleGroup.Item>
                  <ToggleGroup.Item value="system" class="h-16 w-24 flex-col gap-1">
                    <span class="material-symbols-outlined text-lg" data-icon="desktop_windows">desktop_windows</span>
                    <span class="text-xs">System</span>
                  </ToggleGroup.Item>
                </ToggleGroup.Root>
              </div>

              <div class="flex items-start justify-between gap-4">
                <div>
                  <h3 class="text-sm font-semibold">Launch on Startup</h3>
                  <p class="text-sm text-muted-foreground">Start tracking silently when OS boots.</p>
                </div>
                <Switch bind:checked={launchOnStartup} />
              </div>
              <div class="flex items-start justify-between gap-4">
                <div>
                  <h3 class="text-sm font-semibold">Hardware Acceleration</h3>
                  <p class="text-sm text-muted-foreground">Use GPU for smoother UI rendering.</p>
                </div>
                <Switch bind:checked={hardwareAcceleration} />
              </div>
            </Card.Content>
          </Card.Root>

          <!-- Save Actions -->
          <div class="flex justify-end gap-2">
            <Button variant="ghost">Discard</Button>
            <Button>Save Changes</Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</main>
