import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./../../src/index.css";
import Day18 from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Day18 />
  </StrictMode>
);
