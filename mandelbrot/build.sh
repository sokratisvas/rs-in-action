#!/bin/bash
rustc main.rs -o main && ./main && xdg-open mandelbrot.ppm
