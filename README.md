![Release Build](https://github.com/websmithcode/automatic-brightness/actions/workflows/release.yml/badge.svg)

# Automatic Brightness

Automatic Brightness is a time-based brightness controller written in Rust. It supports both Linux and Windows operating systems.

## Features

- Time-based brightness adjustment
- Cross-platform support (Linux and Windows)

## About

This is my first package written in Rust.

## Installation

Instructions for installing the package will go here.

## Usage

Just tun package and it will automatically adjust the brightness of your screen.

## Installation
You can just download builded binary for Linux or Windows from [latest release](https://github.com/websmithcode/automatic-brightness/releases/latest)
Now tested only for windows, but should works and on linux.


### Build Steps
#### Prerequisites
- Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).


1. Clone the repository:
  ```sh
  git clone https://github.com/websmithcode/automatic-brightness.git
  ```
2. Navigate to the project directory:
  ```sh
  cd automatic-brightness
  ```
3. Build the project:
  ```sh
  cargo build --release
  ```
4. Run the executable:
  ```sh
  ./target/release/automatic-brightness
  ```

For Windows, the executable will be located at `.\target\release\automatic-brightness.exe`.

























