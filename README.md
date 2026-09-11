# SysGS 🚀

> A fast and lightweight Git repository summary tool written in Rust.

# Example

Python (Django): ![IMG_20260910_220129_204.jpg](https://github.com/user-attachments/assets/0a460760-6823-4d2b-a5fd-218936e218a6)

C (Linux): ![IMG_20260910_220125_668.jpg](https://github.com/user-attachments/assets/62c0f555-1607-413d-97b8-a65a1798905f)

Rust (SysPrint): ![IMG_20260910_220127_659.jpg](https://github.com/user-attachments/assets/c085b5b3-8f01-4bc7-a118-03bc93ef8286)

# ⚡ Features
* **Fast repository analysis:** Counts total files, lines of code, commits, headers, name project, programming language in project, authors, and more.
* **Language breakdown:** Displays precise percentage distribution of programming languages, and print logos.
* **Lightweight:** Built with performance and minimal dependencies in mind.

# 📦 Installation

## From Source
Ensure you have Rust and Cargo installed:
```bash
git clone https://github.com/MBKCHEL/SysGS.git
cd SysGS
cargo install --path .
```
## Binary (soon)

## Linux / MacOS / BSD
``` bash
chmod +x sysgs-linux # Linux, if you have MacOS or BSD write sysgs-bsd or sysgs-macos-intel / sysgs-macos-arm64
sudo mv sysgs-linux /usr/local/bin/sysgs # Linux, if you have MacOS or BSD write sysgs-bsd or sysgs-macos-intel / sysgs-macos-arm64
```
## Windows 
``` bash
Rename-Item .\sysgs-windows.exe sysgs.exe
Move-Item .\sysgs.exe C:\Windows\System32\
```
# Uninstall
## Linux / BSD / MacOS
``` bash
sudo rm /usr/local/bin/sysgs
```
## Windows
``` bash
Remove-Item C:\Windows\System32\sysgs.exe
```
# Licence
Distributed under the [GNU General Public License v3.0](LICENSE).
