#!/bin/bash
set -x # echo on

BASEDIR=$(dirname "$0")
BIN=${BIN=$1}
RENODE_PATH=$(dirname $(which renode))
RENODE_PYTHON_PERIPHERALS="$RENODE_PATH/scripts/pydev"
VP_PYTHON_PERIPHERALS="$BASEDIR/../../vp/devel/python_peripherals"

# Check if symlinks exist
if [ ! -h "$RENODE_PYTHON_PERIPHERALS/DLA.py" ]; then
   echo "Symlinks not found"
   read -p "Create symlinks and continue? (y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1
   # Symlink the updated Python peripheral to Renode search directory
   ln -s $(readlink -f "$VP_PYTHON_PERIPHERALS/DLA.py") "$RENODE_PYTHON_PERIPHERALS/DLA.py"
fi

renode --console -e "set bin @$BIN; include @$BASEDIR/../../scripts/2_run_hpc.resc"
