# tmc-cli-rust

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://raw.githubusercontent.com/rage/tmc-cli-rust/main/LICENSE)
![Linux](https://github.com/rage/tmc-cli-rust/workflows/Linux/badge.svg)
![Windows](https://github.com/rage/tmc-cli-rust/workflows/Windows/badge.svg)
![macOS](https://github.com/rage/tmc-cli-rust/workflows/macOS/badge.svg)

Command line interface for TMC, written in Rust. 

The old Java CLI can be found at [testmycode/tmc-cli](https://github.com/testmycode/tmc-cli) 

## Table of Contents

1. [Installation](#installation)
2. [Commands](#commands)
3. [Usage manual](#usage-manual)
4. [Contribution](#contribution)
5. [Project documentation](#project-documentation)
6. [Credits](#credits)

## Installation

### Published Builds

Published *Builds* will be located to the [https://download.mooc.fi](https://download.mooc.fi). Builds are published for the some different operating systems (Windows, MacOS, Linux).

### Using Flatpak

        flatpak install flathub fi.mooc.tmc.tmc-cli-rust

Create alias after installation for ease of use with:

        echo "alias tmc=\"flatpak run fi.mooc.tmc.tmc-cli-rust\"" >> ~/.bashrc

After restarting the terminal, this should work:

        tmc --help


### Using the install script (Linux/OS X)

```cd``` into the directory where you want to download tmc-cli-rust and run the following command:

64 bit Linux:

        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s x86_64 linux
        
32 bit Linux:
    
        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s i686 linux

64 bit MacOS:

        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s x86_64 mac
        
32 bit MacOS:
    
        curl -0 https://raw.githubusercontent.com/rage/tmc-cli-rust/dev/scripts/install.sh | bash -s i686 mac

### Using the Windows installer

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

## Commands

        tmc [FLAGS] [SUBCOMMAND]

### Flags

FLAG | Description
:--- | :---
`-h, --help` | Prints help information
`-d, --no-update` | Disable auto update temporarily
`-V, --version` | Prints version information

### Subcommands

SUBCOMMAND | Description
:--- | :---
`courses` | List the available courses
`download` | Downloads course exercises
`exercises` | List the exercises for a specific course
`help` | Prints this message or the help of the given subcommand(s)
`login` | Login to TMC server
`logout` | Logout from TMC server
`organization` | Change organization
`paste` | Submit exercise to TMC pastebin
`submit` | Submit exercises to TMC server
`test` | Run local exercise tests
`update` | Update exercises

## Usage manual

Manual for using the program.

### Logging in

You can log in using `tmc login`. This saves your TMC login information to a configuration file in /home/username/tmc-config/tmc-tmc_cli_rust (or %APPDATA%\tmc-tmc_cli_rust on Windows) - you will only have to log in once.

```
~ $ tmc login
Email / username: username
Password: 
Logged in successfully!
```

After you have logged in, you can choose your organization with interactive menu. To see all organizations, select *View all organizations* with keyboard arrows. Press keyboard characters to filter.

```
Select your organization:            Press keys
>> MOOC                              to filter
   Helsingin Yliopisto
   View all organizations

```

After you have selected your organization, you can choose course with interactive menu. Exercises of the course will be downloaded. Press keyboard characters to filter. If you don't want to download anything, select *Don't download anything* with keyboard arrows.

```
Select your course:                  Press keys
>> Don't download anything           to filter
   2013 Object-oriented programming,
   2013 Object-oriented programming,
   Aikatauluton Ohjelmoinnin MOOC, Oh
   Aikatauluton Ohjelmoinnin MOOC, Oh
   Cyber Security Base Advanced Topic
   Java Programming I                
   Java Programming II
   Ohjelmoinnin MOOC 2021
   Securing Software 2020
   Securing Software 2021
```

When filtering, only courses with filtered name are shown.

```
Select your course:                  ohjelmoinn
>> Aikatauluton Ohjelmoinnin MOOC, Oh
   Aikatauluton Ohjelmoinnin MOOC, Oh
   Ohjelmoinnin MOOC 2021
```

After course is selected, exercises are downloaded. Download folder is informed for the user.

```
Successfully downloaded 15 out of 15 exercises.
 100%[█████████████████████████] [00:00:00]
Exercises downloaded successfully to /home/user/.local/share/tmc/tmc_cli_rust\
```

### Organization

You can change your organization with the command `tmc organization`. To see all organizations, select *View all organizations* with keyboard arrows. All available organizations will be listed. You can choose your organization with interactive menu.

```
~ $ tmc organization
```

### Logging out

You can log out using 'tmc logout'. This will remove your login token from the configuration file.

```
~ $ tmc logout
Logged out successfully.
```

### Listing courses

Once you have logged in, you can list all the available courses on the server with `tmc courses`.
```
~ $ tmc courses
```

### Downloading course exercises

*Either*

When you have already selected your organization, simply run `tmc download` and select right course to download with interactive menu.

*Or*

Navigate to a suitable directory in which you wish to download your exercises. Then, run `tmc download -d` to download to the current directory after course is selected with interactive menu. 

*Or*

Give suitable course name as an argument `tmc download -c [COURSE_NAME]`.

*Or*

Give suitable course name as an argument and use `-d` flag to download to the current directory: `tmc download -c [COURSE_NAME] -d`.

```
~ $ tmc download
Fetching courses...
Successfully downloaded 37 out of 37 exercises.
 100%[█████████████████████████] [00:00:01]
```

### Running tests

*Either*

After you've completed an exercise and wish to run tests on it, just write command `tmc test`. You can choose course and exercise with interactive menu.

```
~ $ tmc test

Testing: exercise1

Test results: 1/1 tests passed
All tests passed! Submit to server with 'tmc submit'
 100%[████████████████████████████████████████████████████████████████]
```

*Or*

Navigate to the exercise directory and run `tmc test`.

```
~/tmc-courses/test-course/exercise1 $ tmc test

Testing: exercise1

Test results: 1/1 tests passed
All tests passed! Submit to server with 'tmc submit'
 100%[████████████████████████████████████████████████████████████████]

```

### Listing exercises

If you want to see your current progress, you can view the status of all course exercises with `tmc exercises [course]`.

```
~/tmc-courses/test-course/ $ tmc exercises test-course
Course name: test-course
Deadline: none
Soft deadline: none
  Completed: exercise1
  Completed: exercise2
  Not completed: exercise3
```

### Paste

*Either*

When you want to send your current solution for an exercise to someone else for review, just write command `tmc paste`. You can choose course and exercise with interactive menu. Give your paste message when program asks *Write a paste message, enter sends it*.

```
~ $ tmc paste
Write a paste message, enter sends it:
example paste message

Paste finished, running at https://examplewebpage
 100%[█████████████████████████] [00:00:00]
```

*Or*

Navigate to the exercise directory and run `tmc paste`. Give your paste message when program asks *Write a paste message, enter sends it*.

```
~/tmc-courses/test-course/exercise1 $ tmc paste
Write a paste message, enter sends it:
example paste message

Paste finished, running at https://examplewebpage
 100%[█████████████████████████] [00:00:00]
```

### Submit

*Either*

You can send your solution to the server with `tmc submit`. You can choose course and exercise with interactive menu.

```
~ $ tmc submit
You can view your submission at: https://examplewebpage
Submission finished processing!
 100%[█████████████████████████] [00:00:02]
All tests passed on server!
Points permanently awarded: [1.excercise1]
Model solution: https://examplewebpage

```

*Or*

Navigate to the exercise directory and run `tmc submit`.

```
~/tmc-courses/test-course/exercise1 $ tmc submit
You can view your submission at: https://examplewebpage
Submission finished processing!
 100%[█████████████████████████] [00:00:02]
All tests passed on server!
Points permanently awarded: [1.excercise1]
Model solution: https://examplewebpage
```

### Update

If some updates have done to the exercises by your organization, you can download the latest exercises with `tmc update`. You can choose course with interactive menu.

## Contribution

Notes for further development:

We've used [cargo-wix](https://github.com/volks73/cargo-wix) to create the Windows Installer.
Further instructions for it's usage can be found on the given repository.
Please mind that the *License.rtf* has been modified (for the copyright part) and changed the manufacturer has been to University of Helsinki in the *main.wxs* file. We have not implemented creating the installer in the GitHub Actions, so it needs to be done manually.

Github actions builds every commit and runs all tests on them on every supported platform. When tests
are successful there is a option to create a release build. These are triggered by creating a
tag on the commit you want to create release from. Usually commits are tagged with version like this:
```
git tag v0.0.1
```
Only hard requirement is that all tags which are used to create release builds **must begin with letter v**.
Release builds are uploaded to [download.mooc.fi](https://download.mooc.fi/). Each release can be downloaded
by using url:
```
https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-<ARCH>-<PLATFORM>-<VERSION>.<EXT>
```

where each <> should be replaced with one of these:

 - ARCH: x86_64, i686
 - PLATFORM: pc-windows-msvc, unknown-linux-gnu, apple-darwin
 - VERSION: This one is given by the tag (e.g v0.0.1)
 - EXT: On windows: exe. On other platforms this is empty.

When in doubt, you can always check all downloadable files at [download.mooc.fi](https://download.mooc.fi/)
by examining the xml file manually.

For example x86_64 downloads for v.0.3.5 look like this:
```
https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-x86_64-unknown-linux-gnu-v0.3.5
https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-x86_64-apple-darwin-v0.3.5
https://download.mooc.fi/tmc-cli-rust/tmc-cli-rust-x86_64-pc-windows-msvc-v0.3.5.exe
```

When it comes to creating releases, our typical workflow looks like this:

First make sure that all tests pass then:
```
git checkout main
git merge dev
git tag v0.0.1
git push --tags
```

### Formatting

Code should be formatted with [rustfmt](https://github.com/rust-lang/rustfmt)

The recommended linter is [rust-clippy](https://github.com/rust-lang/rust-clippy)

## Project documentation

*These documentations are written in Finnish*

### Root folder
https://drive.google.com/drive/folders/1SpDOYh5NAp5xwluWRrK-B3j-_ZcEHIr0

### Product & Sprint backlogs
https://docs.google.com/spreadsheets/d/1KxWFXeK85lhkcf2Z5QLoIwfEJHsCtVBftUomchilN9Q/edit#gid=0 

### Work time monitoring
https://docs.google.com/spreadsheets/d/1KxWFXeK85lhkcf2Z5QLoIwfEJHsCtVBftUomchilN9Q/edit#gid=1477657539

### Client meetings
https://drive.google.com/drive/folders/1SpDOYh5NAp5xwluWRrK-B3j-_ZcEHIr0

## Credits

Software was developed during spring 2021 as a part of the course *Ohjelmistotuotantoprojekti* in the University of Helsinki.

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
