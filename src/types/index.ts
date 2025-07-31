export interface ProcessStatus {
    isFound: boolean;
    pid?: number;
    name: string;
}

export interface InjectionConfig {
    dllPath: string;
    usePipeMode: boolean;
}

// export interface FeatureBase {
//     id: string;
//     name: string;
//     description: string;
//     enabled: boolean;
// }
//
// export interface Feature<T = unknown> extends FeatureBase {
//     data?: T;
// }

export interface AppState {
    processStatus: ProcessStatus;
    injectionConfig: InjectionConfig;
    // features:
    isLoading: boolean;
    status: string;
    currentPage: "home" | "features";
}
