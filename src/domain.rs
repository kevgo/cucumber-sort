pub struct File {
    pub initial_lines: Vec<String>,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Block {
    pub start: usize,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    /// the full text of the step
    pub text: String,
    /// just the relevant title of the step
    pub title: String,
}
