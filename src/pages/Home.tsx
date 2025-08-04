import React, { useState } from "react";
import { Check, Syringe, X } from "lucide-react";
import * as Checkbox from "@radix-ui/react-checkbox";
import { InjectionConfig } from "../types";
import { useProcMon } from "../hooks/useProcMon.ts";
import { useInjector } from "../hooks/useInjector.ts";
import { StatusIndicator } from "../components/ui/StatusIndicator.tsx";

interface HomeProps {
    onInjectionSuccess: (moduleName: string) => void;
    loadedModuleName?: string;
}

export default function Home({ onInjectionSuccess, loadedModuleName }: HomeProps) {
    const [injectionConfig, setInjectionConfig] = useState<InjectionConfig>({
        dllPath: "",
        usePipeMode: false,
    });
    const [status, setStatus] = useState("");

    const { processStatus } = useProcMon("BlueArchive.exe");
    const { injectDLL, unloadDLL, isLoading, isUnloading } = useInjector();

    const getStatusIndicatorProps = () => {
        if (isLoading || isUnloading) {
            return { status: "loading" as const, message: "Processing..." };
        }
        if (processStatus.isFound) {
            return {
                status: "ready" as const,
                message: `Process found (PID: ${processStatus.pid})`,
            };
        }
        return {
            status: "not-found" as const,
            message: "Process not found - Please start Blue Archive",
        };
    };

    const handleInjection = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!processStatus.isFound || !injectionConfig.dllPath.trim()) return;

        setStatus("");
        try {
            const rsp = await injectDLL(processStatus.name, injectionConfig);
            setStatus(`Success: ${rsp}`);

            const moduleName = injectionConfig.dllPath.split("\\").pop() || injectionConfig.dllPath;
            onInjectionSuccess(moduleName);
        } catch (error) {
            setStatus(`Error: ${error}`);
            console.error("DLL Injection failed:", error);
        }
    };

    const handleUnload = async () => {
        if (!processStatus.isFound || !loadedModuleName) return;

        setStatus("");
        try {
            await unloadDLL(processStatus.name, loadedModuleName);
            setStatus(`Success: DLL '${loadedModuleName}' unloaded successfully`);
        } catch (error) {
            setStatus(`Error: ${error}`);
            console.error("DLL Unload failed:", error);
        }
    };

    const canInject = processStatus.isFound && injectionConfig.dllPath.trim() && !isLoading && !isUnloading;
    const canUnload = processStatus.isFound && loadedModuleName && !isLoading && !isUnloading;

    return (
        <div className="w-full max-w-md mx-auto">
            <div className="text-center mb-8">
                <h1 className="text-3xl font-bold bg-gradient-to-r from-mirage-100 to-mirage-300 bg-clip-text text-transparent mb-2">
                    BA Cheeto
                </h1>
                <p className="text-mirage-300 text-sm">Blue Archive Enhancement Tool</p>
            </div>

            <div className="glass rounded-2xl p-6 mb-6">
                <div className="mb-6">
                    <h2 className="text-lg font-semibold text-mirage-100 mb-3">Process Status</h2>
                    <StatusIndicator {...getStatusIndicatorProps()} />
                </div>

                <div className="mb-6">
                    <h2 className="text-lg font-semibold text-mirage-100 mb-3 flex items-center">
                        <Syringe className="w-5 h-5 mr-2 text-mirage-400" />
                        DLL Injection
                    </h2>

                    <form onSubmit={handleInjection} className="space-y-4">
                        <input
                            type="text"
                            id="dll-path"
                            value={injectionConfig.dllPath}
                            onChange={(e) =>
                                setInjectionConfig((prev) => ({
                                    ...prev,
                                    dllPath: e.target.value,
                                }))
                            }
                            placeholder="Enter DLL path..."
                            className="input-modern w-full px-4 py-3 rounded-lg text-mirage-100 placeholder-mirage-400 focus:outline-none"
                            disabled={isLoading || isUnloading}
                        />

                        <label className="flex items-center space-x-2 text-mirage-300 text-sm">
                            <Checkbox.Root
                                className="w-5 h-5 rounded bg-mirage-800 border border-mirage-600 flex items-center justify-center data-[state=checked]:bg-mirage-600"
                                checked={injectionConfig.usePipeMode}
                                onCheckedChange={(checked) =>
                                    setInjectionConfig((prev) => ({
                                        ...prev,
                                        usePipeMode: !!checked,
                                    }))
                                }
                                disabled={isLoading || isUnloading}
                            >
                                <Checkbox.Indicator>
                                    <Check className="w-3 h-3 text-white" />
                                </Checkbox.Indicator>
                            </Checkbox.Root>
                            <span>Use Pipe Mode</span>
                        </label>

                        <button
                            type="submit"
                            disabled={!canInject}
                            className="btn-modern w-full px-4 py-3 rounded-lg text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {isLoading ? "Processing..." : "Inject DLL"}
                        </button>
                    </form>
                </div>

                {loadedModuleName && (
                    <div className="border-t border-mirage-600 pt-6">
                        <h2 className="text-lg font-semibold text-mirage-100 mb-3 flex items-center">
                            <X className="w-5 h-5 mr-2 text-mirage-400" />
                            Loaded Module
                        </h2>
                        
                        <div className="bg-mirage-800/50 rounded-lg p-4 mb-4">
                            <p className="text-mirage-300 text-sm mb-2">Currently loaded:</p>
                            <p className="text-mirage-100 font-medium">{loadedModuleName}</p>
                        </div>

                        <button
                            onClick={handleUnload}
                            disabled={!canUnload}
                            className="btn-modern w-full px-4 py-3 rounded-lg text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed bg-red-600 hover:bg-red-700"
                        >
                            {isUnloading ? "Unloading..." : "Unload DLL"}
                        </button>
                    </div>
                )}
            </div>

            {status && (
                <div
                    className={`glass rounded-lg p-4 ${
                        status.startsWith("Error")
                            ? "border-l-4 border-red-500 bg-red-500/10"
                            : "border-l-4 border-green-500 bg-green-500/10"
                    }`}
                >
                    <p
                        className={`text-sm font-medium ${
                            status.startsWith("Error") ? "text-red-200" : "text-green-200"
                        }`}
                    >
                        {status}
                    </p>
                </div>
            )}
        </div>
    );
}
