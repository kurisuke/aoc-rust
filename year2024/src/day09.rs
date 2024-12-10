use std::{cmp::Reverse, collections::BinaryHeap};

use common::day::Day;

pub struct Day09 {}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let mut blocks = parse_input_star1(input);
        compact(&mut blocks);
        format!("{}", checksum_star1(&blocks))
    }

    fn star2(&self, input: &str) -> String {
        let (mut file_offsets, mut free_offsets) = parse_input_star2(input);
        defrag(&mut file_offsets, &mut free_offsets);
        format!("{}", checksum_star2(&file_offsets))
    }
}

fn parse_input_star1(input: &str) -> Vec<Option<usize>> {
    let mut blocks = vec![];

    for (i, c) in input.trim().chars().enumerate() {
        let length = c.to_digit(10).unwrap();
        let entry = if i % 2 == 0 { Some(i / 2) } else { None };

        for _ in 0..length {
            blocks.push(entry);
        }
    }

    blocks
}

fn checksum_star1(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, c)| i * c.unwrap_or(0))
        .sum()
}

fn compact(blocks: &mut [Option<usize>]) {
    let mut free_pos = blocks.iter().position(|e| e.is_none()).unwrap();
    let mut file_pos = blocks.len() - 1 - blocks.iter().rev().position(|e| e.is_some()).unwrap();
    assert_eq!(file_pos, blocks.len() - 1);

    loop {
        blocks.swap(free_pos, file_pos);

        // get next file block
        loop {
            file_pos -= 1;
            if blocks[file_pos].is_some() {
                break;
            }
        }

        // get next free block
        loop {
            free_pos += 1;
            if blocks[free_pos].is_none() {
                break;
            }
        }

        if free_pos > file_pos {
            return;
        }
    }
}

struct File {
    offset: usize,
    len: usize,
}

fn parse_input_star2(input: &str) -> (Vec<File>, [BinaryHeap<Reverse<usize>>; 9]) {
    let mut free_offsets = [const { BinaryHeap::new() }; 9];
    let mut file_offsets = vec![];

    let mut offset = 0;

    for (i, c) in input.trim().chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            file_offsets.push(File { offset, len });
        } else if len > 0 {
            free_offsets[len - 1].push(Reverse(offset));
        }
        offset += len;
    }

    (file_offsets, free_offsets)
}

fn defrag(file_offsets: &mut [File], free_offsets: &mut [BinaryHeap<Reverse<usize>>; 9]) {
    for file in file_offsets.iter_mut().rev() {
        let mut chosen = None;

        // find the free space interval which will fit the file, which has the lowest offset
        // search only the buckets which are at least this size
        for l in file.len..=9 {
            if let Some(Reverse(offset)) = free_offsets[l - 1].peek() {
                if *offset < file.offset {
                    if let Some((_, chosen_offset)) = chosen {
                        if offset < chosen_offset {
                            chosen = Some((l, offset));
                        }
                    } else {
                        chosen = Some((l, offset));
                    }
                }
            }
        }

        // we have found a fitting free space
        if let Some((l, new_offset)) = chosen {
            // reset the file offset to the free space
            file.offset = *new_offset;

            // remove the now occupied free space
            free_offsets[l - 1].pop();
            // if there is any remaining free space, add it as a new free space to the correct bucket
            if l > file.len {
                let diff = l - file.len;
                free_offsets[diff - 1].push(Reverse(file.offset + file.len));
            }
        }
    }
}

fn checksum_star2(file_offsets: &[File]) -> usize {
    let mut sum = 0;
    for (id, file) in file_offsets.iter().enumerate() {
        for i in file.offset..(file.offset + file.len) {
            sum += i * id;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn star1() {
        let d = Day09 {};
        assert_eq!(d.star1(INPUT), "1928");
    }

    #[test]
    fn star2() {
        let d = Day09 {};
        assert_eq!(d.star2(INPUT), "2858");
    }
}
