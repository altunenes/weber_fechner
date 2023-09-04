[![Rust](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/weber_fechner/actions/workflows/rust.yml)

# weber_fechner

![Firefly Weber Fechner Ψ-rust programming, red happy cartoon crab 45875](https://github.com/altunenes/weber_fechner/assets/54986652/fce47cdf-dc3b-4d9d-bc89-2eb7ebded0f3)

A psychophysics experiment about Weber–Fechner law based on vision using `bevy` game engine.


### Weber-Fechner Law in Psychophysics
The Weber-Fechner law is a foundational principle in the field of psychophysics that describes the relationship between a physical stimulus and its perceived intensity. According to this law, the change in stimulus required to produce a noticeable difference is a constant proportion of the original stimulus. In simple terms, if you need to add 10 lbs to a 100 lbs weight to notice a difference, you'd need to add 20 lbs to a 200 lbs weight for the same effect.

Understanding the Weber-Fechner law is crucial for experiments that aim to measure human perception, as it helps to quantify how changes in physical stimuli translate to changes in human experience. This is vital in various applications ranging from medical diagnosis to the design of user interfaces.


### 🔵 Weber's Law

Weber's Law states that the smallest noticeable change in a stimulus (`ΔI`) is proportional to the initial intensity of the stimulus (`I`).

#### Formula
\[
`ΔI = k × I`
\]
**Where:**
- `ΔI` is the smallest noticeable change in the stimulus.
- `I` is the initial intensity of the stimulus.
- `k` is the Weber constant.

---

### 🔴 Fechner's Law

Fechner's Law takes Weber's Law a step further. It relates the perceived intensity (`S`) to the physical intensity (`I`) of the stimulus using a logarithmic scale.

#### Formula
\[
`S = k × log(I / I₀)`
\]
**Where:**
- `S` is the perceived intensity.
- `I` is the physical intensity of the stimulus.
- `I₀` is the threshold intensity for perception.
- `k` is a constant.

## WASM Support

Currently, it is possible to run the experiment on the web. You can try it [here](https://altunenes.github.io/weber_fechner/). But the desktop version is more stable.

Note: In WASM version, the loading time may take a while, so if you see a blank/white screen, please wait for a while.


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

