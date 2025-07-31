import { useCallback, useState } from "react";
import { InjectionConfig } from "../types";
import { invoke } from "@tauri-apps/api/core";

export const useInjector = () => {
    const [isLoading, setIsLoading] = useState(false);

    const injectDLL = useCallback(async (processName: string, config: InjectionConfig): Promise<string> => {
        setIsLoading(true);
        try {
            const rsp = await invoke<string>("inject_dll_by_name", {
                processName,
                dllPath: config.dllPath,
                usePipeMode: config.usePipeMode,
            });
            return rsp;
        } finally {
            setIsLoading(false);
        }
    }, []);

    return { injectDLL, isLoading };
};
