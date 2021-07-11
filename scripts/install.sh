#!/usr/bin/env sh

if ! command -v unzip >/dev/null; then
  echo "error: \`unzip\` is required to install Recast. Please install it and try again."
  exit 1
fi

dataDir="$HOME/.recast"

if [ "$OS" = "Windows_NT" ]; then
  target="x86_64-pc-windows-msvc"
else
  case $(uname -sm) in
  "Darwin x86_64")
    target="x86_64-apple-darwin" ;;
  "Darwin arm64")
    target="aarch64-apple-darwin" ;;
  *)
    target="x86_64-unknown-linux-gnu" ;;
  esac
fi

zipUrl="https://github.com/shreyashsaitwal/recast/releases/latest/download/recast-$target.zip"

# Download and unzip recast-$target.zip
curl --location --progress-bar -o "$dataDir/rush-$target.zip" "$zipUrl"
unzip -oq "$dataDir/rush-$target.zip" -d "$dataDir"
rm "$dataDir/rush-$target.zip"

# Give all the necessary scripts execution permission
chmod +x "$dataDir/bin/recast"
chmod +x "$dataDir/tools/jetifier-standalone/bin/jetifier-standalone"

cyan='\033[0;36m'
green='\033[0;32m'
reset='\033[0m'

echo
echo "${green}Success!${reset} Installed Rush at $dataDir/bin/recast"
if ! command -v recast >/dev/null; then
  if [ "$OS" = "Windows_NT" ]; then
    echo
    echo "Now, add the following entry to your 'PATH' environment variable:"
    echo "${cyan}$dataDir/bin${reset}"
  else
    case $SHELL in
      /bin/zsh) shell_profile=".zshrc" ;;
      *) shell_profile=".bash_profile" ;;
    esac

    exp=" export PATH=\"\$PATH:$dataDir/bin\" "
    edge=$(echo " $exp " | sed 's/./-/g')

    echo
    echo "Now, manually add Rush's bin directory to your \$HOME/$shell_profile (or similar):"
    echo "$edge"
    echo "|${cyan}${exp}${reset}|"
    echo "$edge"
  fi
fi
echo
echo "Run ${cyan}rush --help${reset} to get started."
