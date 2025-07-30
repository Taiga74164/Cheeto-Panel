import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Check, Loader, Sparkles, Syringe, X } from "lucide-react";
import * as Checkbox from "@radix-ui/react-checkbox";

export default function Home() {
    const [dllPath, setDllPath] = useState("");
    const [loading, setLoading] = useState(false);
    const [status, setStatus] = useState("");
    const [usePipeMode, setUsePipeMode] = useState(true);

    // TODO: use checkbox later
    async function toggleCheat(enabled: boolean) {
        setLoading(true);
        setStatus("");

        try {
            const response = await invoke<string>("send_feature_command", {
                feature: "InstantWin",
                enable: enabled,
            });

            setStatus(`Success: ${response}`);
            console.log("Cheat response:", response);
        } catch (error) {
            setStatus(`Error: ${error}`);
            console.error("Failed to toggle cheat:", error);
        } finally {
            setLoading(false);
        }
    }

    async function injectDLL(path: string) {
        setLoading(true);
        setStatus("");

        try {
            const response = await invoke<string>("inject_dll_by_name", {
                processName: "BlueArchive.exe",
                dllPath: path,
                usePipeMode: usePipeMode,
            });
            setStatus(`DLL injected successfully: ${response}`);
            console.log("DLL injection response:", response);
        } catch (error) {
            setStatus(`Error: ${error}`);
            console.error("Failed to inject DLL:", error);
        } finally {
            setLoading(false);
        }
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-mirage-950 via-mirage-900 to-mirage-800 flex items-center justify-center p-4">
            <main className="w-full max-w-md mx-auto">
                <div className="text-center mb-8">
                    <h1 className="text-3xl font-bold bg-gradient-to-r from-mirage-100 to-mirage-300 bg-clip-text text-transparent mb-2">
                        BA Cheeto
                    </h1>
                    <p className="text-mirage-300 text-sm">Blue Archive Enhancement Tool</p>
                </div>

                <div className="glass rounded-2xl p-6 mb-6">
                    <div className="mb-6">
                        <h2 className="text-lg font-semibold text-mirage-100 mb-3 flex items-center">
                            <Syringe className="w-5 h-5 mr-2 text-mirage-400" />
                            DLL Injection
                        </h2>

                        <form
                            onSubmit={(e) => {
                                e.preventDefault();
                                injectDLL(dllPath);
                            }}
                            className="space-y-3"
                        >
                            <input
                                id="dll-path"
                                onChange={(e) => setDllPath(e.currentTarget.value)}
                                placeholder="Enter DLL path..."
                                className="input-modern w-full px-4 py-3 rounded-lg text-mirage-100 placeholder-mirage-400 focus:outline-none"
                                disabled={loading}
                            />

                            <label className="flex items-center space-x-2 text-mirage-300 text-sm">
                                <Checkbox.Root
                                    className="w-5 h-5 rounded bg-mirage-800 border border-mirage-600 flex items-center justify-center data-[state=checked]:bg-mirage-600"
                                    checked={usePipeMode}
                                    onCheckedChange={(checked) => setUsePipeMode(!!checked)}
                                    disabled={loading}
                                >
                                    <Checkbox.Indicator>
                                        <Check className="w-3 h-3 text-white" />
                                    </Checkbox.Indicator>
                                </Checkbox.Root>
                                <span>Use Pipe Mode</span>
                            </label>

                            <button
                                type="submit"
                                disabled={loading}
                                className="btn-modern w-full px-4 py-3 rounded-lg text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                {loading ? (
                                    <span className="flex items-center justify-center">
                                        <Loader className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" />
                                        Processing...
                                    </span>
                                ) : (
                                    "Inject DLL"
                                )}
                            </button>
                        </form>
                    </div>

                    <div>
                        <h2 className="text-lg font-semibold text-mirage-100 mb-3 flex items-center">
                            <Sparkles className="w-5 h-5 mr-2 text-mirage-400" />
                            Feature
                        </h2>

                        <div className="space-y-3">
                            <button
                                onClick={() => toggleCheat(true)}
                                disabled={loading}
                                className="btn-modern w-full px-4 py-3 rounded-lg text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                            >
                                {loading ? (
                                    <span className="flex items-center justify-center">
                                        <Loader className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" />
                                        Processing...
                                    </span>
                                ) : (
                                    <>
                                        <Check className="w-5 h-5 mr-2" />
                                        Enable Instant Win
                                    </>
                                )}
                            </button>
                        </div>
                    </div>
                </div>

                {status && (
                    <div
                        className={`glass rounded-lg p-4 status-enter ${
                            status.startsWith("Error")
                                ? "border-l-4 border-red-500 bg-red-500/10"
                                : "border-l-4 border-green-500 bg-green-500/10"
                        }`}
                    >
                        <div className="flex items-start">
                            <div
                                className={`flex-shrink-0 w-5 h-5 rounded-full flex items-center justify-center ${
                                    status.startsWith("Error") ? "bg-red-500" : "bg-green-500"
                                }`}
                            >
                                {status.startsWith("Error") ? (
                                    <X className="w-4 h-4 text-white" />
                                ) : (
                                    <Check className="w-4 h-4 text-white" />
                                )}
                            </div>
                            <div className="ml-3">
                                <p
                                    className={`text-sm font-medium ${
                                        status.startsWith("Error") ? "text-red-200" : "text-green-200"
                                    }`}
                                >
                                    {status}
                                </p>
                            </div>
                        </div>
                    </div>
                )}
            </main>
        </div>
    );
}
