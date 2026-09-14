import { listen } from "@tauri-apps/api/event"
import { Cpu, Gpu, HardDrive, MemoryStick } from "lucide-react"
import React, { useState } from "react"
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip"
import { useStatsStore } from "../context/usage-context"



export function Usage() {
    const stats = useStatsStore((s) => s.stats)

    return (


        <div className="h-full" >
            <h1 className="font-semibold text-2xl flex-1">Usage</h1>

            <div className="grid grid-cols-2 gap-9 w-full h-full">
                <div className="flex flex-col items-center justify-center">
                    <MemoryStick />
                    <h1>{stats.memory_percent.toFixed(1)} %</h1>
                    <h1>{stats.memory_used_gb.toFixed(1)}GB / {stats.memory_total_gb.toFixed(1)}GB</h1>
                </div>

                <div className="flex flex-col items-center justify-center">
                    <Tooltip >
                        <TooltipTrigger className="flex flex-col items-center justify-center">
                            <Cpu />
                            <h1>{stats.cpu_name}</h1>
                            <h1>{stats.cpu_percent.toFixed(1)} %</h1>


                        </TooltipTrigger>
                        <TooltipContent>
                            <div>
                                {stats.cpu_per_core.map((c, i) => (<div key={i}>{i}: {c}</div>))}
                            </div>
                        </TooltipContent>
                    </Tooltip>
                </div>
                <div className="flex flex-col items-center justify-center">
                    <HardDrive />
                    <h1>{stats.disks[0].percent.toFixed(1)} %</h1>
                    <h1>{stats.disks[0].used_gb.toFixed(1)}GB / {stats.disks[0].total_gb.toFixed(1)} GB</h1>
                </div>

                <div className="flex flex-col items-center justify-center">
                    <Gpu />
                    <h1>{stats.gpu[0].name}: {stats.gpu[0].usage_percent.toFixed(1)} %</h1>
                    <h1>{stats.gpu[0].memory_used_gb.toFixed(1)}GB / {stats.gpu[0].memory_total_gb.toFixed(1)} GB</h1>
                </div>

            </div>


        </div >
    )
}