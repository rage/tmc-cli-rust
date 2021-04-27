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

#### Using Flatpak

        flatpak install flathub fi.mooc.tmc.tmc-cli-rust

Create alias after installation for ease of use with:

        echo "alias tmc=\"flatpak run fi.mooc.tmc.tmc-cli-rust\"" >> ~/.bashrc

After restarting the terminal, this should work:

        tmc --help


#### Using the install script (Linux/OS X)

```cd``` into the directory where you want to download tmc-cli-rust and run the following command:

64 bit Linux:

        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s x86_64 linux
        
32 bit Linux:
    
        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s i686 linux

64 bit MacOS:

        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s x86_64 mac
        
32 bit MacOS:
    
        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s i686 mac

#### Using the Windows installer

Windows installer can be found [here](https://github.com/rage/tmc-cli-rust/tree/dev/installer).
Download and run the newest installer found. After installation, the application updates automatically.

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

## Logging in

You can log in using `tmc-cli-rust login`. This saves your TMC login information to a configuration file in <linux config path> (or %APPDATA%\tmc-vscode_plugin on Windows) - you will only have to log in once.

```
~ $ tmc-cli-rust login
Email / username:
Password:
All available organizations will be listed.
Choose organization by writing its slug:
```

You can change your organization with the command `organization`. All available organizations will be listed.

## Logging out

you can log out using 'tmc-cli-rust logout'. This will remove your login token from the configuration file.

```
~ $ tmc-cli-rust logout
Logged out successfully.
```

## Listing courses

Once you have logged in, you can list all the available courses on the server with `tmc-cli-rust courses`.
```
~ $ tmc-cli-rust courses
```

## Downloading course exercises

Either
Navigate to a suitable directory in which you wish to download your exercises. Then, run `tmc-cli-rust download [COURSE_NAME] .`. 
Or
Enter suitable filepath as an argument `tmc-cli-rust download [COURSE_NAME] [FILEPATH]`
This will download all available exercises into it.


```
~ $ tmc-cli-rust download test-course tmc-courses/test-course
[12 / 15] exercises downloaded.
```

## Running tests

After you've completed an exercise and wish to run tests on it, navigate to the exercise directory and run `tmc-cli-rust test`. If you are in the course root directory, you can also give the name of the exercise as argument: `tmc-cli-rust test exercise1`.

```
~/tmc-courses/test-course/exercise1 $ tmc-cli-rust test
Testing: exercise1

Test results: 1/1 tests passed
All tests passed! Submit to server with 'tmc submit'
100%[████████████████████████████████████████████████████████████████]
```

## Listing exercises

If you want to see your current progress, you can view the status of all course exercises with `tmc-cli-rust exercises [course]`.

```
~/tmc-courses/test-course/ $ tmc-cli-rust exercises test-course
Course name: test-course
Deadline: none
Soft deadline: none
  Completed: exercise1
  Completed: exercise2
  Not completed: exercise3
```





## Contribution
Notes for further development:

We've used [cargo-wix](https://github.com/volks73/cargo-wix) to create the Windows Installer.
Further instructions for it's usage can be found on the given repository.
Please mind that the License.rtf has been modified (for the copyright part) and changed the manufacturer has been to University of Helsinki in the main.wxs file. We have not implemented creating the installer in the GitHub Actions, so it needs to be done manually.

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
