pub fn transpose_blocks(bytes: &Vec<u8>, num_blocks: usize) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::with_capacity(num_blocks as usize);
    for i in 0..num_blocks {
        blocks.push(Vec::with_capacity((bytes.len() / num_blocks) + 1));
    }

    for i in 0..bytes.len() {
        let block_i = i % num_blocks;
        blocks[block_i].push(bytes[i]);
    }

    return blocks;
}

#[test]
fn test_transpose_blocks() {
    let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let transposed = transpose_blocks(&bytes, 2);

    assert_eq!(
        vec![
            vec![1, 3, 5, 7],
            vec![2, 4, 6, 8]
        ],
        transposed
    );
}