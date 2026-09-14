import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { revealItemInDir } from '@tauri-apps/plugin-opener'

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
import { ArrowDown, ArrowUp, Clipboard, FolderOpen, X } from "lucide-react"
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip"
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue, } from "./ui/select"
import { ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "./ui/context-menu"

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

type SortField = "name" | "cpu_usage" | "memory" | "pid"
type SortDirection = "asc" | "desc"
export function Processes() {
    const [selectedP, setSelectedP] = React.useState<ProcessInfo | null>(null);
    const [processes, setProcesses] = React.useState<ProcessInfo[]>([])
    const [filter, setFilter] = React.useState("")

    const [sortField, setSortField] = React.useState<SortField>("memory")
    const [sortDirection, setSortDirection] = React.useState<SortDirection>("desc")

    const sortProcesses = React.useCallback(
        (list: ProcessInfo[]) => {
            const sorted = [...list].sort((a, b) => {
                let cmp = 0
                switch (sortField) {
                    case "name":
                        cmp = a.name.localeCompare(b.name)
                        break
                    case "pid":
                        cmp = a.pid - b.pid
                        break
                    case "cpu_usage":
                        cmp = a.cpu_usage - b.cpu_usage
                        break
                    case "memory":
                        cmp = a.memory - b.memory
                        break
                }
                return sortDirection === "asc" ? cmp : -cmp
            })
            return sorted
        },
        [sortField, sortDirection]
    )

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
        let list = processes
        if (filter.trim()) {
            const lower = filter.toLowerCase()
            list = list.filter((p) => p.name.toLowerCase().includes(lower))
        }
        return sortProcesses(list)
    }, [processes, filter, sortProcesses])


    const killProcess = async () => {
        if (!selectedP) return
        await invoke<boolean>("kill_process", { pid: selectedP?.pid })
    }
    const copy_path = async (p: ProcessInfo) => {
        if (!p.exe_path) return
        await writeText(p.exe_path)
    }
    async function openInExplorer(path: string) {
        await revealItemInDir(path)
    }

    const getTabSortIcon = (field: SortField) => {
        if (field !== sortField) return null
        return sortDirection === "asc" ? <ArrowUp className="h-3 w-3" /> : <ArrowDown className="h-3 w-3" />
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
                    <div className="flex items-center gap-1">
                        <Select
                            value={sortField}
                            onValueChange={(e) => setSortField(e as SortField)}
                        >
                            <SelectTrigger>
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectItem value="name">Name</SelectItem>
                                    <SelectItem value="cpu_usage">CPU Usage</SelectItem>
                                    <SelectItem value="memory">Memory</SelectItem>
                                    <SelectItem value="pid">PID</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                        <Button
                            variant="secondary"
                            size="icon"
                            onClick={() =>
                                setSortDirection((d) => (d === "asc" ? "desc" : "asc"))
                            }
                            title={sortDirection === "asc" ? "Ascending" : "Descending"}
                        >
                            {sortDirection === "asc" ? <ArrowUp /> : <ArrowDown />}
                        </Button>
                    </div>
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={"destructive"} onClick={killProcess} disabled={selectedP === null}>
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
                            <TableHead className="w-[100px]">  <div className='flex justify-start items-center gap-2'>{getTabSortIcon('name')}<p>Name</p></div> </TableHead>
                            <TableHead className="w-[100px]"> <div className='flex justify-start items-center gap-2'>{getTabSortIcon('pid')}<p>PID</p></div> </TableHead>
                            <TableHead ><div className='flex justify-start items-center gap-2'>{getTabSortIcon('cpu_usage')} <p>CPU</p></div> </TableHead>
                            <TableHead > <div className='flex justify-start items-center gap-2'>{getTabSortIcon('memory')}<p>Memory</p></div></TableHead>
                            <TableHead>Disk Read</TableHead>
                            <TableHead>Path</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody className="">

                        {filteredProcesses.map((p) => (
                            <ContextMenu>
                                <ContextMenuTrigger render={
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
                                }>
                                </ContextMenuTrigger>
                                <ContextMenuContent>
                                    <ContextMenuItem className={"justify-between"} variant="destructive">
                                        <p>Kill Process</p>
                                        <X />
                                    </ContextMenuItem>
                                    <ContextMenuItem disabled={!p.exe_path} className={"justify-between"} onClick={() => {
                                        if (!p.exe_path) return
                                        openInExplorer(p.exe_path)
                                    }} >
                                        <p>Open Path</p>
                                        <FolderOpen />
                                    </ContextMenuItem>
                                    <ContextMenuItem className={"justify-between"} onClick={() => copy_path(p)} disabled={!p.exe_path} >
                                        <p>Copy Path</p>
                                        <Clipboard />
                                    </ContextMenuItem>

                                </ContextMenuContent>

                            </ContextMenu>
                        ))}
                    </TableBody>

                </Table>
            </div>
        </div>
    )
}