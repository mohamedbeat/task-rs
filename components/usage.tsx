import { Cpu, Gpu, HardDrive, MemoryStick } from "lucide-react"
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip"
import { useStatsStore } from "../context/usage-context"



export function Usage() {
    const stats = useStatsStore((s) => s.stats)

    return (


        <div className="h-[87%]" >
            <h1 className="font-semibold text-2xl flex-1">Usage</h1>

            <div className=" grid grid-cols-1 sm:grid-cols-2 gap-9 w-full h-full">
                <div className="flex flex-col items-center justify-center">
                    <h1 className="font-semibold text-xl">Memory</h1>
                    <MemoryStick size={40} />
                    <h1>{stats.memory_percent.toFixed(1)} %</h1>
                    <h1>{stats.memory_used_gb.toFixed(1)}GB / {stats.memory_total_gb.toFixed(1)}GB</h1>
                </div>

                <div className="flex flex-col items-center justify-center">
                    <Tooltip >
                        <TooltipTrigger className="flex flex-col items-center justify-center">

                            <h1 className="font-semibold text-xl">CPU</h1>
                            <Cpu size={40} />
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

                    <h1 className="font-semibold text-xl">Storage</h1>
                    <HardDrive size={40} />
                    <h1>{stats.disks[0].percent.toFixed(1)} %</h1>
                    <h1>{stats.disks[0].used_gb.toFixed(1)}GB / {stats.disks[0].total_gb.toFixed(1)} GB</h1>
                </div>

                <div className="flex flex-col items-center justify-center">

                    <h1 className="font-semibold text-xl">GPU</h1>
                    <Gpu size={40} />
                    <h1>{stats.gpu[0].name}: {stats.gpu[0].usage_percent.toFixed(1)} %</h1>
                    <h1>{stats.gpu[0].memory_used_gb.toFixed(1)}GB / {stats.gpu[0].memory_total_gb.toFixed(1)} GB</h1>
                </div>

            </div>


        </div >
    )
}