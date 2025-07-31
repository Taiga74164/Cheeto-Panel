import { BrowserRouter, Route, Routes, useNavigate } from "react-router-dom";
import Layout from "./components/Layout.tsx";
import Home from "./pages/Home";
import Features from "./pages/Features";

function AppRoutes() {
    const navigate = useNavigate();

    return (
        <Layout>
            <main className="p-4">
                <Routes>
                    <Route path="/" element={<Home onInjectionSuccess={() => navigate("/features")} />} />
                    <Route path="/features" element={<Features onBackToHome={() => navigate("/")} />} />
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
