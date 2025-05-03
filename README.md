# Termaite

Simple AI client for the terminal.

`Termaite` allows you to efficiently ask questions or give instructions to a LLM without leaving the terminal using a CLI that behaves like a shell, prompting you in a loop and providing the answers you seek.

# Features

* Chat history
* Multiple profiles
* Markdown rendering
* Easy configuration
* Powered by Gemini

## Install/run

Just download the latest version from the [releases](https://github.com/vascocosta/termaite/releases) page, then uncompress it to a location of your choosing and run the `termaite` executable. Additionally add this binary's folder to your PATH for convenience.

## Build

To build `termaite` you need the `Rust toolchain` as well as these `dependencies`:

* anyhow = "1.0.97"
* dirs = "6.0.0"
* gemini_client_rs = "0.3.0"
* serde = { version = "1.0.219", features = ["derive"] }
* serde_json = "1.0.140"
* termimad = "0.31.2"
* tokio = { version = "1.44.0", features = ["full"] }

Follow these steps to fetch and compile the source of `termaite` and its `dependencies`:

```
git clone https://github.com/vascocosta/termaite.git

cd termaite

cargo build --release
```

## Screenshots

![termaite_01](screenshots/termaite_01.png)
![termaite_02](screenshots/termaite_02.png)

## Configuration (samples)

When you run `termaite` for the first time it will create a default configuration file for you at the root of your home dir named .termaite.json. Please make sure you change at least the api_key to match your own, which you can obtain from Google's AI Studio website...

### .termaite.json (default)

```json
{
  "api_key": "YOUR_API_KEY",
  "active_profile": "default",
  "profiles": {
    "default": {
      "model_name": "gemini-2.0-flash",
      "chars": 4000,
      "system_prompt": [
        "Please reply in about {chars} chars."
      ]
    }
  },
  "color": "Blue"
}
```

### .termaite.json (learn-french)

```json
{
  "api_key": "YOUR_API_KEY",
  "active_profile": "learn-french",
  "profiles": {
    "default": {
      "model_name": "gemini-2.0-flash",
      "chars": 4000,
      "system_prompt": [
        "Please reply in about {chars} chars."
      ]
    },
    "learn-french": {
      "model_name": "gemini-2.0-flash",
      "chars": 2000,
      "system_prompt": [
        "You are an experienced French tutor who can speak your student's language too.",
        "Engage in conversations with a complexity matching the student's level of fluency.",
        "Be extremely helpful and patient, and use the student's language only if needed.",
        "Try to answer within {chars} chars but use more if needed."
      ]
    }
  },
  "color": "Green"
}
```