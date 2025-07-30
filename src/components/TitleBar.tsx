import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";

// https://v2.tauri.app/learn/window-customization/
export default function TitleBar() {
    const appWindow = getCurrentWindow();

    return (
        <div
            data-tauri-drag-region={true}
            className="h-10 bg-mirage-900/80 backdrop-blur-sm flex items-center justify-between px-4 select-none border-b border-mirage-700/50"
        >
            <div className="flex items-center space-x-3">
                <div className="text-mirage-100 font-medium text-sm">BA Cheeto</div>
            </div>

            <div className="flex items-center">
                <button
                    title="Minimize"
                    onClick={() => appWindow.minimize()}
                    className="w-8 h-8 flex items-center justify-center hover:bg-mirage-700/50 rounded"
                >
                    <Minus className="w-4 h-4 text-mirage-300" />
                </button>
                <button
                    title="Maximize"
                    onClick={() => appWindow.toggleMaximize()}
                    className="w-8 h-8 flex items-center justify-center hover:bg-mirage-700/50 rounded"
                >
                    <Square className="w-4 h-4 text-mirage-300" />
                </button>
                <button
                    title="Close"
                    onClick={() => appWindow.close()}
                    className="w-8 h-8 flex items-center justify-center hover:bg-red-500/50 rounded"
                >
                    <X className="w-4 h-4 text-mirage-300" />
                </button>
            </div>
        </div>
    );
}
