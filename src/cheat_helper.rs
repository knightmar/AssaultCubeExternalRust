use crate::cheat_helper::data::DataSaver;

pub mod helper {
    use std::fmt::{Display, Formatter};

    pub struct Coord {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Display for Coord {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{},{})", self.x, self.y, self.z)
        }
    }

    #[derive(Debug)]
    pub enum Error {
        InvalidInput,
        ParseFloatError(std::num::ParseFloatError),
    }

    impl Coord {
        pub fn parse(input: &String) -> Result<Coord, Error> {
            let vec: Vec<&str> = input.split_whitespace().collect();

            if vec.len() != 3 {
                return Err(Error::InvalidInput);
            }

            let x = vec[0].parse::<f32>().map_err(Error::ParseFloatError)?;
            let y = vec[1].parse::<f32>().map_err(Error::ParseFloatError)?;
            let z = vec[2].parse::<f32>().map_err(Error::ParseFloatError)?;

            Ok(Coord { x, y, z })
        }
    }
}

pub mod data {
    #[derive(Clone)]
    pub struct DataSaver {
        pub(crate) infinite_ammo: bool,
        pub(crate) tp_text: String,
        pub(crate) is_running: bool,
    }
}

impl PartialEq for DataSaver {
    fn eq(&self, other: &Self) -> bool {
        &self.is_running == &other.is_running && &self.infinite_ammo == &other.infinite_ammo && &self.tp_text == &other.tp_text
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}