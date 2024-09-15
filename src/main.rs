extern crate noise;
extern crate image;
extern crate rand;
extern crate log;
extern crate env_logger;

use noise::{NoiseFn, Perlin, Seedable};
use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::error::Error;
use std::fmt;

use log::{debug, error, log_enabled, info, Level};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const SCALE: f64 = 0.00675;

#[derive(Clone, Copy)]
enum Tile {
    Water,
    Land,
    Snow,
    Volcano,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Biome {
    Ocean,
    Coast,
    Beach,
    Grassland,
    Forest,
    Jungle,
    Rainforest,
    Swamp,
    Tundra,
    Taiga,
    Mountain,
    Highland,
    Hill,
    Desert,
    Steppe,
    Ice,
}

impl fmt::Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Biome::Ocean => write!(f, "Ocean"),
            Biome::Coast => write!(f, "Coast"),
            Biome::Beach => write!(f, "Beach"),
            Biome::Grassland => write!(f, "Grassland"),
            Biome::Forest => write!(f, "Forest"),
            Biome::Jungle => write!(f, "Jungle"),
            Biome::Rainforest => write!(f, "Rainforest"),
            Biome::Swamp => write!(f, "Swamp"),
            Biome::Tundra => write!(f, "Tundra"),
            Biome::Taiga => write!(f, "Taiga"),
            Biome::Mountain => write!(f, "Mountain"),
            Biome::Highland => write!(f, "Highland"),
            Biome::Hill => write!(f, "Hill"),
            Biome::Desert => write!(f, "Desert"),
            Biome::Steppe => write!(f, "Steppe"),
            Biome::Ice => write!(f, "Ice"),
        }
    }
}


struct Map {
    tiles: Vec<Vec<Tile>>,
    biomes: Vec<Vec<Biome>>,
    heightmap: Vec<Vec<f64>>,
    precipitation_map: Vec<Vec<f64>>,
}

fn interpolate_color(color1: Rgb<u8>, color2: Rgb<u8>, factor: f64) -> Rgb<u8> {
    let r = (color1[0] as f64 * (1.0 - factor) + color2[0] as f64 * factor) as u8;
    let g = (color1[1] as f64 * (1.0 - factor) + color2[1] as f64 * factor) as u8;
    let b = (color1[2] as f64 * (1.0 - factor) + color2[2] as f64 * factor) as u8;
    Rgb([r, g, b])
}

fn color(tile: Tile, biome: Biome, height: f64, precipitation: f64) -> Rgb<u8> {
    let base_color = match tile {
        Tile::Water => match biome {
            Biome::Ocean => Rgb([42u8, 132u8, 171u8]),
            Biome::Coast => Rgb([47u8, 172u8, 225u8]),
            Biome::Beach => Rgb([92u8, 207u8, 255u8]),
            _ => Rgb([42u8, 132u8, 171u8]),
        },
        Tile::Land => match biome {
            Biome::Grassland => Rgb([34u8, 139u8, 34u8]),
            Biome::Forest => Rgb([102u8, 204u8, 0u8]),
            Biome::Jungle => Rgb([0u8, 153u8, 0u8]),
            Biome::Swamp => Rgb([0u8, 51u8, 0u8]),
            Biome::Tundra => Rgb([0u8, 153u8, 153u8]),
            Biome::Taiga => Rgb([0u8, 173u8, 173u8]),
            Biome::Highland => Rgb([95u8, 193u8, 123u8]),
            Biome::Hill => Rgb([74u8, 150u8, 96u8]),
            Biome::Mountain => Rgb([153u8, 153u8, 0u8]),
            Biome::Desert => Rgb([230u8, 155u8, 24u8]),
            //Biome::Savanna => Rgb([161u8, 144u8, 36u8]),
            Biome::Rainforest => Rgb([0u8, 102u8, 51u8]),
            Biome::Steppe => Rgb([255u8, 255u8, 102u8]),
            Biome::Ice => Rgb([204u8, 229, 255u8]),
            _ => Rgb([34u8, 139u8, 34u8]),
        },
        Tile::Snow => match biome {
            _ => Rgb([255u8, 255u8, 255u8]),
        },
        //Tile::Mountain => match biome {
        //    Biome::Mountain => Rgb([139u8, 69u8, 19u8]),
        //    _ => Rgb([139u8, 69u8, 19u8]),
        //},
        Tile::Volcano => match biome {
            Biome::Mountain => Rgb([139u8, 69u8, 19u8]),
            _ => todo!(),
        },
        //Tile::Desert => match biome {
        //    Biome::Desert => Rgb([230u8, 155u8, 24u8]),
        //    _ => Rgb([230u8, 155u8, 24u8]),
        //},
    };

    // Define the color to interpolate towards based on height
    let height_color = Rgb([255u8, 255u8, 255u8]); // White color for high altitude

    // Interpolate the base color with the height color based on the height
    interpolate_color(base_color, height_color, height/2.0)
}

fn distance_to_equator(latitude: f64) -> f64 {
    latitude.abs() / 90.0 // Normalize to range [0, 1]
}

fn calculate_latitude(y: f64) -> f64 {
    let normalized_y = 2.0 * (y as f64 / HEIGHT as f64) - 1.0;
    let latitude = normalized_y * 90.0;
    latitude
}


fn equivalent_elevation(latitude: f64, height: f64) -> f64 {
    let distance = distance_to_equator(latitude);
    if height > -0.2 {
        0.5 * height + distance
    } else {
        height
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
        let mut heightmap = vec![vec![0.0; WIDTH]; HEIGHT];
        let mut precipitation_map = vec![vec![0.0; WIDTH]; HEIGHT];

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

                heightmap[y][x] = height_value;
                precipitation_map[y][x] = precipitation_value;

                if height_value > -0.2 {
                    tiles[y][x] = Tile::Land;
                } else {
                    tiles[y][x] = Tile::Water;
                }

                let equivalent_elevation = equivalent_elevation(calculate_latitude(y as f64), height_value);

                biomes[y][x] = Self::biome(equivalent_elevation, precipitation_value, calculate_latitude(y as f64));


            }
        }

        Map { tiles , biomes , heightmap , precipitation_map }
    }

    fn biome(elevation: f64, precipitation: f64, latitude : f64) -> Biome {
        let distance_to_equator = distance_to_equator(latitude);

        if distance_to_equator > 0.95 {
            return Biome::Ice;
        }
        else if elevation > 0.85 {
            return Biome::Mountain;
        }
        else if elevation > 0.65 {
            return Biome::Highland;
        }
        else if elevation > 0.45 {
            return Biome::Hill;
        }
        else if elevation > -0.2 {
            if precipitation > 0.75 && (distance_to_equator > 0.1 || distance_to_equator < -0.1) {
                return Biome::Jungle;
            }
            if precipitation > 0.75 {
                return Biome::Rainforest;
            }
            if precipitation > 0.5 {
                return Biome::Forest;
            }
            if precipitation > -0.9 {
                return Biome::Grassland;
            }
            if precipitation > -0.9 &&
                ((distance_to_equator > 0.1 && distance_to_equator < 0.3) ||
                (distance_to_equator < -0.1 && distance_to_equator > -0.3)) {
                return Biome::Desert;
            }
            return Biome::Steppe;
        }
        else {
            //TODO check for neighbouring elevation for sea biome etc
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
                let pixel = color(*tile, self.biomes[y][x], self.heightmap[y][x], self.precipitation_map[y][x]);
                imgbuf.put_pixel(x as u32, y as u32, pixel);
            }
        }

        imgbuf.save(filename).unwrap();
    }
}

fn adjust_biomes(biomes: &mut Vec<Vec<Biome>>, heightmap: &Vec<Vec<f64>>, precipitation_map: &Vec<Vec<f64>>) {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let height = biomes.len();
    let width = biomes[0].len();

    for y in 0..height {
        for x in 0..width {
            let current_biome = biomes[y][x];
            let mut biome_counts = std::collections::HashMap::new();

            for (dy, dx) in &directions {
                let ny = y as isize + dy;
                let nx = x as isize + dx;

                if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                    let neighbor_biome = biomes[ny as usize][nx as usize];
                    *biome_counts.entry(neighbor_biome).or_insert(0) += 1;
                }
            }

            // Adjust the current biome based on the most common neighboring biome
            if let Some((&most_common_biome, _)) = biome_counts.iter().max_by_key(|&(_, count)| count) {
                if most_common_biome != current_biome {
                    // Apply some logic to decide whether to change the current biome
                    // For example, if the most common neighboring biome is different and the current biome is not dominant
                    //info!("Changing biome at ({}, {}) from {:?} to {:?}", x, y, current_biome, most_common_biome);
                    if let Some(&current_biome_count) = biome_counts.get(&current_biome) {
                        if current_biome_count < 6 { // Less than half of the neighbors are the same biome
                            biomes[y][x] = most_common_biome;
                        }
                    } else {
                        // If the current biome is not found in the counts, it means it has no neighbors of the same type
                        biomes[y][x] = most_common_biome;
                    }
                }
            }
        }
    }
}

fn main() {
    env_logger::init();

    let mut map = Map::new();
    adjust_biomes(&mut map.biomes, &map.heightmap, &map.precipitation_map);
    map.save_as_png("map.png");
}

//TODO
// - add climate system (temperature, humidity, wind)
// - add fantasy options (red line from one piece, abrupt changes in climate, etc)
// - add fantasy biomes (amplified)
// - add rivers
// - make biome generation more realistic
//   - deserts only inland and near tropics
//   - jungle near equator
//   - tundra near poles
//   - etc
// - add cities, roads, etc
