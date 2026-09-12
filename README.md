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

### Rulesets

I am working on adding a system for easily plugging in new rulesets using a trait: `CellRule`.

It's quite easy to add new rules by making a new struct and implementing the trait:

```rs
/// Conway's Game of Life Ruleset
pub struct CellRuleConway {}

impl CellRule for CellRuleConway {
    fn apply_rule(neighbors: Vec<(Vec2D, bool)>, state: bool) -> bool {
        let mut alive = 0;
        // Count living neighbors.
        for (_, s) in neighbors {
            if s {
                alive += 1;
            }
        }
        if state {
            if alive < 2 {
                // Alive, less than 2 living neighbors.
                false
            } else if alive > 3 {
                // Alive, more than 3 living neighbors.
                false
            } else {
                state
            }
        } else if alive == 3 {
            // Dead, exactly 3 living neighbors.
            true
        } else {
            state
        }
    }
}
```

Then we call our tick method with our new rule struct as the generic:

```rs
//...
// When we hit zero, run the simulation tick and set the timer back to the rate.
self.sim_timer = self.sim_rate as f64;
// By default we will use the Conway rules.
self.cells.tick::<CellRuleConway>();
self.render_sim();
//...
```

### Controls

TODO: keybindings???

- `Spacebar` Starts and stops the simulation.
- `Backspace` Stops the simulation and then resets the cell states using the random seed.
- `Escape` Closes the application.
