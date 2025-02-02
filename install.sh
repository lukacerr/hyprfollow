#!/bin/bash

# URL of the binary
URL="https://github.com/lukacerr/hyprfollow/releases/download/v1.0.1/hyprfollow"

# Function to download the binary to the specified destination directory
download() {
    DESTINATION="$1"
    echo "Installing to: $DESTINATION"

    if [ ! -d "$DESTINATION" ]; then
        echo "Directory doesn't exist bro :("; exit 1;
    fi

    # Full path for the binary
    BINARY="$DESTINATION/hyprfollow"

    # Download the file
    echo "Downloading from $URL ..."
    if command -v curl >/dev/null 2>&1; then
        curl -L "$URL" -o "$BINARY" || { echo "Error downloading :("; exit 1; }
    elif command -v wget >/dev/null 2>&1; then
        wget "$URL" -O "$BINARY" || { echo "Error downloading :("; exit 1; }
    else
        echo "Either curl or wget are required to download :("
        exit 1
    fi

    # Set execution permissions
    chmod +x "$BINARY" || { echo "Error setting execute permissions."; exit 1; }

    echo "Installation completed @ $BINARY ! Thank you :3"
}

# Function to display the menu
show_menu() {
  printf "\n"
  echo "HyprFollow: JSON based event monitors for your Hyprland activity."
  printf "\n"
  
  echo "Select an installation destination:"
  echo "  1. /usr/local/bin (requires root privileges)"
  echo "  2. \$HOME/bin (local installation)"
  echo "  3. \$HOME/.config/eww/scripts (then can be accessed on deflisten with 'scripts/hyprfollow')"
  echo "  4. \$HOME/Downloads (then you decide what to do c:)"
  echo "  9. Uninstall (works as long as you didn't move the binary, I'm not a magician)"
  echo "  0. idk what I'm doing here (exit without installing)"
  printf "\n"
}

while true; do
  show_menu
  read -rp "(1/2/3/4/9/0) -> " option </dev/tty

  case $option in
    1)
      if [ "$EUID" -ne 0 ]; then
        echo "This option requires root privileges. Please run the script with sudo or as root >:("
        exit 1
      fi
      download "/usr/local/bin"
      break
      ;;
    2)
      download "$HOME/bin"
      break
      ;;
    3)
      download "$HOME/.config/eww/scripts"
      break
      ;;
    4)
      download "$HOME/Downloads"
      break
      ;;
    9)
      if [ -e "/usr/local/bin/hyprfollow" ]; then
          sudo rm "/usr/local/bin/hyprfollow"
      elif [ -e "$HOME/bin/hyprfollow" ]; then
          rm "$HOME/bin/hyprfollow"
      elif [ -e "$HOME/.config/eww/scripts/hyprfollow" ]; then
          rm "$HOME/.config/eww/scripts/hyprfollow"
      
      else
        echo "Couldn't find actual installation."
        exit 1
      fi
      
      echo "Task failed successfully :("
      echo "I'm so sorry I didn't satisfy you"
      echo "Feedback appreciated: https://github.com/lukacerr/hyprfollow"
      
      exit 0
      ;;
    0)
      echo "Exiting :) see you soon (hopefully)"
      exit 0
      ;;
    *)
      echo "Didn't get you :( may you say it again?"
      ;;
  esac
done
