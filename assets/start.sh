#!/usr/bin/env bash
set -e

if [ ! -z "$NIX_LD_LIBRARY_PATH" ]; then 
	LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH java -Xmx2048m -jar paper.jar -nogui
else
	java -Xmx2048m -jar paper.jar -nogui
fi
