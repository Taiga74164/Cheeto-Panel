import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ProcessStatus } from "../types";

export const useProcMon = (processName: string, interval: number = 1000) => {
    const [checked, setChecked] = useState(false);
    const [processStatus, setProcessStatus] = useState<ProcessStatus>({
        isFound: false,
        name: processName,
    });

    const checkProcess = useCallback(async () => {
        try {
            const pid = await invoke<number>("find_process_by_name", { processName });
            setProcessStatus({
                isFound: true,
                pid,
                name: processName,
            });
        } catch {
            setProcessStatus({
                isFound: false,
                name: processName,
            });
        } finally {
            setChecked(true);
        }
    }, [processName]);

    useEffect(() => {
        checkProcess();
        const intervalId = setInterval(checkProcess, interval);
        return () => clearInterval(intervalId);
    }, [checkProcess, interval]);

    return { processStatus, checkProcess, checked };
};
