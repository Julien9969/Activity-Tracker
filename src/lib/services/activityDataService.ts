import { invoke } from "@tauri-apps/api/core";
import { get, writable } from "svelte/store";
import { getLocalTimeZone, today, type DateValue } from "@internationalized/date";
import type { DateRange } from "bits-ui";

export type GroupedEntry = { name: string; totalMs: number };
export type ChartItem = { app: string; duration: number; color: string };

// TODO should assign per apps (maybe generate using app name hash)
const CHART_COLORS = [
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
];

const localTimeZone = getLocalTimeZone();
const todayLocal = today(localTimeZone);

export const selectedRange = writable<DateRange>({
    start: todayLocal,
    end: todayLocal,
});

export const chartData = writable<ChartItem[]>([]);
export const totalMinutes = writable(0);
export const isLoading = writable(true);
export const error = writable("");
export const rangeLabel = writable("");

function toStartOfDay(date: Date): Date {
    return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function toEndOfDay(date: Date): Date {
    return new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 999);
}

function toLocalDate(value: DateValue): Date {
    return value.toDate(localTimeZone);
}

function formatRangeLabel(start: Date, end: Date): string {
    const labelStart = start.toLocaleDateString(undefined, {
        month: "long",
        day: "numeric",
        year: "numeric",
    });

    const labelEnd = end.toLocaleDateString(undefined, {
        month: "long",
        day: "numeric",
        year: "numeric",
    });

    return labelStart === labelEnd ? labelStart : `${labelStart} - ${labelEnd}`;
}

function rangesEqual(a: DateRange, b: DateRange): boolean {
    if (!a.start || !b.start) return false;

    const aStart = toStartOfDay(toLocalDate(a.start)).getTime();
    const aEnd = toEndOfDay(toLocalDate(a.end ?? a.start)).getTime();
    const bStart = toStartOfDay(toLocalDate(b.start)).getTime();
    const bEnd = toEndOfDay(toLocalDate(b.end ?? b.start)).getTime();

    return aStart === bStart && aEnd === bEnd;
}

export function setRange(range: DateRange | undefined): void {
    if (!range?.start) return;

    const current = get(selectedRange);
    if (rangesEqual(current, range)) return;

    selectedRange.set(range);
    void refreshData();
}

export async function refreshData(): Promise<void> {
    if (typeof window === "undefined") return;

    const range = get(selectedRange);
    if (!range?.start) return;

    const start = toStartOfDay(toLocalDate(range.start));
    const end = toEndOfDay(toLocalDate(range.end ?? range.start));

    rangeLabel.set(formatRangeLabel(start, end));
    isLoading.set(true);
    error.set("");

    try {
        const entries = await invoke<GroupedEntry[]>("get_grouped_data", {
            groupBy: "app_name",
            startTime: start.getTime(),
            endTime: end.getTime(),
        });

        const top = entries.slice(0, 7);
        const otherMs = entries.slice(7).reduce((sum, e) => sum + e.totalMs, 0);
        if (otherMs > 0) top.push({ name: "Other", totalMs: otherMs });

        const items = top.map((entry, index) => ({
            app: entry.name,
            duration: Math.round(entry.totalMs / 60000),
            color: CHART_COLORS[index % CHART_COLORS.length],
        }));

        chartData.set(items);
        totalMinutes.set(items.reduce((acc, item) => acc + item.duration, 0));
    } catch (err) {
        error.set(String(err));
    } finally {
        isLoading.set(false);
    }
}

let refreshTimer: number | null = null;

export function startAutoRefresh(intervalMs = 60 * 1000): () => void {
    if (typeof window === "undefined") return stopAutoRefresh;

    stopAutoRefresh();
    refreshTimer = window.setInterval(() => {
        void refreshData();
    }, intervalMs);

    return stopAutoRefresh;
}

export function stopAutoRefresh(): void {
    if (refreshTimer === null) return;

    clearInterval(refreshTimer);
    refreshTimer = null;
}
