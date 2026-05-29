<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { Minus, SquareIcon, X } from "@lucide/svelte";
  import Button from "./ui/button/button.svelte";

  const appWindow = getCurrentWindow();

  const handleMinimize = () => appWindow.minimize();
  const handleToggleMaximize = () => appWindow.toggleMaximize();
  const handleClose = () => appWindow.close();
</script>

<header class="titlebar">
  <div class="titlebar-drag" data-tauri-drag-region>
    <Sidebar.Trigger class="titlebar-sidebar-trigger" />
    <span class="titlebar-title font-bold" data-tauri-drag-region>Activity Tracker</span>
  </div>
  <div class="titlebar-controls">
    <Button
      class="titlebar-button"
      type="button"
      aria-label="Minimize"
      onclick={handleMinimize}
    >
      <Minus class="size-4 translate-y-px" strokeWidth={3} />
    </Button>
    <Button
      class="titlebar-button"
      type="button"
      aria-label="Maximize"
      onclick={handleToggleMaximize}
    >
      <SquareIcon class="size-3" strokeWidth={3} />
    </Button>
    <Button
      class="titlebar-button titlebar-button-close"
      type="button"
      aria-label="Close"
      onclick={handleClose}
    >
      <X class="size-4" strokeWidth={3} />
    </Button>
  </div>
</header>
