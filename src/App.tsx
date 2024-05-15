import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@nextui-org/button";
import {Input} from "@nextui-org/input";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="pt-6 flex flex-col justify-center items-center">
      <h1 className="text-3xl font-bold">
        Hello world!
      </h1>

      <div className="flex flex-row justify-center">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="h-24 p-6 hover:drop-shadow-2xl transition-all duration-700" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="h-24 p-6 hover:drop-shadow-2xl transition-all duration-700" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="h-24 p-6 hover:drop-shadow-2xl transition-all duration-700" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="flex flex-row items-center"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <Input
          className="mr-4"
          onChange={(e) => setName(e.currentTarget.value)}
          label="Name:"
        />
        <Button color="primary" type="submit">
          Greet
        </Button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
