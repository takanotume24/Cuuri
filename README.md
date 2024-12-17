<p align="center">
    <img src=https://github.com/takanotume24/cuuri/blob/master/src-tauri/icons/Square284x284Logo.png?raw=true width=138/>
</p>
<h1 align="center">Cuuri</h1>

Cuuri is a GUI client for ChatGPT built with Tauri, Vue, and TypeScript.

![Screenshot](./public/screenshot.png)

### Features

- Privacy
  - Your data is not sent anywhere except to OpenAI. All data resides within your storage. No debug logs are sent.

### Installation

1. Download or build the Cuuri binary.
   - macOS
     - Unfortunately, you need to build from source. It seems that I need to sign the binaries to make them executable on macOS. I plan to address this in the future.
   - Windows
     - Download and run `Cuuri_[version]_x64-setup.exe` from <https://github.com/takanotume24/Cuuri/releases>.
   - Linux
     - Download your preferred executable format from <https://github.com/takanotume24/Cuuri/releases>.
2. Launch Cuuri.

### Build Instructions

1. Install the necessary dependencies by following the guide at <https://v2.tauri.app/start/prerequisites/>.
1. Clone this repository.

    ```bash
    $ https://github.com/takanotume24/Cuuri.git
    $ cd Cuuri
    ```

1. Install npm dependencies.

    ```bash
    $ npm install
    ```
  
1. Build the project.

    ```bash
    $ npm run tauri build
    ```

1. Run the built binary to install.
   - macOS: `src-tauri/target/release/bundle/dmg`
   - Windows: `src-tauri/target/release/`

### Customize

All Cuuri data is saved under `$HOME/.cuuri`. Typically, the following files are generated automatically, so there's no need to edit them manually.

- `$HOME/.cuuri/chat.db`: The chat history is saved here.
- `$HOME/.cuuri/config.toml`
  - `default_model`: You can set the model that is selected at startup. Make sure the model name matches the one shown in the list.
  - `openai_api_key`: Set your OpenAI API key here.

