#!/bin/bash

# Example usage
# ./update.sh v0.3.5

version="$1"

if [ $version == '']; then 
  echo "Invalid version"
  exit 
fi

pkg_name="fi.mooc.tmc.tmc-cli-rust"
gh_user="flathub"

url='https://download.mooc.fi'
file="$url/tmc-cli-rust/tmc-cli-rust-x86_64-unknown-linux-gnu-$version"

sha256_sum=$(curl $file | sha256sum | awk '{ print $1 }')

yaml="app-id: fi.mooc.tmc.tmc-cli-rust
runtime: org.freedesktop.Platform
runtime-version: '19.08'
sdk: org.freedesktop.Sdk
command: tmc
finish-args:
    - --share=network
    - --filesystem=host
modules:
  - name: tmc
    buildsystem: simple
    build-commands:
      - install -D tmc /app/bin/tmc
      - install -Dm644 metainfo.xml /app/share/appdata/fi.mooc.tmc.tmc-cli-rust.appdata.xml
      - install -Dm644 64x64.png /app/share/app-info/icons/flatpak/64x64/fi.mooc.tmc.tmc-cli-rust.png
      - install -Dm644 128x128.png /app/share/app-info/icons/flatpak/128x128/fi.mooc.tmc.tmc-cli-rust.png
    sources:
      - type: file
        path: 64x64.png
      - type: file
        path: 128x128.png
      - type: file
        path: metainfo.xml
      - type: file
        url: ${file}
        sha256: ${sha256_sum}
        dest-filename: tmc"

echo "$yaml" > fi.mooc.tmc.tmc-cli-rust.yml

# Pushes changes to Flathub in branch $version
git clone git@github.com:$gh_user/$pkg_name
if [ -d "./$pkg_name" ]; then
  rm $pkg_name/$pkg_name.yml
  cp $pkg_name.yml $pkg_name
  cd $pkg_name
  git branch $version
  git checkout $version
  git add .
  git commit -m "Update to $version"
  git push --set-upstream origin $version
  cd ..
  sudo rm -r $pkg_name
else
  echo "No permissions to $pkg_name"
fi
