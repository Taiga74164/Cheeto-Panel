import { useEffect } from "react";
import { useProcMon } from "../hooks/useProcMon.ts";

interface FeaturesProps {
    onBackToHome: () => void;
}

export default function Features({ onBackToHome }: FeaturesProps) {
    const { processStatus, checked } = useProcMon("BlueArchive.exe");

    useEffect(() => {
        if (!processStatus.isFound && checked) {
            onBackToHome();
        }
    }, [processStatus.isFound, checked, onBackToHome]);

    // TODO: use checkbox later
    // async function toggleCheat(enabled: boolean) {
    //     setLoading(true);
    //     setStatus("");
    //
    //     try {
    //         const response = await invoke<string>("send_feature_command", {
    //             feature: "InstantWin",
    //             enable: enabled,
    //         });
    //
    //         setStatus(`Success: ${response}`);
    //         console.log("Cheat response:", response);
    //     } catch (error) {
    //         setStatus(`Error: ${error}`);
    //         console.error("Failed to toggle cheat:", error);
    //     } finally {
    //         setLoading(false);
    //     }
    // }

    // TODO: setup core module for features
    return (
        <div className="w-full">
            <div className="flex justify-center">
                <h1>Test</h1>
            </div>
        </div>
    );
}
