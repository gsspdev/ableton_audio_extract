import os
import shutil

def main():
    # Define the target directory for .wav files
    dir_freezes = 'Freezes'
    dir_crops = 'Crops'
    proj_subdir_freeze = 'Samples/Processed/Freeze'
    proj_subdir_crop= 'Samples/Processed/Crop'

    # Create the target directory if it doesn't exist
    os.makedirs(dir_freezes, exist_ok=True)
    os.makedirs(dir_crops, exist_ok=True)

    # Collect all .wav files in the specified subdirectory using a list comprehension
    wav_files = [
        os.path.join(root, file)
        for root, _, files in os.walk('.')
        if proj_subdir_freeze in root
        for file in files
        if file.endswith('.wav')
    ]

    # Copy files or print a message if no files were found
    if wav_files:
        for file_path in wav_files:
            shutil.copy(file_path, dir_freezes)
    else:
        print("No .wav files found to copy.")

if __name__ == '__main__':
    main()