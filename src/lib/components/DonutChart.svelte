<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as Chart from "$lib/components/ui/chart/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { PieChart, Text, Tooltip as TooltipPrimitive } from "layerchart";
  import {
    chartData,
    totalMinutes,
    isLoading,
    error,
    rangeLabel,
    refreshData,
    startAutoRefresh,
  } from "$lib/services/activityDataService";

  const chartConfig = $derived({
    duration: { label: "Minutes" },
    ...Object.fromEntries($chartData.map(({ app, color }) => [app, { label: app, color }])),
  });

  const totalDisplay = $derived(
    $totalMinutes >= 60
      ? `${Math.floor($totalMinutes / 60)}h ${$totalMinutes % 60}m`
      : `${$totalMinutes}m`
  );

  let stopRefresh: (() => void) | null = null;

  onMount(async () => {
    await refreshData();
    stopRefresh = startAutoRefresh(60 * 1000);
  });

  onDestroy(() => {
    stopRefresh?.();
  });
</script>

<Card.Root class="flex flex-col h-full" style="padding: 0.8rem 0rem;">
  <Card.Header class="items-center">
    <Card.Title>App Usage</Card.Title>
    <Card.Description>{$rangeLabel}</Card.Description>
  </Card.Header>
  <Card.Content class="flex-1">
    {#if $isLoading}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-muted-foreground text-sm">Loading…</span>
      </div>
    {:else if $error}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-destructive text-sm">{$error}</span>
      </div>
    {:else if $chartData.length === 0}
      <div class="flex items-center justify-center" style="height: 200px;">
        <span class="text-muted-foreground text-sm">No activity recorded for selected range.</span>
      </div>
    {:else}
      <Chart.Container config={chartConfig} class="mx-auto aspect-square" style="max-height: 200px;">
        <PieChart
          data={$chartData}
          key="app"
          value="duration"
          c="color"
          innerRadius={70}
          padding={6}
          props={{ pie: { motion: "tween" } }}
        >
          {#snippet aboveMarks()}
            <Text
              value={totalDisplay}
              textAnchor="middle"
              verticalAnchor="middle"
              class="fill-foreground text-3xl! font-bold"
              dy={3}
            />
            <Text
              value="TODO ?"
              textAnchor="middle"
              verticalAnchor="middle"
              class="fill-muted-foreground! text-muted-foreground"
              dy={22}
            />
          {/snippet}
          {#snippet tooltip()}
            <TooltipPrimitive.Root variant="none">
              <div class="border-border/50 bg-background grid min-w-48 items-start gap-1.5 rounded-lg border px-2.5 py-2 text-sm shadow-xl">
                <div class="mb-1 font-medium text-foreground">App Usage List</div>
                <div class="grid gap-1.5">
                  {#each $chartData as item}
                    <div class="flex w-full items-center gap-2">
                      <div class="h-2 w-2 shrink-0 rounded-full" style="background-color: {item.color};"></div>
                      <div class="flex flex-1 items-center justify-between leading-none">
                        <span class="text-muted-foreground">{item.app}</span>
                        <span class="text-foreground font-mono font-medium tabular-nums ml-4">{item.duration} min</span>
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
            </TooltipPrimitive.Root>
          {/snippet}
        </PieChart>
      </Chart.Container>
    {/if}
  </Card.Content>
  <!-- <Card.Footer class="flex-col gap-1 text-sm">
    <div class="text-muted-foreground leading-none">
      Showing active app time for selected range
    </div>
  </Card.Footer> -->
</Card.Root>
