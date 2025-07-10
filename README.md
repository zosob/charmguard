# 🧘‍♂️ CharmGuard

> *Sip tea. Block distractions. Stay focused.*  
> Now with a GUI and AI Chatbot Assistant.

CharmGuard is a **local-first**, distraction-blocking productivity assistant built in **Rust**.  
It empowers deep focus through gamification, AI-powered drift detection, and intelligent distraction handling.

---

## ✨ Features

| Feature                        | CLI | GUI | Rasa |
|-------------------------------|:---:|:---:|:----:|
| Focus timer                   | ✅  | ✅  | —    |
| Distraction logging           | ✅  | ✅  | —    |
| Auto-block distracting windows| ✅  | ❌  | —    |
| Focus drift prediction        | ✅  | ❌  | —    |
| Charm rewards system          | ✅  | ✅  | —    |
| Inspirational card deck       | ✅  | ❌  | —    |
| AI Assistant (Rasa)           | ❌  | ✅  | ✅    |

---

## 🚀 Getting Started

### 🔧 Prerequisites

- Rust (`cargo`, `rustc`)
- Python 3.x with:
  - `pandas`
  - `scikit-learn`
- [Rasa](https://rasa.com/) (for chatbot functionality)
- Windows OS (currently supported)

---

### 🧠 Build & Run

#### 📦 Clone and Build
```bash
git clone https://github.com/yourusername/charmguard.git
cd charmguard
cargo build --release
```

#### 🖥️ Run CLI
```bash
cargo run --bin charmguard -- --focus 30 --track --charms
```
#### Draw an inspiration card:
```bash
cargo run --bin charmguard -- card
```
#### Show charm progress:
```bash
cargo run --bin charmguard -- --charms
```

#### 💬 Run GUI
```bash
cargo run --bin charmguard-gui
```

### GUI Highlights
- Start/stop focus sessions
- View charm progress
- Talk to your Rasa-powered AI assistant

# 🤖 Rasa Integration
Make sure your Rasa server is running:
```bash
rasa run actions &
rasa shell --enable-api
```
CharmGuard sends messages to Rasa via HTTP API at:
http://localhost:5005

## ✅ Roadmap

- [ ] Charm leaderboard  
- [ ] “Focus Mode” — limit all inputs  
- [ ] Export sessions to Markdown or HTML  
- [ ] Pomodoro mode  
- [ ] Voice input support via Rasa  

---

## 🤝 Credits

Built with 💙 by **Bhaskar Ghosh**  
Thanks to **Hannah** for the inspiration  

- Chatbot powered by **Rasa**  
- GUI built with **eframe**  
- ML drift detection via **scikit-learn**


### Copyright © Bhaskar Ghosh.  
CharmGuard is licensed for internal use only. Redistribution, modification, or commercial use without explicit permission is prohibited.






