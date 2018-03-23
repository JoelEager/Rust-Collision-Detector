# Rust Collision Detector
Performs collision detection on convex 2D polygons by means of the separating axis theorem.

This is a Rust port of the collision detection logic used by pyTanks 
[here](https://github.com/JoelEager/pyTanks.Server/blob/master/gameLogic/collisionDetector.py). Both this 
implementation and the Python implementation located there are configured to test the execution time of this algorithm.

## Setup
Just clone this repo, install the cargo dependencies, and compile it through cargo.

## Usage
./rust-collision-detector [iterations]

## Testing results
**1,000 iterations on an Asus laptop with a Intel Core i7-8550U:**

| Mode            | Time for Rust   | Time for Python   |
| --------------- | --------------- | ----------------- |
| With max_dist   | 0.0567 seconds  | 0.8252 seconds	  |
| Without         | 0.5075 seconds  | 2.2710 seconds    |
