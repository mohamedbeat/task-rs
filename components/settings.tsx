
import { ThemeToggle } from "./theme-toggler"

export function Settings() {
    return (
        <div>
            <div className="flex items-center justify-between my-4">
                <h1 className="font-semibold text-2xl flex-1">Settings </h1>
            </div>
            <div className="flex gap-2 items-center">
                <p>

                    Theme selector:
                </p>
                <ThemeToggle />

            </div>
        </div>

    )

}