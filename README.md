# Harita

Harita is a Rust project that generates a procedural map using Perlin noise. The map includes various tiles and biomes, and it can be saved as a PNG image.

## Features

- Procedural generation of a map using Perlin noise
- Different tiles such as Water, Land, Snow, Mountain, Highland, Hill, Volcano, and Desert
- Various biomes including Ocean, Coast, Beach, Grassland, Forest, Jungle, Swamp, Tundra, Taiga, Mountain, Desert, Savanna, and Ice
- Save the generated map as a PNG image

## Dependencies

This project uses the following Rust crates:

- `noise`: For generating Perlin noise
- `image`: For creating and saving PNG images
- `rand`: For generating random seeds

## Installation

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository:

   ```sh
   git clone https://github.com/yourusername/harita.git
   cd harita
   ```

3. Build the project:

   ```sh
   cargo build --release
   ```

## Usage

To generate a map and save it as a PNG image, run the following command:

```sh
cargo run --release
```

The generated map will be saved as `map.png` in the project directory.

## License

This project is licensed under the MIT License.

## Acknowledgements

Big thanks to Amit Patel for his articles on procedural map generation:
- [Redblobgames](https://www.redblobgames.com/maps/terrain-from-noise/)
```
