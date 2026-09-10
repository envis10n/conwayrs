# ConwayRS

Rusty conway's game of life.

## Usage

TODO: Use args for parameters.

For now, parameters are handled as consts.

- `FRAME_WIDTH: u32 = 128` The simulation width / pixel buffer width.
- `FRAME_HEIGHT: u32 = 128` The simulation height / pixel buffer height.
- `FRAME_SCALE: u32 = 4` The render scale (final window size).
- `SIM_RATE: u32 = 20` The simulation rate in hertz.
- `RANDOM_SEED: u64 = 1337` The seed used for the RNG instance.

The framerate is decoupled from the simulation.

### Controls

TODO: keybindings???

- `Spacebar` Starts and stops the simulation.
- `Backspace` Stops the simulation and then resets the cell states using the random seed.
