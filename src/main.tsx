import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import {NextUIProvider} from '@nextui-org/react'
import MainMenu from "./MainMenu";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <NextUIProvider>
      <MainMenu />
    </NextUIProvider>
  </React.StrictMode>,
);
