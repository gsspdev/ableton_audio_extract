#!/bin/bash

# copies all ableton project frozen audio to Freezes directory
mkdir -p Freezes

# Loop through the found .wav files and copy each to the Freezes directory
for i in */Samples/Processed/Freeze/*.wav; do
    # Check if file exists and is a regular file before attempting to copy
    if [ -f "$i" ]; then
        cp "$i" Freezes/
    else
        echo "No .wav files found to copy."
    fi
done
