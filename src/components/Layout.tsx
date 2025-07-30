import React from "react";
import TitleBar from "./TitleBar.tsx";

interface LayoutProps {
    children: React.ReactNode;
}

export default function Layout({ children }: LayoutProps) {
    return (
        <div className="app-container">
            <TitleBar />
            <div className="app-content">{children}</div>
        </div>
    );
}
