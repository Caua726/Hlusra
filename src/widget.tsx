import React from "react";
import ReactDOM from "react-dom/client";
import RecordingWidget from "./components/RecordingWidget";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RecordingWidget />
  </React.StrictMode>,
);
