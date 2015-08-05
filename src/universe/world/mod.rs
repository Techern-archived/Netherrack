//! A world, where everything ingame happens.

/// A structure defining a world
///
/// Will be changed shortly
pub struct World {

    /// The name of this world
    pub name: &'static str,
    
    /// The base seed used by this world to set the seed for chunk generators
    pub seed: u64,
    
    /// The border for this world
    /// 
    /// If set to 0, no world border is available
    pub border: u64/*,
    
    pub chunkcolumns: Vec<ChunkColumn>,
    
    pub difficulty: Difficulty*/
    
}

impl World {

    /// Checks to see if this world has a border
    pub fn has_border(&self) -> bool {
        self.border != 0
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn new_world_has_no_border() {
        assert!((World { name: "Hello", seed: 21337420u64, border: 0 }).has_border() == false);
    }

    #[test]
    pub fn new_world_has_border() {
        assert!((World { name: "Hello", seed: 21337420u64, border: 100000 }).has_border() == true);
    }

}