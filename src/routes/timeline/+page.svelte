<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Separator } from "$lib/components/ui/separator";
  import {
    CalendarDays,
    Clock,
    Code,
    Globe,
    MessageCircle,
    Moon,
    ZoomIn,
    ZoomOut,
  } from "@lucide/svelte";
  import CalendarInput from "$lib/components/Calendar-input.svelte";

  type IconComponent = typeof Code;

  type TimelineEntry = {
    time: string;
    app: string;
    detail: string;
    duration: string;
    icon: IconComponent;
    iconClass: string;
    iconBgClass: string;
    dotClass: string;
    borderClass: string;
    durationClass: string;
    isIdle?: boolean;
  };

  const timelineEntries: TimelineEntry[] = [
    {
      time: "09:00",
      app: "Visual Studio Code",
      detail: "project-alpha.ts - Editing",
      duration: "1h 45m",
      icon: Code,
      iconClass: "text-sky-400",
      iconBgClass: "bg-muted",
      dotClass: "bg-primary",
      borderClass: "border-primary/40",
      durationClass: "text-primary",
    },
    {
      time: "10:45",
      app: "Google Chrome",
      detail: "Stack Overflow - Research",
      duration: "22m",
      icon: Globe,
      iconClass: "text-amber-400",
      iconBgClass: "bg-muted",
      dotClass: "bg-amber-400",
      borderClass: "border-amber-400/40",
      durationClass: "text-amber-400",
    },
    {
      time: "11:07",
      app: "Visual Studio Code",
      detail: "api-routes.js - Editing",
      duration: "1h 10m",
      icon: Code,
      iconClass: "text-sky-400",
      iconBgClass: "bg-muted",
      dotClass: "bg-primary",
      borderClass: "border-primary/40",
      durationClass: "text-primary",
    },
    {
      time: "12:17",
      app: "Idle Time",
      detail: "System Locked",
      duration: "45m",
      icon: Moon,
      iconClass: "text-muted-foreground",
      iconBgClass: "bg-muted/60",
      dotClass: "bg-muted-foreground/60",
      borderClass: "border-muted-foreground/40",
      durationClass: "text-muted-foreground",
      isIdle: true,
    },
    {
      time: "13:02",
      app: "Discord",
      detail: "#engineering-team - Chatting",
      duration: "15m",
      icon: MessageCircle,
      iconClass: "text-indigo-400",
      iconBgClass: "bg-indigo-950/40",
      dotClass: "bg-indigo-400",
      borderClass: "border-indigo-400/40",
      durationClass: "text-indigo-400",
    },
  ];

  const dayLabel = "Today, Oct 26";
  const zoomLevel = 100;
  const currentIndicatorTop = "85%";
</script>

<main class="flex-1 bg-background">
  <div class="mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-6">
    <header
      class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between"
    >
      <div class="space-y-1">
        <h1 class="text-2xl font-semibold tracking-tight">Timeline</h1>
        <p class="text-sm text-muted-foreground">
          Track the blocks of focus, research, and breaks across your day.
        </p>
      </div>
      <CalendarInput />
    </header>

    <!-- <div
        class="inline-flex items-center gap-2 rounded-lg border bg-card p-1 shadow-sm"
      >
        <Button variant="ghost" size="icon-sm" aria-label="Zoom out">
          <ZoomOut class="size-4" />
        </Button>
        <Badge
          variant="outline"
          class="rounded-md px-2 py-1 text-xs text-muted-foreground"
        >
          {zoomLevel}%
        </Badge>
        <Button variant="ghost" size="icon-sm" aria-label="Zoom in">
          <ZoomIn class="size-4" />
        </Button>
      </div> -->

    <Card.Root class="flex min-h-130 flex-1 overflow-hidden">
      <Card.Header class="pb-3">
        <div class="flex flex-wrap items-center justify-between gap-3">
          <div>
            <Card.Title>Day Timeline</Card.Title>
            <Card.Description>
              Activity blocks ordered by start time.
            </Card.Description>
          </div>
          <Badge variant="secondary" class="gap-1">
            <Clock class="size-3" />
            {timelineEntries.length} entries
          </Badge>
        </div>
      </Card.Header>
      <Separator />
      <Card.Content class="relative flex flex-1 flex-col gap-4 pb-6 pt-4">
        <div
          class="flex border-b border-border/60 pb-2 text-xs font-medium text-muted-foreground"
        >
          <div class="w-20 shrink-0 text-right pr-4">Time</div>
          <div class="flex flex-1 justify-between px-4">
            <span>Application</span>
            <span>Duration</span>
          </div>
        </div>

        <div class="relative flex-1 overflow-y-auto pr-2">
          <div
            class="absolute left-[88px] top-0 h-full w-px bg-border/70"
          ></div>
          <div class="relative flex flex-col gap-4">
            {#each timelineEntries as entry}
              <div
                class={`group relative flex gap-4 ${entry.isIdle ? "opacity-70" : ""}`}
              >
                <div
                  class="w-20 shrink-0 text-right pr-2 pt-3 font-mono text-xs text-muted-foreground group-hover:text-foreground"
                >
                  {entry.time}
                </div>
                <div class="relative flex w-4 justify-center">
                  <div
                    class={`absolute top-4 size-3 rounded-full ${entry.dotClass} ring-4 ring-background`}
                  ></div>
                </div>
                <div class="flex-1 pb-4">
                  <div
                    class={`flex items-center justify-between gap-3 rounded-lg border p-3 transition ${
                      entry.isIdle ? "border-dashed bg-muted/40" : "bg-card"
                    } ${entry.borderClass}`}
                  >
                    <div class="flex items-center gap-3">
                      <div
                        class={`flex h-10 w-10 items-center justify-center rounded-md border ${entry.iconBgClass}`}
                      >
                        <svelte:component
                          this={entry.icon}
                          class={`size-5 ${entry.iconClass}`}
                        />
                      </div>
                      <div>
                        <p
                          class={`text-sm font-semibold ${
                            entry.isIdle ? "italic text-muted-foreground" : ""
                          }`}
                        >
                          {entry.app}
                        </p>
                        <p class="text-xs text-muted-foreground">
                          {entry.detail}
                        </p>
                      </div>
                    </div>
                    <Badge
                      variant={entry.isIdle ? "outline" : "secondary"}
                      class={`min-w-[72px] justify-center font-mono text-xs ${
                        entry.durationClass
                      }`}
                    >
                      {entry.duration}
                    </Badge>
                  </div>
                </div>
              </div>
            {/each}
          </div>

          <div
            class="pointer-events-none absolute left-[88px] flex w-full items-center"
            style={`top: ${currentIndicatorTop};`}
          >
            <div
              class="size-3 -ml-[5px] rounded-full bg-destructive ring-2 ring-background shadow-[0_0_10px_rgba(220,38,38,0.4)]"
            ></div>
            <div
              class="h-px w-full bg-destructive/80 shadow-[0_0_8px_rgba(220,38,38,0.3)]"
            ></div>
          </div>
        </div>
      </Card.Content>
    </Card.Root>
  </div>
</main>
