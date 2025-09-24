// pub fn file(text: impl BufRead) -> Result<domain::File> {
//     let mut blocks = Vec::<Block>::new();
//     let mut current_block = Block::default();
//     let mut step_indent = usize::MIN;
//     for (number, line) in text.lines().into_iter().enumerate() {
//         let line = line.unwrap();
//         let indent = line_indent(&line);
//         if is_step(&line) {
//             if current_block.steps.is_empty() {
//                 current_block.start = number;
//             }
//             current_block.steps.push(line);
//         } else {
//             if !current_block.steps.is_empty() {
//                 blocks.push(current_block);
//                 current_block = Block::default();
//             }
//         }
//     }
//     if !current_block.steps.is_empty() {
//         blocks.push(current_block);
//     }
//     Ok(domain::File { blocks })
// }
