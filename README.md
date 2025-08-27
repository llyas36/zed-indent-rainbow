# Indent Rainbow for Zed

Adds a fun rainbow color effect to indentation in Zed editor.  
Currently, this is a minimal dev extension skeleton — the highlighting logic will be added in future updates.  

## Installation (Dev)

1. Clone this repo:

```bash
git clone https://github.com/llyas36/zed-indent-rainbow.git
cd zed-indent-rainbow
```
## Build the extension:
cargo build --release --target wasm32-wasip1

    Open Zed, run Install Dev Extension, and select this folder.

    Run Zed with logs to verify:

zed --foreground
