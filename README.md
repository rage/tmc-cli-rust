# tmc-cli-rust

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://raw.githubusercontent.com/rage/tmc-cli-rust/main/LICENSE)
![Linux](https://github.com/rage/tmc-cli-rust/workflows/Linux/badge.svg)
![Windows](https://github.com/rage/tmc-cli-rust/workflows/Windows/badge.svg)
![macOS](https://github.com/rage/tmc-cli-rust/workflows/macOS/badge.svg)

Command line interface for TMC, written in Rust. 

The old Java CLI can be found at [testmycode/tmc-cli](https://github.com/testmycode/tmc-cli) 

## Project documentation

*These documentations are written in Finnish*

#### Root folder
https://drive.google.com/drive/folders/1SpDOYh5NAp5xwluWRrK-B3j-_ZcEHIr0

#### Product & Sprint backlogs
https://docs.google.com/spreadsheets/d/1KxWFXeK85lhkcf2Z5QLoIwfEJHsCtVBftUomchilN9Q/edit#gid=0 

#### Work time monitoring
https://docs.google.com/spreadsheets/d/1KxWFXeK85lhkcf2Z5QLoIwfEJHsCtVBftUomchilN9Q/edit#gid=1477657539

#### Client meetings
https://drive.google.com/drive/folders/1SpDOYh5NAp5xwluWRrK-B3j-_ZcEHIr0

## Getting started

### Installation

#### Using the install script (Linux/OS X)

```cd``` into the directory where you want to download tmc-cli-rust and run the following command:

64 bit:

        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s x86_64
        
32 bit:
    
        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s i686

#### Using the Windows installer

Download the installer 'tmc-cli-rust.msi' at ... TODO

### Manual installation

If using the installers is not an option for you, you can download the [latest release](https://github.com/rage/tmc-cli-rust/releases/latest) manually. 

Windows: tmc-cli-rust.exe

Linux/OS X: tmc-cli-rust

#### For Linux: 

After downloading 'tmc-cli-rust', navigate to the directory it's located in and make it an executable with the command

        chmod u+x ./tmc-cli-rust

To use the software from any directory, you can add it to your environmental variables with the following command (substituting DIRECTORY for the location where tmc-cli-rust resides at.)

        echo "alias tmc='DIRECTORY/tmc-cli-rust'" >> "$HOME/.bashrc"

#### For Windows: 

After downloading tmc-cli-rust.exe you can start using it from the command line by navigating to the directory it resides at. 

To be able to use it from any directory, you can add it to your environmental variables with the following command. (substituting DIRECTORY for the directory where tmc-cli-rust.exe resides at)

        set PATH=%PATH%;DIRECTORY


### Commands

        tmc-cli-rust [FLAGS] [SUBCOMMAND]

FLAG | Description
:--- | :---
`-h, --help` | Prints help information
`-d, --no-update` | Disable auto update temporarily
`-V, --version` | Prints version information


## Published Builds

Published *Builds* will be located to the [https://download.mooc.fi](https://download.mooc.fi). Builds are published for the some different operating systems (Windows, MacOS, Linux).

## Usage manual

Manual for using the program

## Contribution

### Formatting

Code should be formatted with [rustfmt](https://github.com/rust-lang/rustfmt)

The recommended linter is [rust-clippy](https://github.com/rust-lang/rust-clippy)

## Credits

Software will be developed during spring 2021 as a part of the course *Ohjelmistotuotantoprojekti* in the University of Helsinki.

### Original developers

* Aleksis [Tykky](https://github.com/Tykky)
* Arttu [ShootingStar91](https://github.com/ShootingStar91)
* Jaime
* Joni [Nooblue](https://github.com/Nooblue/)
* Juha [Robustic](https://github.com/Robustic/)
* Miika
* Tatu 

### Disclaimer

This software is licensed under the [Apache 2.0 license](https://raw.githubusercontent.com/rage/tmc-cli-rust/main/LICENSE).

This software comes with no warranty. University of Helsinki and the tmc-cli-rust developers are not responsible for any damages caused by misuse or misbehaviour of this software.
