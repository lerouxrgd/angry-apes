# Angry Apes

<p align="center">
<img width="315" height="250" src="assets/cover.png">
</p>

This game was made in the context of the [first Bevy Game Jam][bevy-jam-1] on
[Itch][itch-io], you can play it [here][live-game]. The theme was "unfair advantage",
so you'll find out that collecting enough ETH helps you quite a lot !

[bevy-jam-1]: https://itch.io/jam/bevy-jam-1
[itch-io]: https://lerouxrgd.itch.io/angry-apes
[live-game]: https://lerouxrgd.github.io/angry-apes/

## Play

Here are the controls of the game:

|           | Keyboard | Gamepad       |
|-----------|----------|---------------|
| Attack    | Enter    | West          |
| Dash      | RShift   | East          |
| Jump      | Space    | South         |
| Movements | WASD     | Stick or DPad |

Try to survive and kill as many Apes as you can !

## Run

**Native**

```sh
cargo run --release
```

## Build

**Window**

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

**Wasm**

```sh
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --out-name angry-apes --out-dir wasm --target web target/wasm32-unknown-unknown/release/angry-apes.wasm

python -m http.server -d wasm
```

## Assets

The following assets were used:

* Bored Apes ([here][apes])
* Paladin/Crusader sprite from HoMM2 ([here][homm2])
* Ethereum sprite ([here][eth])
* Lasers sprite ([here][lasers])
* Monkey dead icon ([here][monkey-dead])
* Monkey angry icon ([here][monkey-ok])
* A blockchain image background ([here][background])
* The gameover toilets ([here][toilets])

[apes]: https://boredapeyachtclub.com/#/gallery
[homm2]: https://www.spriters-resource.com/pc_computer/heroesofmightandmagic2
[eth]: https://steemit.com/slothicorn/@wanaf/ethereum-in-3d-pixel-art-gifs
[lasers]: https://opengameart.org/content/laser-effect-sheet
[monkey-dead]: https://www.iconspng.com/image/71030/monkey-emoji-dead-apea
[monkey-ok]: https://www.iconspng.com/image/70806/monkey-emoji-dissatisfied
[background]: https://suedholstein.sparkasseblog.de/files/uploads/sharedContent/9927/1559574685PK-Blockchain-iStock-913017224.jpg
[toilets]: https://ik.imagekit.io/bayc/assets/toilet.png

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Angry Apes by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
