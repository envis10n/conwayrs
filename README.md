# ConwayRS

Rusty conway's game of life.

## Usage

Configuration is handled via command-line arguments.

```txt
Options:
  -w, --width <WIDTH>    Simulation bounds width [default: 128]
  -H, --height <HEIGHT>  Simulation bounds height [default: 128]
  -s, --scale <SCALE>    Simulation scale (window size) [default: 4]
  -r, --rate <RATE>      Simulation rate in hertz [default: 20]
  -S, --seed <SEED>      Seed to use for RNG. Omitting this will use a random seed
  -h, --help             Print help
  -V, --version          Print version
```

The framerate is decoupled from the simulation.

### Controls

TODO: keybindings???

- `Spacebar` Starts and stops the simulation.
- `Backspace` Stops the simulation and then resets the cell states using the random seed.
