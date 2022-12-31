import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [omikujiMsg, setOmikujiMsg] = useState("");

  async function omikuji() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setOmikujiMsg(await invoke("omikuji"));
  }

  return (
    <div className="container">
      <h1>Happy New Year!</h1>

      <p>Click on the Omikuji button.</p>

      <div className="row">
        <div>
          <button type="button" onClick={() => omikuji()}> Omikuji </button>
        </div>
      </div>
      <p>{omikujiMsg}</p>
    </div>
  );
}

export default App;
