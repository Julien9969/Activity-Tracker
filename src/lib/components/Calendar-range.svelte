<script lang="ts">
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

<div class="flex p-2">
  <RangeCalendar 
    bind:value
    class="bg-transparent p-0 [--cell-size:--spacing(8)]" 
    // sm:[--cell-size:--spacing(6)] md:[--cell-size:--spacing(7)]"
    isDateDisabled={(date) => date.compare(currentDate) > 0}
  />
</div>
