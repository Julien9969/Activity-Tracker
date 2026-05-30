<script lang="ts">
  import { CalendarIcon, ChevronLeft, ChevronRight } from "@lucide/svelte";
  import { getLocalTimeZone, today, type DateValue } from "@internationalized/date";
  import { untrack } from "svelte";
  import Calendar from "$lib/components/ui/calendar/calendar.svelte";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  function formatDate(date: DateValue | undefined) {
    if (!date) return "";
    return date.toDate(getLocalTimeZone()).toLocaleDateString("en-US", {
      day: "2-digit",
      month: "long",
      year: "numeric",
    });
  }

  function dayStep(step: number) {
    if (!value) return;
    value = value.add({ days: step });
    inputValue = formatDate(value);
  }

  const id = $props.id();

  let value = $state<DateValue | undefined>(today(getLocalTimeZone()));
  let open = $state(false);
  let inputValue = $state(untrack(() => formatDate(value)));
</script>

<div
  class="inline-flex items-center gap-2 rounded-lg border bg-card p-1 shadow-sm"
>
  <Button
    variant="ghost"
    size="icon-sm"
    aria-label="Previous day"
    onclick={() => {
      dayStep(-1);
    }}
  >
    <ChevronLeft class="size-4" />
  </Button>

  <div class="flex flex-1 justify-center min-w-0">
    <Popover.Root bind:open>
      <Popover.Trigger id="{id}-date-picker">
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            class="inline-flex items-center gap-3 max-w-full min-w-0 px-2 text-sm font-normal bg-transparent hover:bg-transparent focus-visible:ring-0 shadow-none"
            onkeydown={(e) => {
              if (e.key === "ArrowDown") {
                e.preventDefault();
                open = true;
              }
            }}
          >
            <span class="truncate">{inputValue || "Select date"}</span>
            <CalendarIcon class="size-4 shrink-0" />
          </Button>
        {/snippet}
      </Popover.Trigger>
        <Popover.Content
          class="w-auto overflow-hidden p-0"
          align="end"
          alignOffset={-8}
          sideOffset={10}
        >
          <Calendar
            type="single"
            bind:value
            captionLayout="dropdown"
            onValueChange={(v) => {
              inputValue = formatDate(v);
              open = false;
            }}
          />
        </Popover.Content>
    </Popover.Root>
  </div>
  <Button
    variant="ghost"
    size="icon-sm"
    aria-label="Next day"
    onclick={() => {
      dayStep(1);
    }}
  >
    <ChevronRight class="size-4" />
  </Button>
</div>
