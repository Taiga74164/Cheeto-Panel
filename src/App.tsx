import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");
    const [loading, setLoading] = useState(false);
    const [status, setStatus] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        setGreetMsg(await invoke("greet", { name }));
    }

    async function toggleCheat(enabled: boolean) {
        setLoading(true);
        setStatus("");

        try {
            const response = await invoke<string>("send_feature_command", {
                feature: "InstantWin",
                enable: enabled,
            });

            setStatus(`Success: ${response}`);
            console.log("Cheat response:", response);
        } catch (error) {
            setStatus(`Error: ${error}`);
            console.error("Failed to toggle cheat:", error);
        } finally {
            setLoading(false);
        }
    }

    return (
        <main className="container">
            <h1>Welcome to Tauri + React</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo" />
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>
            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                }}
            >
                <input
                    id="greet-input"
                    onChange={(e) => setName(e.currentTarget.value)}
                    placeholder="Enter a name..."
                />
                <button type="submit">Greet</button>
            </form>
            <p>{greetMsg}</p>

            <div className="space-y-4 space-x-4">
                <button
                    onClick={() => toggleCheat(true)}
                    disabled={loading}
                    className="w-full bg-green-600 hover:bg-green-700 disabled:bg-slate-600
                    disabled:cursor-not-allowed text-white font-semibold py-3 px-6
                    rounded-lg transition-all duration-200 transform hover:scale-[1.02]
                    disabled:transform-none shadow-lg"
                >
                    {loading ? "Processing..." : "Enable Instant Win"}
                </button>

                <button
                    onClick={() => toggleCheat(false)}
                    disabled={loading}
                    className="w-full bg-red-600 hover:bg-red-700 disabled:bg-slate-600
                    disabled:cursor-not-allowed text-white font-semibold py-3 px-6
                    rounded-lg transition-all duration-200 transform hover:scale-[1.02]
                    disabled:transform-none shadow-lg"
                >
                    {loading ? "Processing..." : "Disable Instant Win"}
                </button>

                {status && (
                    <div
                        className={`p-3 rounded-lg ${status.startsWith("Error") ? "bg-red-100 text-red-800" : "bg-green-100 text-green-800"}`}
                    >
                        {status}
                    </div>
                )}
            </div>
        </main>
    );
}

export default App;
