import {
    Table,
    TableBody, TableCell, TableHead,
    TableHeader,
    TableRow
} from "./ui/table"
import { Input } from './ui/input'
import { Button } from "./ui/button"

import React from "react"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"

import { getCpuColor, getMemoryColor } from "../lib/utils"
import { X } from "lucide-react"
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip"

export type ProcessInfo = {
    pid: number,
    name: string,
    cpu_usage: number,
    cpu_usage_h: string,
    memory: number,
    memory_h: string,
    disk_usage: number
    disk_usage_h: string
    exe_path?: string
    icon?: string
}

export function Processes() {
    const [selectedP, setSelectedP] = React.useState<ProcessInfo | null>(null);
    const [processes, setProcesses] = React.useState<ProcessInfo[]>([])
    const [filter, setFilter] = React.useState("")

    React.useEffect(() => {
        const getProcesses = async () => {
            const res = await invoke<ProcessInfo[]>("get_processes", {})
            setProcesses(res.sort((a, b) => b.memory - a.memory))
        }
        const unlisten = listen<ProcessInfo[]>("processes-update", (e) => {

            setProcesses(e.payload.sort((a, b) => b.memory - a.memory))
        })


        getProcesses()

        return () => {
            unlisten.then((fn) => fn());
        }
    }, [])

    const filteredProcesses = React.useMemo(() => {
        if (!filter.trim()) return processes
        const lower = filter.toLowerCase()
        return processes.filter((p) => p.name.toLowerCase().includes(lower))
    }, [processes, filter])


    const killProcess = async () => {
        if (!selectedP) return
        await invoke<boolean>("kill_process", { pid: selectedP?.pid })
    }

    return (
        <div>
            <div className="flex items-center justify-between my-4">
                <h1 className="font-semibold text-2xl flex-1">Processes - {processes.length}</h1>
                <div className="flex-2">
                    <Input
                        type="text"
                        value={filter}
                        onChange={(e) => setFilter(e.target.value)}
                        placeholder="Filter by name..."
                    />
                </div>
                <div className="flex-1 flex justify-end">
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={"secondary"} onClick={killProcess} disabled={selectedP === null}>
                                <X />
                            </Button>

                        </TooltipTrigger>
                        <TooltipContent>
                            <p>Kill Process</p>
                        </TooltipContent>
                    </Tooltip>
                </div>

            </div>
            <div className="max-h-[87vh] overflow-y-auto rounded-md border">
                <Table >
                    <TableHeader className="sticky top-0 bg-background z-10" >
                        <TableRow>
                            <TableHead className="w-[100px]">Name</TableHead>
                            <TableHead className="w-[100px]">PID</TableHead>
                            <TableHead>CPU</TableHead>
                            <TableHead>Memory</TableHead>
                            <TableHead>Disk Read</TableHead>
                            <TableHead>Path</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody className="">
                        {filteredProcesses.map((p) => (
                            <TableRow key={p.pid} onClick={() => {
                                if (!!selectedP && selectedP?.pid === p.pid) {
                                    setSelectedP(null)
                                    return
                                }
                                setSelectedP(p)
                            }} className={`box-border cursor-pointer ${p.pid === selectedP?.pid && "border-2 border-blue-300"} `}>
                                <TableCell>
                                    <div className="flex gap-2 items-center">
                                        {p.icon ? <img src={p.icon} className="h-4 w-4" alt="" /> : "? "}
                                        <p>{p.name}</p>

                                    </div>
                                </TableCell>
                                <TableCell>{p.pid}</TableCell>
                                <TableCell className={`${getCpuColor(p.cpu_usage)}`}>{p.cpu_usage_h}</TableCell>
                                <TableCell className={`${getMemoryColor(p.memory)}`}>{p.memory_h}</TableCell>
                                <TableCell>{p.disk_usage_h}</TableCell>
                                <TableCell>{p.exe_path}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>

                </Table>
            </div>
        </div>
    )
}