# SysGS 🚀

> A fast and lightweight Git repository summary tool written in Rust.

# Example

Python (pokeapi): <img width="1280" height="563" alt="image" src="https://github.com/user-attachments/assets/49d17ca6-ff62-4ccd-8e40-6163a464770e" />

C (Linux): <img width="1280" height="603" alt="image" src="https://github.com/user-attachments/assets/9c0eb04e-f7ad-431b-81d6-792972b4cc3a" />

Rust (SysPrint): <img width="1280" height="594" alt="image" src="https://github.com/user-attachments/assets/a890654a-5e90-4383-9952-cfcfc1e1425e" />

Java (java-design-patterns): <img width="1280" height="566" alt="image" src="https://github.com/user-attachments/assets/f6d979ce-e4c9-4126-88aa-1f34649472c3" />

CPP (wfrest): <img width="1280" height="569" alt="image" src="https://github.com/user-attachments/assets/8fd50de8-9bb2-49a0-89a6-7f4db5ac011d" />

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
## Binary

## Linux / MacOS / BSD
``` bash
cd Downloads # or Загрузки
chmod +x sysgs-linux # Linux, if you have MacOS or BSD write sysgs-bsd or sysgs-macos-intel / sysgs-macos-arm64
sudo mv sysgs-linux /usr/local/bin/sysgs # Linux, if you have MacOS or BSD write sysgs-bsd or sysgs-macos-intel / sysgs-macos-arm64
```
## Windows 
``` powershell
# Run PowerShell as Administrator and execute the following commands:
Rename-Item -Path ".\sysgs-windows.exe" -NewName "sysgs.exe"
Move-Item -Path ".\sysgs.exe" -Destination "C:\Windows\System32\"
```
# Uninstall
## Linux / BSD / MacOS
``` bash
sudo rm /usr/local/bin/sysgs
```
## Windows
``` powershell
Remove-Item C:\Windows\System32\sysgs.exe
```
# Licence
Distributed under the [GNU General Public License v3.0](LICENSE).
