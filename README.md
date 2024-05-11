# Stem Extractor for Ableton Live

## Recover old stems, generate a catalogue of your own samples, and 10x your productivity with Ableton Live.

## Stem Extractor for Ableont Live (henceforth called SEAL) is a set of third-party utilities for Live that allow you to recover old project stems amd extract audio from old projects. 

### If you have a backcatalogue of hundreds or thousands of unfinished projects this is for you. I needed a way to get something useful out of the thousands of hours put into old beats -- which in practical terms meaans audio.

### Don't leave your life's work in the form of unfrozen chains of niche unsupported audio plug-ins and at the whis of companies that will make you repurchase the same software every four years that hasn't been maintained since 2003.

### If I could give one piece of advice to a younger version of me it would be this: FLATTEN YOUR SHIT. Followed by, PUT ALL YOUR SHIT IN ONE PLACE, LABEL THAT SHIT, and BACK UP THAT SHIT.

### That could be difficult so instead I decided to learn a bit about computers, and then after two years of studying systems engineering and category theory until I had finally written it. The prompt that would get chatGPT to write the three lines of bash that solved the original problmem that I had. Now to expand upon it.

I bulit this to make it so that the thousands of hours put into the creation of those songs could be tur


This utility automates the extraction of frozen audio from Ableton Live projects by copying the files to a designated "Freezes" folder within the master Ableton projects directory.

## Features

-  **Current Functionality**: Automatically copies frozen audio files from your Ableton project into a master "Freezes" folder where the script is run from.

## Roadmap

-  **File Organization**: Implement copying of cropped audio samples from `<project>/Samples/Processed/Crop` to a separate "Crops" directory.
-  **File Size Options**: Add functionality to specify the maximum file size for copies to manage space efficiently.
-  **Metadata Tagging**: Introduce tagging of files to include origin project data.
-  **Duplicate Management**: Enhance the utility to check for and handle duplicate files intelligently.
-  **Overwrite Options**: Evaluate and implement options concerning file overwrites and optimization strategies.
-  **CLI Tool**: Develop a command-line interface (CLI) version for easier use.
-  **Rust Implementation**: Consider porting the project to Rust to improve performance and maintainability.
-  **Cargo Integration**: Explore transforming the utility into a Cargo package for easier distribution and installation.

## Contributions

Contributions towards developing new features or resolving issues in the roadmap are welcome. For detailed guidelines on how to contribute, please refer to the CONTRIBUTING.md file.

## License

Refer to LICENSE.md to understand the usage rights and restrictions for this utility.

---

*Note: This utility is in active development, and some planned features might not be available yet.*
