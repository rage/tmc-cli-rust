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

## Formatting

Code should be formatted with [rustfmt](https://github.com/rust-lang/rustfmt)

The recommended linter is [rust-clippy](https://github.com/rust-lang/rust-clippy)
