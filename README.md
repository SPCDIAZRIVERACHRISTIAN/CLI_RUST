# 📘 Rust CLI Utility — How to Use

A simple and fast command-line tool written in **Rust** to search patterns inside files (`grrs`) and analyze word/line counts (`wcs`).  
Powered by **clap**, **anyhow**, and pure Rust performance.

---

## 🚀 Installation

Clone the repository and build the project:

```bash
git clone <your-repo-url>
cd CLI
cargo build --release

Or run directly without building:

cargo run -- <command> <arguments>

🧩 Available Commands

This CLI provides two subcommands:

1. grrs — Search for a pattern inside a file

Equivalent to a simplified version of grep.

## Usage:

cargo run -- grrs <pattern> <file>

## Arguments

| Argument    | Description                        |
| ----------- | ---------------------------------- |
| `<pattern>` | The text you want to search for    |
| `<file>`    | The path to the file to be scanned |

2. wcs — Count words and lines in a file

Similar to a minimal version of wc.

Usage:

cargo run -- wcs <file>


Arguments:


| Argument | Description         |
| -------- | ------------------- |
| `<file>` | The file to analyze |

---

## CLI V1

It finally works! It can use better error handling and
more features than just the one it has but I dont care it served its purpose.
the only thing you need to do to make it work in your machine is git clone the
project
```bash
cargo install --path .
```

and boom you have one command with two subcommands
made with rust.
If you have any questions or doubts feel free to ask google
or the AI of your preference because this is not meant to be
useful in any way it was just made to learn rust.
Thank you.
