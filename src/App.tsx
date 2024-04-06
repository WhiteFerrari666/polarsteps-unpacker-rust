import { createSignal } from "solid-js";
import logo from "./assets/polarsteps-unpacker-logo.png";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  // createSignal("") -> Store mit leerem String initialisieren
  const [path, setPath] = createSignal("");
  const [greet, setGreeting] = createSignal("");
  const [file, setFile] = createSignal("");

  async function printPathMessage() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreeting(await invoke("show_path", { path: path() }));
  }

  async function generateFile() {
    setPath(await invoke("generate_file", { data_path: path() }));
    console.log(path())
  }

  return (
    <div class="container">
      <h1>Welcome to Polarsteps Unpacker!</h1>
      <h2>coming soon</h2>

      <div class="row">
        <a href="https://github.com/WhiteFerrari666/polarsteps-unpacker-rust.git" target="_blank">
          <img src={logo} class="logo solid" alt="Polarsteps unpacker logo"/>
        </a>
      </div>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          printPathMessage();
        }}
        autocomplete="off"
      >
        <input
          id="filepath-input" class="input"
          onChange={(e) => setPath(e.currentTarget.value)}
          placeholder="Enter the path to your user data..."
        />
        <button type="submit">Print path</button>
      </form>

      <form class="row"
        onSubmit={(e) => {
          e.preventDefault();
          generateFile();
        }}>
        <button type="submit">Generate!</button>
      </form>

      <p>{greet()}</p>
    </div>
  );
}

export default App;
