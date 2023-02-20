#!/bin/bash

set -euo pipefail

echo "~ Installing TMC-CLI ~"
echo "(If your shell is not bash, you may have to do the installation manually.)"
echo ""

echo "Fetching latest version URL from https://download.mooc.fi"
if ! PAGE=$(curl -s https://download.mooc.fi); then
  echo "Failed to reach download.mooc.fi" >&2
  exit
fi

# get platform and os, e.g. "x86_64" and "Linux"
if (( $# == 2 )); then
  # get from args if any
  platform=$1
  os=$2
else
  # else get from uname
  platform="$(uname -m)"
  os="$(uname -s)"
fi

if [[ "$os" == "Darwin" ]] || [[ "$os" == "mac" ]]; then
  file_prefix="tmc-cli-rust-${platform}-apple-darwin-v"
else
  file_prefix="tmc-cli-rust-${platform}-unknown-linux-gnu-v"
fi

regex="^tmc-cli-rust/${file_prefix}([0-9]+\.[0-9]+\.[0-9]+)$"

# Finding the latest version of the executable
version="0.0.0"
# splits using < and >, hacky but gets the job done...
IFS="<>"
read -r -a entries <<< "$PAGE"
for entry in "${entries[@]}"; do
  if [[ ${entry} =~ $regex ]]; then
    new_version="${BASH_REMATCH[1]}"

    IFS=.
    read -r -a old <<< "$version"
    read -r -a new <<< "$new_version"

    if (( "${new[0]}" > "${old[0]}" )); then
      version=$new_version
    elif (( "${new[0]}" >= "${old[0]}" )) && (( "${new[1]}" > "${old[1]}" )) ; then
      version=$new_version
    elif (( "${new[0]}" >= "${old[0]}" )) && (( "${new[1]}" >= "${old[1]}" )) && (( "${new[2]}" > "${old[2]}" )) ; then
      version=$new_version
    fi
  fi
done

if [[ $version == "0.0.0" ]]; then 
  echo "Could not find version";
  exit 1
fi
echo "Latest version: $version" 

filename="${file_prefix}${version}"
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

SHELLNAME=$(basename "$SHELL")
if [ "$SHELLNAME" = "bash" ]; then
  PROFILEFILE=$HOME/.bashrc
  echo "You are currently using bash as your shell, so defaulting to .bashrc for environment variables."
elif [ "$SHELLNAME" = "zsh" ]; then
  PROFILEFILE=$HOME/.zshrc
  echo "You are currently using zsh as your shell, so defaulting to .zshrc for environment variables."
elif [ "$SHELLNAME" = "csh" ] || [ "$SHELLNAME" = "tcsh" ]; then
  PROFILEFILE=$HOME/pg-shellvariables
  echo "This script does not automatically add variables with csh syntax to your shell configuration."
  echo "Please add manually variables from $PROFILEFILE to your .cshrc using csh syntax."
else
  PROFILEFILE=$HOME/.shrc
  echo "Defaulting to .shrc for environment variables. If this is incorrect, please copy these manually to correct file."
fi
# Removes old aliases and such
sed -i '/alias tmc=/d' "$PROFILEFILE"
sed -i "/export TMC_LANGS_CONFIG_DIR=/d" "$PROFILEFILE"

echo "$PROFILEFILE"

COMPLETIONS_PATH=$HOME/.local/share/tmc-autocomplete


CMD=$PWD/$filename

# Saves new alias to .bashrc
echo "alias tmc='$CMD'" >> "$PROFILEFILE"
echo "export TMC_LANGS_CONFIG_DIR='$HOME/tmc-config'" >> "$PROFILEFILE"

echo ""



#
#
# Auto-complete scripts
# 
#
if [ "$SHELLNAME" = "bash" ]; then
 

  echo "Generating auto-complete scripts to $COMPLETIONS_PATH"
  echo ""
  echo "" 

  # removing possibly existing sourcing
  sed -i '/source/!b;/tmc-autocomplete/d' "$PROFILEFILE"


  # creating the completions directory, if it doesn't exist
  eval "mkdir -p $COMPLETIONS_PATH"

  # calling the generate-completions subcommand to generate the completion script
  eval "$CMD generate-completions --bash > $COMPLETIONS_PATH/tmc.bash"

  # adding the line to .bashrc so that bash knows where to look for
  echo "source $COMPLETIONS_PATH/tmc.bash" >> "$PROFILEFILE"

elif [ "$SHELLNAME" = "zsh" ]; then
  echo "Generating auto-complete scripts to $COMPLETIONS_PATH"
  echo ""
  echo ""

  # removing possibly existing definitions
  sed -i "/compdef _tmc/d" "$PROFILEFILE"
  sed -i '/fpath/!b;/tmc-autocomplete/d' "$PROFILEFILE"

  eval "mkdir -p $COMPLETIONS_PATH"
  eval "$CMD generate-completions --bash > $COMPLETIONS_PATH/_tmc"

  echo "fpath=($COMPLETIONS_PATH/_tmc " '$fpath)' >> "$PROFILEFILE"

  echo "compdef _tmc tmc" >> "$PROFILEFILE"
fi

echo "Installation complete. Please restart the terminal."
echo "After opening a new terminal, you can try using TMC-CLI from the command line with:"
echo "  'tmc login'"
exit
