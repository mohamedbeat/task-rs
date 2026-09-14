import { listen } from "@tauri-apps/api/event"
import { create } from "zustand"

export type GpuInfo = {
    name: string
    usage_percent: number
    memory_used_gb: number
    memory_total_gb: number
}

export type DiskInfo = {
    name: string
    used_gb: number
    total_gb: number
    percent: number
}

export type SystemStats = {
    memory_used_gb: number
    memory_total_gb: number
    memory_percent: number

    cpu_name: string
    cpu_percent: number
    cpu_per_core: number[]

    disks: DiskInfo[]
    gpu: GpuInfo[]
}

const defaultStats: SystemStats = {
    cpu_name: "",
    cpu_per_core: [0],
    cpu_percent: 0,
    memory_percent: 0,
    memory_total_gb: 0,
    memory_used_gb: 0,
    disks: [{ name: "", percent: 0, total_gb: 0, used_gb: 0 }],
    gpu: [{ memory_total_gb: 0, memory_used_gb: 0, name: "", usage_percent: 0 }],
}

type StatsState = {
    stats: SystemStats
    setStats: (s: SystemStats) => void
}

export const useStatsStore = create<StatsState>((set) => ({
    stats: defaultStats,
    setStats: (stats) => set({ stats }),
}))

// Start the Tauri listener once for the whole app lifetime.
// Call this from main.tsx / App.tsx.
let started = false
export function startStatsListener() {
    if (started) return
    started = true

    listen<SystemStats>("stats", (e) => {
        useStatsStore.getState().setStats(e.payload)
    })
}