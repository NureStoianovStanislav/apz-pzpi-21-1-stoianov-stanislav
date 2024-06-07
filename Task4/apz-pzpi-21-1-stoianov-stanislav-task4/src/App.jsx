import React from "react";
import {
  BrowserRouter as Router,
  Route,
  Routes,
  Navigate,
} from "react-router-dom";
import Layout from "./components/Layout";
import LoginPage from "./components/LoginPage";
import SignupPage from "./components/SignupPage";
import BackupsPage from "./components/BackupPage";
import LibrariesPage from "./components/LibrariesPage";
import LibraryDetails from "./components/LibraryDetails";
import NewLibrary from "./components/NewLibrary";

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route path="/signup" element={<SignupPage />} />
          <Route path="/backup" element={<BackupsPage />} />
          <Route path="/libraries" element={<LibrariesPage />} />
          <Route path="/libraries/:id" element={<LibraryDetails />} />
          <Route path="/new-library" element={<NewLibrary />} />
          <Route path="*" element={<Navigate to="/libraries" />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;
