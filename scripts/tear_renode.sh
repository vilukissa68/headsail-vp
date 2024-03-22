#!/bin/bash

# Detect renode location
RENODE="${RENODE:-$(which renode)}"
RENODE_DIR="$(dirname $(which renode))"

RENODE_PYTHON_PERIPHERALS="$RENODE_DIR/scripts/pydev"
REQUIRED_SYMLINK="$RENODE_PYTHON_PERIPHERALS/DLA.py"

# Check if symlink exists
if [ -h "$REQUIRED_SYMLINK" ]; then
    unlink $REQUIRED_SYMLINK
    echo "Unlinked symlink at \"$REQUIRED_SYMLINK\""
else
    echo "No symlink \"$REQUIRED_SYMLINK\" at Nothing to be done."
fi
