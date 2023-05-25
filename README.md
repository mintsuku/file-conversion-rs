# Rust File Conversion Tool

This is a command-line file conversion tool written in Rust that allows you to convert PNGs, MP4s, JPEGs, and GIFs. It utilizes FFmpeg for the actual conversion process. With this tool, you can easily convert your media files between different formats with just a few simple steps.

## Prerequisites

Before using this tool, you need to ensure that FFmpeg is installed on your system, This is neccasary for any video conversions. Follow the instructions below based on your operating system:

## Mac

To install FFmpeg on macOS using Homebrew, open a terminal and run the following command:
```
brew install ffmpeg
```

## Windows 
On Windows, you can download a pre-built FFmpeg binary from the official website. Follow these steps:

- Visit the FFmpeg download page on the official FFmpeg website.
- Scroll down to the "Windows" section.
- Under "Static Builds," click on the link that corresponds to your system architecture (32-bit or 64-bit).
- Download the ZIP file and extract its contents to a directory of your choice.
- the FFmpeg binary directory to your system's PATH environment variable. This step is important for the tool to find and execute FFmpeg correctly.

## Linux

On Linux, you can install FFmpeg using the package manager specific to your distribution. Open a terminal and run the appropriate command:

- Ubuntu or Debian-based distributions:
```
sudo apt-get install ffmpeg
```

- Fedora or CentOS-based distributions:
```
sudo dnf install ffmpeg
```

Make sure you have `cargo` and `git` installed on your system as well. If they are not already installed, you can install them using the package manager specific to your operating system.

## Installation

To install and use this file conversion tool, follow the steps below:

- Clone the repository by running the following command in your terminal:
```
git clone https://github.com/shelovesmox/file-conversion-rs.git
```

- Navigate to the project directory:
```
cd file-convesion-rs
```

- Build using cargo:
```
cargo build
```

- Once the build is complete, you can run the tool with the following command:

```
cargo run
```

That's it! Just follow the options the UI to convert whatever you want!







