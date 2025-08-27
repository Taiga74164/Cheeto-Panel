import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./App.css";
import { emit } from "@tauri-apps/api/event";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);

emit("frontend-ready");