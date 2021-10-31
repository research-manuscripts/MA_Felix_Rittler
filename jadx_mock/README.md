# JADX Screenshot Generator 

The subject of this repository is a GUI mock of the [JADX](https://github.com/skylot/jadx) application.
It has been created to generate valid screenshots of the JADX application in order to train neural networks.

## Prerequisites

The application is written in Rust and includes a Bash script.
So, to execute the JADX Mock, [Rust](https://www.rust-lang.org/tools/install) has to be installed and some kind of Bash interpreter.

## Usage
To execute the generator, you just have to execute

1. Install prerequisites
2. Execute `bash generate_data.sh NUMBER_OF_IMAGES` while NUMBER_OF_IMAGES is the number of images you want to create
3. Inspect the generated images in the directory `target/images`

## For Developers
The code is structured into different modules. All OrbTK Widgets are separated into the `components` and the `elements` module. `components` contains all more 
complex Widgets, that contain other Widgets or have multiple by the application user identifiable parts (such as a complext table). The `elements` module contains the simpler Widgets, 
such as buttons or simple descriptions.

The module `generator` contains all logic for the random generator such as the propability distributions, the content for the randomly filled fields of JADX, and utility functions
to make random generation easy.
