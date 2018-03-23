# Rust Collision Detector
Performs collision detection on convex 2D polygons by means of the separating axis theorem.

This is a Rust port of the collision detection logic used by pyTanks 
[here](https://github.com/JoelEager/pyTanks.Server/blob/master/gameLogic/collisionDetector.py). Both this 
implementation and the Python implementation located there are configured to test the execution time of this algorithm.

This algorithm has also been ported to Kotlin [here](https://github.com/JoelEager/Kotlin-Collision-Detector).

## Setup
Just clone this repo and have cargo take care of the setup and compiling as shown below.

## Usage
```bash
cargo run --release -- [iterations]
```

## Testing results
**10,000 iterations on an Asus laptop with a Intel Core i7-8550U:**

| Mode            | Time for Rust   | Time for Python   |
| --------------- | --------------- | ----------------- |
| With max_dist   | 0.0330 seconds  |  8.250 seconds    |
| Without         | 0.2301 seconds  | 21.800 seconds    |

**10,000 iterations on a custom desktop with an AMD A10-6800K:**

| Mode            | Time for Rust   | Time for Python   |
| --------------- | --------------- | ----------------- |
| With max_dist   | 0.0273 seconds  | 15.844 seconds    |
| Without         | 0.4651 seconds  | 40.617 seconds    |

(See the Kotlin results [here](https://github.com/JoelEager/Kotlin-Collision-Detector#testing-results).)