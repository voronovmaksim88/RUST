import { invoke } from "@tauri-apps/api/core";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let portsResultEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function scanPorts() {
  if (portsResultEl) {
    portsResultEl.textContent = "Scanning...";
    try {
      const result = (await invoke<{ ports: number[] }>("scan_available_ports_cmd"));
      if (result.ports.length === 0) {
        portsResultEl.textContent = "No COM ports found.";
      } else {
        portsResultEl.textContent = `Found ports: ${result.ports.map((p) => `COM${p}`).join(", ")}`;
      }
    } catch (e) {
      portsResultEl.textContent = `Error: ${e}`;
    }
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  portsResultEl = document.querySelector("#ports-result");
  document.querySelector("#scan-ports-btn")?.addEventListener("click", () => {
    scanPorts();
  });
});
