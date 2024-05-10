#!/bin/bash

# Create the target directory if it doesn't exist
mkdir -p Freezes

# Loop over each directory in the current working directory
for dir in */
do
  # Ensure it's a directory
  if [ -d "${dir}" ]; then
    # Define the source path
    source_path="${dir}Samples/Processed/Freeze/"
    
    # Check if the source path exists
    if [ -d "${source_path}" ]; then
      # Find and copy all .wav files smaller than 100MB from the source directory to the Freezes/
      find "${source_path}" -name '*.wav' -size -250M -exec cp {} Freezes/ \;
    fi
  fi
done

echo "Copy complete."
