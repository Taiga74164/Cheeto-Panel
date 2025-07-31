import React from "react";
import { Check, X, Loader } from "lucide-react";

interface StatusIndicatorProps {
    status: "ready" | "not-found" | "loading";
    message: string;
}

export const StatusIndicator: React.FC<StatusIndicatorProps> = ({ status, message }) => {
    const getStatusConfig = () => {
        switch (status) {
            case "ready":
                return {
                    icon: <Check className="w-4 h-4" />,
                    className: "bg-green-500/20 border-green-500/30 text-green-200",
                    iconBg: "bg-green-500",
                };
            case "loading":
                return {
                    icon: <Loader className="w-4 h-4 animate-spin" />,
                    className: "bg-blue-500/20 border-blue-500/30 text-blue-200",
                    iconBg: "bg-blue-500",
                };
            default:
                return {
                    icon: <X className="w-4 h-4" />,
                    className: "bg-red-500/20 border-red-500/30 text-red-200",
                    iconBg: "bg-red-500",
                };
        }
    };

    const config = getStatusConfig();

    return (
        <div className={`flex items-center space-x-3 p-3 rounded-lg border ${config.className}`}>
            <div className={`w-8 h-8 rounded-full flex items-center justify-center ${config.iconBg}`}>
                {config.icon}
            </div>
            <div>
                <p className="text-sm font-medium">{message}</p>
            </div>
        </div>
    );
};
