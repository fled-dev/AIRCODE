import { invoke } from '@tauri-apps/api/core'; // Updated import for Tauri v2

let profileInput;
let messageInput;
let keyInput;
let resultEl;
let encryptForm;
let switchProfileButton;

window.addEventListener("DOMContentLoaded", () => {
  profileInput = document.querySelector("#profile");
  messageInput = document.querySelector("#message");
  keyInput = document.querySelector("#key");
  resultEl = document.querySelector("#result");
  encryptForm = document.querySelector("#encrypt-form");
  switchProfileButton = document.querySelector("#switch-profile");

  encryptForm.addEventListener("submit", (e) => {
    e.preventDefault();
    encrypt();
  });

  switchProfileButton.addEventListener("click", () => {
    if (profileInput.value.includes("caesar")) {
      profileInput.value = "../profiles/vigenere.json";
      switchProfileButton.textContent = "Switch to Caesar";
    } else {
      profileInput.value = "../profiles/caesar.json";
      switchProfileButton.textContent = "Switch to Vigenere";
    }
  });
});

async function encrypt() {
  try {
    const result = await invoke("encrypt_message", {
      message: messageInput.value,
      key: keyInput.value,
      profilePathStr: profileInput.value,
    });
    resultEl.textContent = result;
  } catch (error) {
    resultEl.textContent = `Error: ${error}`;
  }
}
