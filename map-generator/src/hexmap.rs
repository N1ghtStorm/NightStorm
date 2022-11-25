use rand::Rng;

pub struct HexMap {
    pub map: Vec<Vec<Option<Hex>>>,
    
}

impl HexMap {
    pub fn create_random_map(width: usize, height: usize, number_of_clusters: u8) -> Self {
        let mut map = vec![vec![None; width]; height];
        // let clusters = vec![];
    
        HexMap {
            map
        }
    }
}

#[derive(Clone)]
pub struct Hex {
    pub surface: Surface,
    pub land_shape: LandShape,
}

#[derive(Clone)]
pub enum Surface {
    GrassLand,
    Steppe,
    Snow,
    Tundra,
    Desert,
}

#[derive(Clone)]
pub enum LandShape {
    Mountain,
    Plains,
    Hill,
    Forest,
    Swamp,
    ForestHill,
    SwampedForest,
}

pub struct Cluster {
    pub hex_number: usize,
    pub surface: Surface,
}

impl Cluster {
    pub fn create_random_cluster(hex_number: usize) -> Self {
        todo!();
    }
}