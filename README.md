# RustCorder

Welcome to RustCorder, your new favorite audio recording tool! RustCorder is a system-agnostic audio recorder module built with the power of Rust. It's like a tape recorder, but without the tape... or the physical recorder. 

## Idea

In the age of digital audio, we often find ourselves needing to record and process audio data. Whether you're building a voice recognition system, a music production software, or just want to record your own podcast, RustCorder is here to help.

The idea behind RustCorder is to provide a simple, yet powerful, audio recording module that can be easily integrated into any project. It's built with Rust, which means it's fast, safe, and concurrent. But the real beauty of RustCorder is its system-agnostic design. Whether you're on Windows, MacOS, or Linux, RustCorder has got you covered.

## Current Functionality

RustCorder currently supports streaming audio data via pipes to the outside user. This means you can easily integrate it with other parts of your system, or even other systems. Need to send your audio data to a server for processing? No problem. Want to pipe it into another application for further processing? Easy peasy.

In addition to streaming, RustCorder also supports saving the audio data as a .wav file. So if you just want to record some audio and save it for later, RustCorder can do that too.

`NOTE: Not all functionality is currently implemented. This is a work in progress guys, you understand. Nonetheless, the README is already a work of art.`

## Future Plans

We're always looking to improve RustCorder. Some of our future plans include adding support for more audio formats, improving the streaming capabilities, and making the API even easier to use.

So why wait? Start recording with RustCorder today!

## Building the Project

To build the project, you can use the following command:

```sh
cargo build
```

And to run it:

```sh
cargo run
```

Happy recording!
