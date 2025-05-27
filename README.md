# Fscan  

A Rust CLI tool that:

* Runs **rustscan** to quickly find open ports on a target IP.
* Parses the output to extract open ports.
* Checks if **nmap** is installed; if not, installs it automatically based on OS (Arch or Debian-based).
* Caches the `nmap` installation status to skip redundant checks.
* Runs an `nmap` scan with service detection and default scripts on the discovered ports.

---
## Prerequisites

* Rust toolchain (stable) installed to build from source.
* `rustscan` installed and available in your PATH.
* (optional) sudo privileges to install `nmap` if not already installed.

---

## Installation

1. Clone the repo or save the Rust source file:

   ```bash
   git clone https://github.com/krypton-0x00/fscan
   cd fscan 
   ```

2. Build the binary:

   ```bash
   cargo build --release
   ```

3. Copy the binary to your $PATH:

   ```bash
   sudo cp /target/release/fscan /usr/local/bin
   ```

---

## Usage

```bash
./fscan 10.129.201.160
```
## How caching works

* On the first run, after installing or confirming `nmap`, the tool writes a small cache file to `~/.cache/fscan/nmap_installed`.
* On future runs, if this file exists, the tool skips the install check for speed.

