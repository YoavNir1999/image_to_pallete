A program to apply color schemes on one or multiple images from a color scheme file or extracted from another image.

Built in rust.

start by cloning the repo and make sure you have the rust lang build tools.

enter the folder via a terminal.

to convert images to a color scheme put the wanted colors in the "config.txt" file in the "config" folder and the images to convert in the folder "files_for_conversion" and run with "cargo run -r"

to extract colors from images put the images you want to extract from in the folder "files_for_extraction" and run "cargo run -r extract k_value". 
k_value is the number of colors it will try to extract, default is 5 if no k is provided.
outputs will be an image of all the colors extracted and a text file with the hex values of the colors.

to extract colors from an image and convert other images to its scheme put the image you want to extract from in the folder "files_for_extraction" and the images you want to convert in "files_for_conversion" run "cargo run -r match-image k_value".
k_value is the number of colors it will try to extract, default is 5 if no k is provided.