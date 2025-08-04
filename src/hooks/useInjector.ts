import { useCallback, useState } from "react";
import { InjectionConfig } from "../types";
import { invoke } from "@tauri-apps/api/core";

export const useInjector = () => {
    const [isLoading, setIsLoading] = useState(false);
    const [isUnloading, setIsUnloading] = useState(false);

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

    const unloadDLL = useCallback(async (processName: string, moduleName: string): Promise<void> => {
        setIsUnloading(true);
        try {
            await invoke("unload_remote_module", {
                processName,
                moduleName,
            });
        } finally {
            setIsUnloading(false);
        }
    }, []);

    const isModuleLoaded = useCallback(async (processName: string, moduleName: string): Promise<boolean> => {
        try {
            return await invoke<boolean>("is_module_loaded", {
                processName,
                moduleName,
            });
        } catch {
            return false;
        }
    }, []);

    return { injectDLL, unloadDLL, isModuleLoaded, isLoading, isUnloading };
};
