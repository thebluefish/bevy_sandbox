# bevy_sandbox

My personal sandbox of game features I'm trying to iron out in bevy.

Notable concepts:

- A manually-implemented System
- Running multiple Schedules
    - One-shot systems
    - Delayed "startup" systems
    - State manager
    - Fixed tick schedules (game, phyiscs, simulation, etc..)
- Asset handling
    - Handy macro for working with assets directly
    - Checking if assets have loaded
    - Show a loading screen while we wait