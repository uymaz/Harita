extern crate noise;
extern crate image;
extern crate rand;

use noise::{NoiseFn, Perlin, Seedable};
use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::error::Error;

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const SCALE: f64 = 0.00675;

#[derive(Clone, Copy)]
enum Tile {
    Water,
    Land,
    Snow,
    Mountain,
    Highland,
    Hill,
    Volcano,
    Desert,
}

#[derive(Clone, Copy)]
enum Biome {
    Ocean,
    Coast,
    Beach,
    Grassland,
    Forest,
    Jungle,
    Swamp,
    Tundra,
    Taiga,
    Mountain,
    Desert,
    Savanna,
    Ice,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    biomes: Vec<Vec<Biome>>,
}

fn color(tile: Tile, biome: Biome) -> Rgb<u8> {
    match tile {
        Tile::Water => match biome {
            Biome::Ocean => Rgb([42u8, 132u8, 171u8]),
            Biome::Coast => Rgb([47u8, 172u8, 225u8]),
            Biome::Beach => Rgb([92u8, 207u8, 255u8]),
            _ => todo!(),
        },
        Tile::Land => match biome {
            Biome::Grassland => Rgb([34u8, 139u8, 34u8]),
            Biome::Forest => Rgb([34u8, 139u8, 34u8]),
            Biome::Jungle => Rgb([34u8, 139u8, 34u8]),
            Biome::Swamp => Rgb([0u8, 102u8, 51u8]),
            Biome::Tundra => Rgb([0u8, 153u8, 153u8]),
            Biome::Taiga => Rgb([0u8, 173u8, 173u8]),
            Biome::Mountain => Rgb([34u8, 139u8, 34u8]),
            Biome::Desert => Rgb([230u8, 155u8, 24u8]),
            Biome::Savanna => Rgb([161u8, 144u8, 36u8]),
            Biome::Ice => Rgb([204u8, 255u8, 255u8]),
            _ => todo!(),
        },
        Tile::Snow => match biome {
            Biome::Ice => Rgb([255u8, 255u8, 255u8]),
            _ => todo!(),
        },
        Tile::Mountain => match biome {
            Biome::Mountain => Rgb([139u8, 69u8, 19u8]),
            _ => todo!(),
        },
        Tile::Highland => match biome {
            Biome::Mountain => Rgb([95u8, 193u8, 123u8]),
            _ => todo!(),
        },
        Tile::Hill => match biome {
            Biome::Mountain => Rgb([74u8, 150u8, 96u8]),
            _ => todo!(),
        },
        Tile::Volcano => match biome {
            Biome::Mountain => Rgb([139u8, 69u8, 19u8]),
            _ => todo!(),
        },
        Tile::Desert => match biome {
            Biome::Desert => Rgb([230u8, 155u8, 24u8]),
            _ => todo!(),
        },
    }
}

impl Map {
    fn new() -> Self {
        let seed_heightmap: u32 = rand::thread_rng().gen();
        let seed_precipitation_map: u32 = rand::thread_rng().gen();

        let perlin_heightmap = Perlin::new().set_seed(seed_heightmap);
        let perlin_precipitation_map = Perlin::new().set_seed(seed_precipitation_map);

        let mut tiles = vec![vec![Tile::Water; WIDTH]; HEIGHT];
        let mut biomes = vec![vec![Biome::Ocean; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let nx = x as f64 * SCALE;
                let ny = y as f64 * SCALE;

                //add noise at different freqs to get more realistic terrain
                let mut height_value = 0.0;
                for i in 0..25 {
                    height_value += perlin_heightmap.get([nx * 2.0_f64.powi(i), ny * 2.0_f64.powi(i)]) / 2.0_f64.powi(i);
                }

                let mut precipitation_value = 0.0;
                for i in 0..25 {
                    precipitation_value += perlin_precipitation_map.get([nx * 2.0_f64.powi(i), ny * 2.0_f64.powi(i)]) / 2.0_f64.powi(i);
                }

                if height_value > 0.995 {
                    tiles[y][x] = Tile::Snow;
                }
                else if height_value > 0.85 {
                    tiles[y][x] = Tile::Mountain;
                }
                else if height_value > 0.65 {
                    tiles[y][x] = Tile::Highland;
                } else if height_value > 0.45 {
                    tiles[y][x] = Tile::Hill;
                } else if height_value > -0.2 {
                    tiles[y][x] = Tile::Land;
                } else {
                    tiles[y][x] = Tile::Water;
                }

                biomes[y][x] = Self::biome(height_value, precipitation_value);
            }
        }

        Map { tiles , biomes }
    }

    fn biome(elevation: f64, precipitation: f64) -> Biome {
        if elevation > 0.995 {
            return Biome::Ice;
        }
        else if elevation > 0.85 {
            return Biome::Mountain;
        }
        else if elevation > 0.65 {
            return Biome::Mountain;
        }
        else if elevation > 0.45 {
            return Biome::Mountain;
        }
        else if elevation > -0.2 {
            if precipitation > 0.75 {
                return Biome::Swamp;
            }
            else if precipitation > 0.5 {
                return Biome::Forest;
            }
            else if precipitation > -0.25 {
                return Biome::Grassland;
            }
            else {
                return Biome::Desert;
            }
        }
        else {
            return Biome::Ocean;
        }
    }

    #[deprecated]
    fn display(&self) {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                print!("{}", match tile {
                    Tile::Water => "~",
                    Tile::Land => "#",
                    _ => todo!(),
                });
            }
            println!();
        }
    }

    fn save_as_png(&self, filename: &str) {
        let mut imgbuf = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                //let pixel = match tile {
                //    Tile::Water => Rgb([0u8, 0u8, 255u8]), // Blue for water
                //    Tile::Land => Rgb([34u8, 139u8, 34u8]), // Green for land
                //    Tile::Highland => Rgb([95u8, 193u8, 123u8]), // Light green for highland
                //    Tile::Hill => Rgb([74u8, 150u8, 96u8]), // Dark green for hill
                //    Tile::Snow => Rgb([255u8, 255u8, 255u8]), // White for snow
                //    Tile::Mountain => Rgb([139u8, 69u8, 19u8]), // Brown for mountain
                //    _ => todo!(),
                //};
                let pixel = color(*tile, self.biomes[y][x]);
                imgbuf.put_pixel(x as u32, y as u32, pixel);
            }
        }

        imgbuf.save(filename).unwrap();
    }
}

fn main() {
    //TODO: Add command line arguments for height, width etc
    let map = Map::new();
    //map.display();
    map.save_as_png("map.png");
}
