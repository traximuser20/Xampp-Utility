<div align="center">

# 🚀 Automated XAMPP Utility Manager

![PowerShell](https://img.shields.io/badge/PowerShell-%235391FE.svg?style=for-the-badge&logo=powerShell&logoColor=white)
![XAMPP](https://img.shields.io/badge/XAMPP-FB7A24?style=for-the-badge&logo=xampp&logoColor=white)

<!-- ![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge) -->

_A beautiful, interactive Text User Interface (TUI) script for safely and efficiently backing up your XAMPP server environments._

</div>

---

## 🌟 Overview

Tired of manually copying your `htdocs` or dragging your `mysql/data` around when moving or securing your local development environment? **Automated XAMPP Utility Manager** handles the tedious work for you. Built entirely in PowerShell, it manages the critical components of your XAMPP installation through an elegant, dynamically-centered terminal interface.

## ✨ Features

- **🎨 Interactive TUI:** A fully terminal-based graphical menu for an improved user experience.
- **🖥️ Responsive Design:** Dynamic layout centers perfectly regardless of your terminal width.
- **📦 Smart Backup:** Identifies and backs up only the essential parts of XAMPP:
  - `htdocs` (Your website files)
  - `mysql\data` (Your databases)
  - `apache\conf` (Apache configurations)
  - `mysql\bin\my.ini` and `php\php.ini` (Key config files)
- **📊 Native Progress Tracking:** Real-time visual progress bar during the backup process.
- **🗜️ Automatic Compression:** Creates an optimized `.zip` archive automatically.
- **📝 Detailed Logging:** Logs all operations with timestamps to a pristine text file for easy auditing.

---

## 🚀 Getting Started

### Prerequisites

- Windows Operating System
- **PowerShell** (Integrated into modern Windows, version 5.1+)
- **XAMPP** installed on your system

### Installation & Usage

#### 🚀 Option 1: Direct Run (Recommended)

Run the script directly from GitHub without manual downloading:

```powershell
powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/traximuser20/Xampp-Utility/main/xampp_utility.ps1 | iex"
```

#### 📂 Option 2: Manual Download

1. **Download** the [`xampp_utility.ps1`](https://github.com/traximuser20/Xampp-Utility/blob/main/xampp_utility.ps1) script to your computer.
2. **Open PowerShell**.
3. **Navigate** to the directory containing the script.
4. **Run the script**:
   ```powershell
   .\xampp_utility.ps1
   ```
5. Choose an option from the beautifully centered on-screen **Main Menu**:
   - `[1] Start Full Backup`
   - `[2] Configure XAMPP Directory`
   - `[3] Configure Backup Destination`
   - `[4] Exit`

> **Note:** If you encounter an "Execution Policy" error when trying to run the script, open PowerShell as an Administrator and enter: `Set-ExecutionPolicy RemoteSigned -Scope CurrentUser`, then press `Y`.

---

## ⚙️ Configuration

By default, the script looks for XAMPP at `C:\xampp` and saves backups to `D:\xampp_backups`.
You can easily change these directly from the in-app TUI using options `[2]` and `[3]`. The script dynamically adjusts to your custom paths for the current session!

---

## 📸 Sneak Peek

_Run the script to be greeted by a custom ASCII banner and a clean interface that respects your terminal's dimensions! The script gracefully reports its status (`[+]`, `[_]`, `[!]`, `[X]`) keeping you informed at every step.\*

---

## 👨‍💻 Author

Created with by **Azeem Ali**

> _"Make backups a habit, not an afterthought."_

---

<div align="center">
  <i>If you find this utility helpful, consider starring the repository or sharing it with your fellow developers!</i>
</div>
