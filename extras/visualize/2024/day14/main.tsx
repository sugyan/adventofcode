import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./../../src/index.css";
import Day14 from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Day14 />
  </StrictMode>
);
