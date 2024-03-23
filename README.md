# Example failure cases for Bevy/Rapier
This a minimal project to demonstrate how a `KinematicCharacterController` can get stuck on walls. See `main.rs` for details.

## Running
```sh
cargo run --release
```

## Getting stuck on walls
Here's a quick example of how collide-and-slide seems to fail with vertical walls. Note that the
forward key is being held almost the whole time - the stops are the collider getting stuck on the wall.
[![Bevy/Rapier Getting Stuck on Walls](http://img.youtube.com/vi/lySzeeX68aI/0.jpg)](http://www.youtube.com/watch?v=lySzeeX68aI "Bevy/Rapier Getting Stuck on Walls")

## Offset collapse Example
In trying to resolve the above issue, I ran across a potentially related issue: offset values
(at least large ones) aren't respected. Below is an example with a large offset:
```rust
KinematicCharacterController {
    offset: CharacterLength::Relative(1.00),
    .. default()
}
```
but as can be seen in the video, this offset is only maintained when initially approaching the wall.
After pushing against it for a second, the collider moves in much closer than this offset.
[![Bevy/Rapier Getting Stuck with large Offsets](http://img.youtube.com/vi/q46BHkLa_s4/0.jpg)](http://www.youtube.com/watch?v=q46BHkLa_s4 "Bevy/Rapier Getting Stuck with large Offsets")