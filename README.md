[![Rust](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml)

# weber_fechner

A psychophysics experiment about Weber–Fechner law based on vision using `bevy` game engine.

Currently, it is just a template for the experiment, and it is not ready for the serious use.

## WASM Support

Currently, it is possible to run the experiment on the web. You can try it [here](https://altunenes.github.io/weber_fechner/). But it is kust a template and it is not ready for the serious use. 

## Roadmap

- [x] Basic experiment template using `bevy`.
- [x] Show the stimulus and change the frames based on the keyboard input.
- [x] Track the keyboard inputs, Response time, and the correctness of the response.
- [x] Collect the data and save it to a file (Response times, accuracy, and the stimulus parameters etc).
- [x] Add web support via wasm 
- [x] Save the data as csv in web version
- [x] Add instrunction and information about the experiment.
- [x] Add a menu to change the parameters of the experiment, give instructions, and start the experiment etc.
- [ ] Reorganize the code, split it into modules, and make it more readable (this will be last step after the experiment is ready for the serious use)

## Future Work
- This template can be used for other psychophysics experiments since it is very generic. Faces, objects, gabors and other stimuli can be used instead of the ellipses. Bevy is a very powerful game engine and it can be used for other experiments as well. And it can be work on the any platform that supports Rust including mobile devices. 

After all isn't all psychology experiments are just a game? :)

## How to run
As I mentioned before, this is just a template and it is not ready for the serious use. But if you want to try it, you can run it with the following command:

```rust
cargo run --release
```
that's it!

## How to use

- If you see more ellipses on the left, press the "1". If you see more ellipses on the right side, "0" if you perceive them as equal, press "space".
If you want to conduct a serious experiment and you are in a hurry don't hesitate to contact me. I will be happy to help you. :)
