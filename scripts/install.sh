#!/bin/bash
set -euo pipefail

echo "~ Installing TMC-CLI ~"
echo "(If your shell is not bash, you may have to do the installation manually.)"
echo ""

if (( $# < 2 )); then
    echo "You need to give architecture (x86_64/i686) and OS (mac, linux) as arguments."
    exit 1
fi

# Get platform-string from first argument, OS from the second
platform=$1
os=$2

echo "Fetching latest version URL from https://download.mooc.fi"
if ! PAGE=$(curl -s https://download.mooc.fi); then
	echo "Failed to reach download.mooc.fi" >&2
	exit
fi

# Adding spaces so ${PAGE[@]} will work.
PAGE=$(echo $PAGE | sed -r 's:</Contents><Contents>:</Contents> <Contents>:g')

testexp="-test" # TODO: remove after first official release

fileprefx=""
if [[ "$os" == "mac" ]]; then
    fileprefx="tmc-cli-rust-${platform}-apple-darwin-v"
else
    fileprefx="tmc-cli-rust-${platform}-unknown-linux-gnu-v"
fi


prefx="<Key>tmc-cli-rust/$fileprefx"
suffx="${testexp}</Key>"

regx="${prefx}[0-9]+\.[0-9]+\.[0-9]+${suffx}"

# Finding the latest version of the executable
version="0.0.0"
for entry in ${PAGE[@]}; do
    if [[ ${entry} =~ $regx ]]; then        
        noprefix="${BASH_REMATCH[0]#$prefx}" #remove prefix
        newversion="${noprefix%$suffx}" #remove suffix
        
        IFS=. verold=(${version##*-})
        IFS=. vernew=(${newversion##*-})

        if ((${vernew[0]} > ${verold[0]} )); then
            version=$newversion
        elif ((${vernew[0]} >= ${verold[0]} )) && ((${vernew[1]} > ${verold[1]} )) ; then
            version=$newversion
        elif ((${vernew[0]} >= ${verold[0]} )) && ((${vernew[1]} >= ${verold[1]} )) && ((${vernew[2]} > ${verold[2]} )) ; then
            version=$newversion
        fi
    fi
done

if [[ $version == "0.0.0" ]]; then 
    echo "Could not find version";
    exit 1
fi
echo "Latest version: $version" 

filename="${fileprefx}${version}${testexp}"
URL="https://download.mooc.fi/tmc-cli-rust/$filename"

echo ""
echo "Downloading TMC-CLI from the following address"
echo "$URL"
echo

curl -L "$URL" > "$PWD/$filename"

if [ ! -f "$PWD/$filename" ]; then
	echo "Error downloading TMC-CLI"
	exit 1
fi

# Gives execution privileges for the file
chmod u+x "$PWD/$filename"

# Removes old aliases
sed -i '/alias tmc=/d' "$HOME/.bashrc"

# Saves new alias to .bashrc
echo "alias tmc='$PWD/$filename'" >> "$HOME/.bashrc"

echo ""

echo "Installation complete. Please restart the terminal."
echo "After opening a new terminal, you can try using TMC-CLI from the command line with:"
echo "  'tmc login'"
exit
