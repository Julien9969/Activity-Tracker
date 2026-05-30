<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import StatusDisplay from "$lib/components/ActivityEntry.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import DonutChart from "$lib/components/DonutChart.svelte";
  type FakeCalendarItem = {
    date: string;
    title: string;
    duration: string;
    location: string;
  };

  const fakeCalendarItems: FakeCalendarItem[] = [
    {
      date: "May 29",
      title: "TODO snooze",
      duration: "10:00-11:00",
      location: "Studio A"
    },
    {
      date: "May 30",
      title: "Activity sync",
      duration: "13:30-14:00",
      location: "Room 3"
    },
    {
      date: "Jun 01",
      title: "User interviews",
      duration: "09:00-12:00",
      location: "Remote"
    },
    {
      date: "Jun 02",
      title: "Sprint planning",
      duration: "15:00-16:30",
      location: "Lab 2"
    }
  ];
  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="min-h-screen bg-background">
  <div class="mx-auto space-y-4 scale-98">
    <div class="flex items-stretch gap-4 [&>*>div]:h-full">
      <div class="flex flex-col *:first:h-full">
        <Card.Root class="w-80">
          <Card.Header>
            <Card.Title>Calendar</Card.Title>
            <Card.Description>Upcoming (fake) entries</Card.Description>
          </Card.Header>
          <Card.Content class="space-y-3">
            {#each fakeCalendarItems as item}
              <div class="flex items-start justify-between gap-3 rounded-md border px-3 py-2">
                <div>
                  <p class="text-sm font-medium">{item.title}</p>
                  <p class="text-xs text-muted-foreground">{item.duration} · {item.location}</p>
                </div>
                <span class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                  {item.date}
                </span>
              </div>
            {/each}
          </Card.Content>
        </Card.Root>
      </div>
      <div class="flex-1 flex flex-col *:first:h-full">
        <DonutChart />
      </div>
    </div>
    <StatusDisplay />
    <Card.Root class="max-w-md w-[40%]">
      <Card.Header>
        <Card.Title>Greeting</Card.Title>
        <Card.Description>Enter your name to receive a personalized greeting</Card.Description>
      </Card.Header>
      <Card.Content>
        <form onsubmit={greet} class="flex gap-2">
          <Input 
            placeholder="Enter a name..." 
            bind:value={name}
            class="flex-1"
          />
          <Button type="submit">Greet</Button>
        </form>
        {#if greetMsg}
          <p class="mt-4 text-sm text-muted-foreground">{greetMsg}</p>
        {/if}
      </Card.Content>
    </Card.Root>
  </div>
</main>
