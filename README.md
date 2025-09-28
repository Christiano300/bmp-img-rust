# 1-image-generation

For groups 3-4 ppl, those who are already familiar with rust may act as coaches :)


For each of the below, take in a file name (how you do this is your choice),
and if it doesn't exist, create a bmp. If it does exist but isn't a valid BMP,
return an error. If it does exist and is a BMP, overwrite it. Try to solve
as many tasks as possible within the hour.

## Tasks
- Draw a single pixel of any color, anywhere on the image.
- Draw a diagonal line of any color, across one diagonal of a square canvas. Error if the canvas is not square.
- Draw an X shape of any color, across the whole of a canvas. 
- Draw a house shape.
- Draw the outline of a square or rectangle (up to you) on a canvas. The width and coordnates of the square should be specified by the user
- Draw a filled-in square on a canvas. The outline should be a different colour to the filling-
- Draw a 1000x600 pixel flag of Austria (entire canvas)
- Draw a 1000x600 pixel flag of Finland (entire canvas)
- Draw the plot of a sine wave that's 20 pixels high, across the center of an image
- For each of the below, take in two files called f1 and f2 (how you do this is your choice). If either don't exist or aren't valid, throw an error.
    - If f1 and f2 have the same dimensions; write to f1 a pixel-wise average of the two.
    - If f1 and f2 have the same dimensions; print the first row of the first; then the second row of the second, then the third row of the first, fourth row of the second, and so on
    - Copy f1 on top of f2. If f1 does not fit inside f2, print an appropriate error message


## Things that might be useful
code to create a simple bmp image. In bmp images you specify a pixel an 
````rust
let mut image = Image::new(100, 100); // create a bmp image width, height
image.set_pixel(50, 50, Pixel::new(255, 255, 255)); // set a pixel at x,y, with r,g,b (0-255 u8)
image.save(path) // saves image at path (string - full or relative path)
````
we might also need some loops ;)

## sinus wave

$$y = A * sin(f * x + p) + C$$

quick breakdown:
- A: Amplitude. This controls the height of the wave from its center max = height/2
- f: Frequency. This controls how many cycles of the wave occur (higher -> more cycles) if you want N cycles f = (N * 2pi)/width
- p: Phase Shift (positiv shifts left, negativ right), can be used to overlay two waves
- C: Vertical Shift (Offset) (the baseline of the wave, try height/2 as a start.

## weired lines
your lines might look a bit broken, if you want you can create "smoother" lines by using
the Bresenham algorithm [bresenham](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) - or
Wu's algorithm [wu](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm) 
