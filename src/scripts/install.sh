#ji!/bin/bash

set -euo pipefail

echo "~ Installing TMC-CLI ~"
echo "(If your shell is not bash, you may have to do the installation manually.)"
echo ""

echo "tmc-cli-rust will be downloaded to '$PWD', overwriting existing files in the process."

echo "Are you sure you want to continue?"
	select yn in "Yes" "No"; do
		case $yn in
			Yes ) break;;
			No ) echo "Download interrupted"; exit;;
		esac
done

echo "Fetching latest release URL"
if ! PAGE=$(curl -s https://api.github.com/repos/rage/tmc-cli-rust/releases/latest); then
	echo "Failed to fetch latest release from github api." >&2
	exit
fi
URL=$(echo "$PAGE" | grep '"browser_download_url"' | grep '/tmc-cli-rust"' | head -n 1 | cut -d '"' -f 4)

echo "Downloading TMC-CLI from following address"
echo "$URL"
echo

curl -LO "$URL" > ./tmc-cli-rust || true

if [ ! -f ./tmc-cli-rust ]; then
	echo "Error downloading TMC-CLI"
	exit 1
fi

chmod u+x ./tmc-cli-rust

echo "alias tmc-cli-rust='$PWD/tmc-cli-rust'" >> "$HOME/.bashrc"
echo "Installation complete. Please restart the terminal."
