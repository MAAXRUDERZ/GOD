GOD

GOD is a command-line documentation tool for Linux.

It started as a small project because I found myself constantly switching between TLDR pages and man pages. TLDR is great when you just want an example, but it often leaves out useful information. Man pages have everything, but they can be overwhelming for everyday use.

The goal of GOD is to combine both into a single interface that is easier to read while still providing the details when you need them.

Screenshots

- Startup

  ![](assets/screenshots/home.png)

- Looking up the ls command

  ![](assets/screenshots/ls.png)

- Looking up the cat command

  ![](assets/screenshots/cat.png)

Building

- Clone the repository.

      git clone https://github.com/MAAXRUDERZ/GOD.git

- Enter the project directory.

      cd GOD

- Build the project.

      cargo build --release

Installing

- Install GOD locally.

      cargo install --path .

Usage

- Show documentation for a command.

      god ls

- Display every available flag.

      god ls --all-flags

Current Features

- Combines TLDR pages with man pages.
- Clean terminal interface.
- Practical examples.
- Important flags shown by default.
- Full flag list available on demand.
- ANSI color output.
- Works completely offline.

Why?

I wanted a tool that was faster to read than man pages without losing the information that makes them useful. GOD is my attempt at solving that problem by presenting the most important information first while still making the complete documentation available when it's needed.

Roadmap

- Improve man page parsing.
- Better selection of important flags.
- Search documentation by keyword.
- Related command suggestions.
- Shell completion.

