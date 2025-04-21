# 🛰️ AIRCODE

> modular encryption & covert transmission framework  
> built for local execution, system access, and spy-grade style  

---

## 🔍 overview

**AIRCODE** is a high-performance encryption and data transmission tool designed for secure message encoding, advanced key management, and unconventional delivery — including frequency-based transmission.

**built with:**  
- 🦀 rust (backend logic, encryption, hardware I/O)  
- 🖥️ tauri (native desktop container)  
- 🧬 html/css/js (frontend, interface, animations)  

**primary use cases:**  
- customizable encryption profiles (via json)  
- local message encryption + decryption  
- sound wave transmission of encrypted content  
- educational insight into how each encryption method works  
- spy-core aesthetic with a minimalist, animated interface  

---

## ✨ features

### 🔐 encryption engine  
- select encryption profiles (defined via `.json`)  
- supports basic (caesar, vigenère), modern (AES), and custom logic  
- load external profiles at runtime  
- client-side processing (no external comms)  
- optional “see how it works” explanation panel

### 📡 transmission modules  
- **audio wave transmission**: converts encrypted message into frequency pulses  
- **morse output**: optional flash + sound mode  
- **QR code export**: encoded message as QR image  
- **file drop**: save to encoded `.air` file format  

### 📁 encryption profile format (example)  
```json
{
  "name": "Vigenère Cipher",
  "type": "vigenere",
  "key_required": true,
  "key_type": "string",
  "description": "Encrypts alphabetic text using a polyalphabetic cipher based on a keyword.",
  "example_key": "AIRCODE",
  "rust_logic_ref": "profiles/vigenere.rs"
}
```

## 🧠 Educational Layer (Optional Toggle)

Real-time walkthrough of:

- Input transformation  
- Key handling  
- Encryption steps (interactive diagram or console step-through)  

---

## 🧱 Tech Stack

| Layer       | Tech                     | Purpose                          |
|-------------|--------------------------|----------------------------------|
| Backend     | Rust                     | Encryption, file I/O, audio      |
| Interface   | Tauri + HTML/CSS/JS      | Desktop delivery + UI            |
| Audio       | `rodio`, `cpal`, `hound` | Audio output + signal encoding   |
| Encryption  | Custom logic per profile | Selectable via JSON              |
| UI Engine   | React or Svelte (TBD)    | Interactive, animated interface  |

---

## 📂 Structure

```
aircode/
├── src-tauri/
│   ├── main.rs
│   ├── profiles/
│   │   ├── caesar.rs
│   │   └── vigenere.rs
│   └── transmission/
│       ├── audio.rs
│       └── morse.rs
├── src/
│   ├── index.html
│   ├── App.tsx / App.svelte
│   └── components/
│       └── EncryptionUI.tsx
├── profiles/
│   └── user_custom.json
└── public/
    └── assets/
```


---

## 🚀 MVP Goals

- ✅ Load encryption profiles (from `.json`)  
- ✅ Encrypt/decrypt plaintext  
- ✅ Audio frequency transmission (basic tone mapping)  
- ✅ Dark-mode terminal UI  
- ✅ QR export module  
- ✅ “Explain encryption” walkthrough  
- 🔜 Mic-based decoding (stretch goal)  

---

## 🧪 Sample Commands (Planned)

- Select profile → type message → provide key → encrypt → transmit  
- Drag in `.json` → live load profile  
- Play audio → other device listens via mic  
- Export QR for peer to scan  

---

## 🔒 Security Notice

AIRCODE is not a production-grade secure comms system.  
It is a **conceptual, educational, and experimental tool**.  
Do **not** use for classified or personal data in high-risk scenarios.

---

## 📜 License

MIT (open-source under weird tool vibes clause #001).
