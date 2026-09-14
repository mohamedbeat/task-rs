import { ThemeProvider } from "next-themes"
import { TooltipProvider } from "../components/ui/tooltip"
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { startStatsListener } from "../context/usage-context"

startStatsListener()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <TooltipProvider>
      <ThemeProvider defaultTheme="dark" enableSystem={true} attribute={"class"} >
        <App />
      </ThemeProvider>
    </TooltipProvider>
  </React.StrictMode>,
);
