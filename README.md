# Translator Agent

A simple mock translator agent built for UOMI testnet.

## What it does

Takes a text string and a target language code (`hi`, `fr`, `ur`) and returns the translated version.  
Pure, clean, no rephrasing — only exact mapped translation.

## Input schema:
```json
{
  "text": "Hello",
  "target_language": "hi"
}

Output schema:

{
  "translated_text": "नमस्ते"
}

Build

cargo build --target wasm32-unknown-unknown --release

Resulting .wasm file can be deployed on UOMI Finney testnet.
