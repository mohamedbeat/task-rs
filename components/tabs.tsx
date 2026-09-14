import { Cpu, DatabaseCheck, Settings as SettingsIcon } from "lucide-react"

import { Tabs, TabsList, TabsTrigger } from "./ui/tabs"
import React from "react"
import { Processes } from "./processes"
import { Usage } from "./usage"
import { Settings } from "./settings"

type Tbs = "processes" | "usage" | "settings"

export function AppTabs() {
    const [activeTab, setActiveTab] = React.useState<Tbs>("processes")
    return (
        <div className="h-full">
            <Tabs defaultValue="processes">
                <TabsList className={"w-full"}>

                    <TabsTrigger value="processes" onClick={() => setActiveTab("processes")}>
                        <Cpu />
                        Processes
                    </TabsTrigger>
                    <TabsTrigger value="usage" onClick={() => setActiveTab("usage")}>
                        <DatabaseCheck />
                        Usage
                    </TabsTrigger>

                    <TabsTrigger value="settings" onClick={() => setActiveTab("settings")}>
                        <SettingsIcon />
                        Settings
                    </TabsTrigger>


                </TabsList>
            </Tabs>

            <div className={` m-2 h-[90%] `}>
                {activeTab === "processes" && <Processes />}
                {activeTab === "usage" && <Usage />}
                {activeTab === "settings" && <Settings />}
            </div>

        </div>
    )
}
