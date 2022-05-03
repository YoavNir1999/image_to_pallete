### A program to apply color schemes on one or multiple images from a color scheme file and for extracting colors from images.
Built in rust.

start by cloning the repo and make sure you have the rust lang build tools.

enter the folder via a terminal.

to convert images to a color scheme put the wanted colors in the "config.txt" file in the "config" folder and the images to convert in the folder "files_for_conversion" and run with "cargo run -r". a conversion percentage can be set under "percentage" in the "config.txt", it is a value between 0.0 and 1.0 the affects how much to preserve of the original pixel with a value of 1.0 as the default meaning a total conversion to the scheme.

to extract colors from images put the images you want to extract from in the folder "files_for_extraction" and run "cargo run -r extract k_value". 
k_value is the number of colors it will try to extract, default is 5 if no k is provided.
outputs will be an image of all the colors extracted and a text file with the hex values of the colors.

to extract colors from an image and convert other images to its scheme put the image you want to extract from in the folder "files_for_extraction" and the images you want to convert in "files_for_conversion" run "cargo run -r match-image k_value".
k_value is the number of colors it will try to extract, default is 5 if no k is provided. make sure to only put one photo in the "files_for_extraction" folder otherwise a random one from the folder will be picked.

## color scheme extraction examples:
original:

![Alt text](/examples/extraction/image1.jpeg?raw=true "source image")

extracted scheme:

![Alt text](/examples/extraction/scheme1.jpeg?raw=true "output scheme")

original:

![Alt text](/examples/extraction/image2.jpeg?raw=true "source image")

extracted scheme:

![Alt text](/examples/extraction/scheme2.jpeg?raw=true "output scheme")

## color scheme conversion examples:
original:

![Alt text](/examples/conversion/image3.jpeg?raw=true "source image")

conversion to nord scheme:

![Alt text](/examples/conversion/conversion3.jpeg?raw=true "output image with nord colors")

original:

![Alt text](/examples/conversion/image4.jpeg?raw=true "source image")

conversion to nord scheme:

![Alt text](/examples/conversion/conversion4.jpeg?raw=true "output image with nord colors")