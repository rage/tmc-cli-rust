#!/bin/bash

set -euo pipefail

echo "~ Installing TMC-CLI ~"
echo "(If your shell is not bash, you may have to do the installation manually.)"
echo ""

os="$(uname -s)"
platform="$(uname -m)"

if (( $# == 2 )); then
  # Get platform-string from first argument, OS from the second
  platform=$1
  os=$2
fi

echo "Fetching latest version URL from https://download.mooc.fi"
if ! PAGE=$(curl -s https://download.mooc.fi); then
  echo "Failed to reach download.mooc.fi" >&2
  exit
fi

# Adding spaces so ${PAGE[@]} will work.
PAGE=$(echo $PAGE | sed -r 's:</Contents><Contents>:</Contents> <Contents>:g')

fileprefx=""
if [[ "$os" == "Darwin" ]] || [[ "$os" == "mac" ]]; then
  fileprefx="tmc-cli-rust-${platform}-apple-darwin-v"
else
  fileprefx="tmc-cli-rust-${platform}-unknown-linux-gnu-v"
fi


prefx="<Key>tmc-cli-rust/$fileprefx"
suffx="</Key>"


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

filename="${fileprefx}${version}"
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

echo $PROFILEFILE

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
