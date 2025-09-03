import { BrowserRouter, Route, Routes, useNavigate, useLocation } from "react-router-dom";
import { useState, useEffect } from "react";
import Layout from "./components/Layout.tsx";
import Home from "./pages/Home";
import Features from "./pages/Features";

function AppRoutes() {
    const navigate = useNavigate();
    const location = useLocation();
    const [loadedModuleName, setLoadedModuleName] = useState<string>("");

    // Initialize loaded module from location state or clear it
    useEffect(() => {
        if (location.state?.moduleName) {
            setLoadedModuleName(location.state.moduleName);
        }
    }, [location.state]);

    const handleInjectionSuccess = (moduleName: string) => {
        setLoadedModuleName(moduleName);
        navigate("/features", { state: { moduleName } });
    };

    const handleBackToHome = () => {
        navigate("/");
    };

    const handleUnloadSuccess = () => {
        setLoadedModuleName("");
        navigate("/");
    };

    return (
        <Layout>
            <main className="p-4">
                <Routes>
                    <Route
                        path="/"
                        element={
                            <Home onInjectionSuccess={handleInjectionSuccess} loadedModuleName={loadedModuleName} />
                        }
                    />
                    <Route
                        path="/features"
                        element={
                            <Features
                                onBackToHome={handleBackToHome}
                                loadedModuleName={loadedModuleName}
                                onUnloadSuccess={handleUnloadSuccess}
                            />
                        }
                    />
                </Routes>
            </main>
        </Layout>
    );
}

export default function App() {
    return (
        <BrowserRouter>
            <AppRoutes />
        </BrowserRouter>
    );
}
