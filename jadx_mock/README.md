# JADX Screenshot Generator

The subject of this repository is a GUI mock of the [JADX](https://github.com/skylot/jadx) application.
It has been created to generate valid screenshots of the JADX application in order to train neural networks.

## Prerequisites

The application is written in Rust and includes a Bash script.
So, to execute the JADX Mock, [Rust](https://www.rust-lang.org/tools/install) has to be installed and some kind of Bash interpreter.

## Usage

To execute the generator, you just have to execute
1. Install prerequisites
2. Execute `bash generate_data.sh NUMBER_OF_IMAGES` (or `bash generate_data_linux.sh NUMBER_OF_IMAGES` in case you are on Linux) while NUMBER_OF_IMAGES is the number of images you want to create. This command automatically builds the project and runs the generator.
3. Inspect the generated images in the directory `target/images`

### Running the mock application
To run the Mock application (without taking a screenshot) run `cargo run --release`. Please note the `--release` flag. Otherwise the application will be very slow as OrbTK is a pure software renderer which is very slow in debug mode.

Please note that this is ***only for debug purposes***. Some parts of the application might not be clickable, especially some entries of the top menu and buttons in additional windows. Also sometimes GUI Widgets might just disappear. The last two problems are there due to layouting bugs of OrbTK.

### General troubleshooting
If the taken screenshots don't show the content of the mock applications, there can be a few problems:
1. Take care that the screenshots are taken on the same screen on which the mock application runs. To make this sure, only use one screen.
2. If it still doesn't work, check that you disabled window opening animations on your host machine. Those animations delay opening new windows, so the screenshot might be taken before the mock application is fully loaded.
3. Otherwise you machine might be too slow. If that happens: Don't panic! You just have to increase the time span before the screenshot is taken. You can find it under `src/main.rs`. There is a constant named `TIME_BEFORE_SCREENSHOT`. Enter the time span you like to / you need.

### Troubleshooting on Linux
The generator has been used mainly on Windows. In general, it should work on Linux, but you have to keep a few things in mind:
- If any error messages occur, you might have to install two libraries: `libsdl2-dev` and `libxcb-randr0-dev`

### Build the project
To just build the project and create an executable file within the target folder just execute `cargo build --release`.

## For Developers
The code is structured into different modules. All OrbTK Widgets are separated into the `components` and the `elements` module. `components` contains all more
complex Widgets, that contain other Widgets or have multiple by the application user identifiable parts (such as a complex table). The `elements` module contains the simpler Widgets,
such as buttons or simple descriptions.

The module `generator` contains all logic for the random generator such as the propability distributions, the content for the randomly filled fields of JADX, and utility functions
to make random generation easy.
