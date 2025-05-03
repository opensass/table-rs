# 📚 Table RS Dioxus Tailwind Components

## 🛠️ Pre-requisites:

### 🐧 **Linux Users**

1. **Install [`rustup`](https://www.rust-lang.org/tools/install)**:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

   ```sh
   cargo install dioxus-cli
   ```

### 🪟 **Windows Users**

1. **Download and install `rustup`**: Follow the installation instructions [here](https://www.rust-lang.org/tools/install).

1. **Install [Windows Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/install)**: Open PowerShell as administrator and run:

   ```sh
   wsl --install
   ```

1. **Reset Network Stack**: In PowerShell (administrator mode), run:

   ```sh
   netsh int ip reset all
   netsh winsock reset
   ```

1. **Install Linux packages in WSL**: Once inside your WSL terminal, update and install required dependencies:

   ```sh
   sudo apt update
   sudo apt install build-essential pkg-config libudev-dev
   ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

   ```sh
   cargo install dioxus-cli
   ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

   ```sh
   git clone https://github.com/opensass/table-rs
   ```

1. Navigate to the application directory.

   ```sh
   cd table-rs/examples/dioxus
   ```

1. Run the client:

   ```sh
   dx serve --port 3000
   ```

Navigate to http://localhost:3000 to explore the landing page.
