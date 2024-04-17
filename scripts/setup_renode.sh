#!/bin/bash

# # Parameters
#
# * `-y` - skip confirmation prompt (e.g., for CI)

# Detect renode location
RENODE="${RENODE:-$(which renode)}"
if ! [ -f "$RENODE" ]; then
  echo "renode not found. Add Renode install directory to path."
  exit 1
fi

RENODE_DIR="$(dirname $(which renode))"
RENODE_PYTHON_PERIPHERALS="$RENODE_DIR/scripts/pydev"
REQUIRED_SYMLINK="$RENODE_PYTHON_PERIPHERALS/DLA.py"

BASEDIR=$(dirname "$0")
VP_PYTHON_PERIPHERALS="$BASEDIR/../vp/devel/python_peripherals"

# Check if symlink exists
if [ ! -h "$REQUIRED_SYMLINK" ]; then
    echo "Symlink not found"
    if [[ ! $* == *-y* ]]; then
        read -p "Create symlinks and continue? (y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
    fi
    # Symlink the updated Python peripheral to Renode search directory
    ln -s $(readlink -f "$VP_PYTHON_PERIPHERALS/DLA.py") "$REQUIRED_SYMLINK"
    echo "Created Symlink at \"$REQUIRED_SYMLINK\""
else
    echo "Symlink at \"$REQUIRED_SYMLINK\" detected. Nothing to be done."
fi
