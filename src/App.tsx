import { useEffect } from "react";
import { AppTabs } from "../components/tabs";
import "./App.css";
import { register, unregister, isRegistered } from "@tauri-apps/plugin-global-shortcut";
import { getCurrentWindow } from "@tauri-apps/api/window";

const SHORTCUT = "Ctrl+Shift+K";

export function useGlobalFocusShortcut() {
  useEffect(() => {
    let registered = false;

    const setup = async () => {
      // avoid double-registering if this hook re-runs (e.g. React strict mode in dev)
      const alreadyRegistered = await isRegistered(SHORTCUT);
      if (alreadyRegistered) {
        await unregister(SHORTCUT);
      }

      await register(SHORTCUT, async (event) => {
        if (event.state === "Pressed") {
          const appWindow = getCurrentWindow();
          await appWindow.show();
          await appWindow.unminimize();
          await appWindow.setFocus();
        }
      });

      registered = true;
    };

    setup().catch(console.error);

    return () => {
      if (registered) {
        unregister(SHORTCUT).catch(console.error);
      }
    };
  }, []);
}

function App() {
  useGlobalFocusShortcut();

  return (
    <main className="h-full">
      <AppTabs />
    </main>
  );
}

export default App;
