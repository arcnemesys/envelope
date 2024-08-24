## Overview



Envelope is a very simple application, that allows for viewing and updating the environment variables, and viewing the components of the path variable on your system. The main driver behind the development of Envelope was to further my understanding of:
- [Ratatui](https://ratatui.rs/), a library for building TUIs, that underlies *more than a few* [exceptional applications](https://github.com/ratatui/awesome-ratatui?tab=readme-ov-file#-apps).
- Terminals/shells, which have an air of opaqueness and mystique around them, yet are essential to the craft of programming.
- Immediate mode, an API pattern for GUI development that differs meaningfully from the Retained mode pattern, which is much more commonly used/experienced.

Currently, I'm exploring ways to make Envelope a bit more, well, useful, and am considering implementing some, perhaps all of the following features:
- Searching for specific environment variable names and values.
- Creating new environment variables and exporting them.
- [In Progress] Editing/writing to .bashrc to set environment variables for future shell sessions.
Editing/writing to /etc/environment to set environment variables system-wide.
