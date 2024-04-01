import { createSignal } from "solid-js";
import logo from "./assets/polarsteps-unpacker-logo.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [path, setPath] = createSignal("");
  const [file, setFile] = createSignal();

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: path() }));
  }

  async function generateFile() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setFile(await invoke("generate_file", { data_path: path() }));
  }

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>
      <h2>Polarsteps Unpacker is coming soon!!!</h2>

      <div class="row">
        <a href="https://google.de" target="_blank">
          <img src={logo} class="logo solid" alt="Polarsteps unpacker logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="filepath"
          onChange={(e) => setPath(e.currentTarget.value)}
          placeholder="Enter the path to your user data..."
        />
        <button type="submit">Greet!</button>
      </form>

      <form class="row"
        onSubmit={(e) => {
          e.preventDefault();
          generateFile();
        }}>
        <button type="submit">Generate!</button>
      </form>

      <p>{path()}</p>
    </div>
  );
}

export default App;
