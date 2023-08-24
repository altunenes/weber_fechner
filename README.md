[![Rust](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml)

# weber_fechner

![Firefly Weber Fechner Ψ-rust programming, red happy cartoon crab 45875](https://github.com/altunenes/weber_fechner/assets/54986652/fce47cdf-dc3b-4d9d-bc89-2eb7ebded0f3)

A psychophysics experiment about Weber–Fechner law based on vision using `bevy` game engine.

## WASM Support

Currently, it is possible to run the experiment on the web. You can try it [here](https://altunenes.github.io/weber_fechner/). But the desktop version is more stable.

Note: Wait for the experiment to load. It may take a while (white screen is normal for a while).


## Future Work
- This template can be used for other psychophysics experiments since it is very generic. Faces, objects, Gabors and other stimuli can be used instead of the ellipses. Bevy is a very powerful game engine and it can be used for other experiments as well. And it can work on any platform that supports Rust including mobile devices. 

After all, isn't all psychology experiments just a game? :)

## How to run

```rust
cargo run --release
```

that's it!

## Logic

- If you see more ellipses on the left, press the "1". If you see more ellipses on the right side, "0" if you perceive them as equal, press "space".

you can modify the number of trials, ellipse radius, or min_max values of distributions before the experiment. Also you can change the drawing method/distribution of ellipses.
![image](https://github.com/altunenes/weber_fechner/assets/54986652/b139cd13-72e6-4ed2-a3aa-ec8a1432433e)





## Contribution
- Feel free to contribute to the project. I am open to any suggestions. Please open an issue if you have any questions or suggestions etc.

