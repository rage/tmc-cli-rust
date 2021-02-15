#ji!/bin/bash
set -euo pipefail

echo "~ Installing TMC-CLI ~"
echo "(If your shell is not bash, you may have to do the installation manually.)"
echo ""

echo "Fetching latest version URL from https://download.mooc.fi"
if ! PAGE=$(curl -s https://download.mooc.fi); then
	echo "Failed to reach download.mooc.fi" >&2
	exit
fi

platform=""

echo "Which platform are you on?"
	select yn in "x86_64" "i686"; do
		case $yn in
			x86_64 ) platform="x86_64"; break;;
			i686 )   platform="i686"; break;;
		esac
done


testexp="-test"
regx="<Key>tmc-cli-rust/tmc-cli-rust-${platform}-unknown-linux-gnu-v[0-9]+\.[0-9]+\.[0-9]+${testexp}</Key>"

if [[ $PAGE =~ $regx ]]; then
  echo "Found the file from mooc server"
else
  exit
fi

bashmatch="${BASH_REMATCH[0]#<Key>tmc-cli-rust/}"
filename="${bashmatch%</Key>}"

echo ""
echo "Downloading the following file to '$PWD', overwriting existing files in the process: $filename "

echo "Are you sure you want to continue?"
	select yn in "Yes" "No"; do
		case $yn in
			Yes ) break;;
			No ) echo "Download interrupted"; exit;;
		esac
done

curl -LO "https://download.mooc.fi/tmc-cli-rust/$filename" > ./$filename || true

if [ ! -f ./$filename ]; then
	echo "Error downloading TMC-CLI"
	exit 1
fi

chmod u+x ./$filename

# TODO here: Remove old alias tmc= from $HOME/.bashrc

echo "alias tmc='$PWD/$filename'" >> "$HOME/.bashrc"
echo "Installation complete. Please restart the terminal."
