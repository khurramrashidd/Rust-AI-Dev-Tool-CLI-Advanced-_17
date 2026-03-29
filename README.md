# 🦀 Rust AI Dev Tool CLI (Advanced)

A powerful **AI-powered Developer CLI Tool written in Rust** that analyzes, summarizes, and fixes code using the **Gemini API (gemini-3-flash-preview model)**.

This project demonstrates real-world CLI design with commands, file processing, and AI integration.

This project is part of my **Rust learning journey (Day 17 Project)**.

---

## 🚀 Features

* Analyze full project (`analyze`)
* Generate summary (`summary`)
* Fix and improve code (`fix`)
* Save output to file (`--output`)
* Colored terminal output
* Multi-language support
* Secure API key using `.env`

---

## 🛠 Built With

* **Rust**
* `clap`
* `reqwest`
* `serde_json`
* `dotenv`
* `walkdir`
* `colored`
* Google Gemini API

---

## ▶️ Usage

```bash
cargo run -- analyze --path <project_path>
cargo run -- summary --path <project_path>
cargo run -- fix --path <project_path> --output result.txt
```

---

## 🧠 Concepts Practiced

* Advanced CLI design
* Command handling
* AI integration
* File processing
* Output management

---

## 👨‍💻 Author

**Khurram Rashid**