<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import RangeCalendar from "$lib/components/ui/range-calendar/range-calendar.svelte";
  import { today, getLocalTimeZone } from "@internationalized/date";
  import type { DateRange } from "bits-ui";
  import { setRange } from "$lib/services/activityDataService";

  let currentDate = today(getLocalTimeZone());
  let value = $state<DateRange | undefined>({
    start: currentDate,
    end: currentDate
  });

  $effect(() => {
    if (value?.start) {
      console.log("Selected range:", value.start.toString(), value.end?.toString());
      setRange(value);
    }
  });
</script>

<Card.Root class="h-full p-2">
  <!-- <Card.Header>
    <Card.Title>Calendar</Card.Title>
    <Card.Description>Select day or period</Card.Description>
  </Card.Header> -->
  <div class="self-center">
    <Card.Content>
        <RangeCalendar 
            bind:value
            class="bg-transparent p-0 [--cell-size:--spacing(5)] sm:[--cell-size:--spacing(7)] md:[--cell-size:--spacing(8)]"
            isDateDisabled={(date) => date.compare(currentDate) > 0}
        />
    </Card.Content>
  </div>
</Card.Root>
