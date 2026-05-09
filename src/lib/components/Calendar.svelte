<script lang="ts">
    import { onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import * as Card from "$lib/components/ui/card";
    // import type { ActivityEntry } from '$lib/shared/structs';
    import RangeCalendar from "$lib/components/ui/range-calendar/range-calendar.svelte";
    import { CalendarDate, today, getLocalTimeZone } from "@internationalized/date";
    import type { DateRange } from 'bits-ui';
    
    let currentDate = today(getLocalTimeZone());
    let value = $state<DateRange | undefined>({
        start: currentDate,
        end: currentDate
    });
    onMount(() => {
        

        return () => {
        };
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
            class="bg-transparent p-0 [--cell-size:--spacing(5)] sm:[--cell-size:--spacing(7)] md:[--cell-size:--spacing(9)]"
        />
    </Card.Content>
  </div>
</Card.Root>
