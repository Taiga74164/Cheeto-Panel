import { useEffect, useState } from "react";
import { useProcMon } from "../hooks/useProcMon.ts";
import { useInjector } from "../hooks/useInjector.ts";
import { X, XCircle } from "lucide-react";

interface FeaturesProps {
    onBackToHome: () => void;
    loadedModuleName?: string;
    onUnloadSuccess?: () => void;
}

export default function Features({ onBackToHome, loadedModuleName, onUnloadSuccess }: FeaturesProps) {
    const { processStatus, checked } = useProcMon("BlueArchive.exe");
    const { unloadDLL, isUnloading } = useInjector();
    const [error, setError] = useState("");

    useEffect(() => {
        if (!processStatus.isFound && checked) {
            onBackToHome();
        }
    }, [processStatus.isFound, checked, onBackToHome]);


    async function handleUnload() {
        if (!processStatus.isFound || !loadedModuleName) return;

        setError("");
        try {
            await unloadDLL(processStatus.name, loadedModuleName);
            onUnloadSuccess?.();
        } catch (err) {
            setError(`Failed to unload DLL: ${err}`);
            console.error("Failed to unload DLL:", err);
        }
    }

    // TODO: Get all features from feature base
    
    return (
        <div className="w-full max-w-6xl mx-auto">
            <div className="mb-6">
                <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                        <h1 className="text-2xl font-bold text-mirage-400">Cheat Features</h1>
                        {loadedModuleName && (
                            <div className="flex items-center space-x-2">
                                <span className="text-sm text-mirage-200">Module:</span>
                                <span className="text-sm font-medium text-mirage-400">{loadedModuleName}</span>
                                <button
                                    onClick={handleUnload}
                                    disabled={isUnloading}
                                    className="inline-flex items-center px-2 py-1 text-xs bg-red-600 text-white rounded hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    <X className="w-3 h-3 mr-1" />
                                    {isUnloading ? "Unloading..." : "Unload"}
                                </button>
                            </div>
                        )}
                    </div>
                </div>
            </div>

            {error && (
                <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
                    <div className="flex">
                        <div className="flex-shrink-0">
                            <XCircle className="w-5 h-5 text-red-500" />
                        </div>
                        <div className="ml-3">
                            <p className="text-sm text-red-800">{error}</p>
                        </div>
                        <div className="ml-auto pl-3">
                            <button
                                onClick={() => setError("")}
                                className="text-red-400 hover:text-red-600"
                            >
                                <X className="w-5 h-5" />
                            </button>
                        </div>
                    </div>
                </div>
            )}

        </div>
    );
}