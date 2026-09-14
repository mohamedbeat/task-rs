import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useEffect, useState } from "react";
import { ThemeToggle } from "./theme-toggler"
import { Switch } from "./ui/switch"

export function Settings() {
    const [enabled, setEnabled] = useState(false);

    useEffect(() => {
        isEnabled().then(setEnabled);
    }, []);

    const handleToggle = async (checked: boolean) => {
        try {
            if (checked) {
                await enable();
            } else {
                await disable();
            }
            setEnabled(checked);
        } catch (error) {
            console.error("Failed to toggle autostart:", error);
        }
    };

    return (
        <div>
            <div className="flex items-center justify-between my-4">
                <h1 className="font-semibold text-2xl flex-1">Settings</h1>
            </div>
            <div className="flex gap-2 items-center">
                <p>Theme selector:</p>
                <ThemeToggle />
            </div>

            <div className="flex gap-2 items-center">
                <p>Auto start:</p>
                <Switch className={"cursor-pointer"}
                    checked={enabled}
                    onCheckedChange={handleToggle}
                />
            </div>
        </div>
    )
}