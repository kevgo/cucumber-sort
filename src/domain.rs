pub struct File {
    pub blocks: Vec<Block>,
}

#[derive(Default)]
pub struct Block {
    pub start: usize,
    pub lines: Vec<String>,
}
