use bitaekcoin::block::Block;

pub struct DB {
    pub blocks: Vec<Block>,
}

impl DB {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn latest_block(&self) -> Option<Block> {
        self.blocks.last().cloned()
    }
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}
