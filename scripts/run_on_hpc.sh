#!/bin/sh

# Runs compiled ELF from $BIN or param $1 on HPC

BASEDIR=$(dirname "$0")
$BASEDIR/setup_renode.sh

BIN=${BIN=$1}
if [ -z "$BIN" ]; then
    echo "!! Pass in an ELF using \$BIN or \$1"
    exit 1
fi

renode --console -e "set bin @$BIN; include @$BASEDIR/resc/2_run_hpc.resc"
