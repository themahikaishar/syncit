# SyncIt 🔄  

**A Powerful Bidirectional File Synchronizer in Rust**

SyncIt is a **two-way (bidirectional) directory synchronization tool** written in **Rust**.  
It is designed for situations where **both sides matter** — for example:

- You edit files on your **PC**
- You edit the same project on your **mobile / laptop / another device**
- Later, you want to **sync both directories without losing data**

Unlike one-way backup tools, **SyncIt treats both directories as equals** and lets **you decide** when conflicts occur.

---

## ✨ Why SyncIt?

Most sync tools assume one directory is the “source” and the other is a “backup”.

**SyncIt does not.**

Instead:

- Both directories are **equally important**
- Files missing on one side are **copied**
- Files present on both sides are **compared**
- If contents differ, **you are prompted to choose which version to keep**

No silent overwrites. No guessing. You stay in control.

---

## 🧠 Key Features

- ✅ True **bidirectional synchronization**
- ✅ **Byte-level file comparison**
- ✅ Interactive conflict resolution
- ✅ Recursive directory traversal
- ✅ No external dependencies
- ✅ Written in safe, fast **Rust**

---

## 📦 Current Status

> ⚠️ **Early / Experimental**

This project is actively evolving.  
Some improvements still planned:

- Better error handling (remove `unwrap()`)
- Dry-run mode
- Ignore rules (`.syncitignore`)
- Non-interactive / auto-resolve modes
- Logging and progress display
- Cross-platform testing

---

## 🤝 Contributing

**Contributions are welcome and encouraged!**

You can help by:

- Improving error handling
- Refactoring traversal logic
- Adding features (CLI, config files, hashing, etc.)
- Writing tests
- Improving documentation

### How to contribute

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a Pull Request

All contributors are appreciated ❤️

---

## 📜 License

This project is **open source** and licensed under:

**GNU General Public License v2.0**

- You are free to use, modify, and distribute this software
- Derivative works must remain open source under GPL v2.0

See the `LICENSE.md` file for full details.

---

## 💡 Philosophy

> “Both sides matter.”

SyncIt exists because real work happens on multiple devices —  
and **you should never have to guess which file is more important**.


---