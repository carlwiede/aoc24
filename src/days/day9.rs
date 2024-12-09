pub fn part1(input: Vec<String>) -> String
{
    let mut blocks = disk_to_blocks(&input.first().unwrap());
    let compact = compact(&mut blocks);
    let checksum = sum(&compact);
    format!("{}", checksum)
}

pub fn part2(input: Vec<String>) -> String
{
    let mut blocks = disk_to_blocks(&input.first().unwrap());
    let mut spaces = empty_spaces(&blocks);
    let compactish = compact_blockwise(&mut blocks, &mut spaces);
    let checksum = sum_two(&compactish);
    format!("{}", checksum)
}

fn disk_to_blocks(disk_map: &String) -> Vec<i64>
{
    let mut blocks: Vec<i64> = Vec::new();
    let mut global_index = 0;

    for (i, ff) in disk_map.chars().enumerate() {
        if i % 2 == 0 {
            blocks.extend(std::iter::repeat(global_index).take(ff.to_string().parse().unwrap()));
            global_index += 1;
        } else {
            blocks.extend(std::iter::repeat(-1).take(ff.to_string().parse().unwrap()));
        }
    }

    blocks
}

fn compact(blocks: &mut Vec<i64>) -> Vec<i64>
{
    let mut write_index = 0;

    for n in (0..blocks.len()).rev() {
        if write_index > n { break }
        if let Some(val) = blocks.get_mut(n) {
            if *val == -1 { continue }
            let original_value = val.clone();
            *val = -1;
            while *blocks.get(write_index).unwrap() != -1 { write_index += 1 }
            *blocks.get_mut(write_index).unwrap() = original_value;
        }
    }

    blocks.to_vec()
}

fn sum(compact: &Vec<i64>) -> i64
{
    let mut sum = 0;
    for (i, n) in compact.iter().enumerate() {
        if *n == -1 { break }
        sum += i as i64 * n;
    }
    sum
}

fn sum_two(compactish: &Vec<i64>) -> i64
{
    let mut sum = 0;
    for (i, n) in compactish.iter().enumerate() {
        if *n == -1 { continue }
        sum += i as i64 * n;
    }
    sum
}

fn empty_spaces(blocks: &Vec<i64>) -> Vec<(usize, usize)>
{
    let mut spaces: Vec<(usize, usize)> = Vec::new();
    let mut start = None;

    for (i, b) in blocks.iter().enumerate() {
        if *b != -1 && start.is_none() { continue }

        if *b != -1 && !start.is_none() {
            spaces.push((start.unwrap(), i-1));
            start = None;
        }

        if *b == -1 && start.is_none() {
            start = Some(i);
        }
    }

    spaces
}

fn compact_blockwise(blocks: &mut Vec<i64>, spaces: &mut Vec<(usize, usize)>) -> Vec<i64>
{
    let mut block_start = None;

    for n in (0..blocks.len()).rev() {
        if *blocks.get(n).unwrap() != -1 && block_start.is_none() {
            block_start = Some(n);
            continue;
        }

        if !block_start.is_none() && *blocks.get(n).unwrap() != *blocks.get(block_start.unwrap()).unwrap() {

            let block_size = block_start.unwrap() - n;
            let mut viable: i64 = -1;

            for (i, pos) in spaces.iter().enumerate() {

                if pos.0 >= block_start.unwrap() { break };

                let size = pos.1 - pos.0 + 1;
                if size >= block_size {

                    for a in pos.0..pos.0+block_size {
                        *blocks.get_mut(a).unwrap() = *blocks.get(block_start.unwrap()).unwrap();
                    }

                    for b in n+1..=block_start.unwrap() {
                        *blocks.get_mut(b).unwrap() = -1;
                    }

                    viable = i as i64;
                    break;
                }
            }

            if viable != -1 {
                let space: (usize, usize) = *spaces.get(viable as usize).unwrap();
                if space.1 - space.0 + 1 > block_size {
                    *spaces.get_mut(viable as usize).unwrap() = (space.0 + block_size, space.1);
                } else {
                    spaces.remove(viable as usize);
                }
            }

            if *blocks.get(n).unwrap() != -1 { block_start = Some(n) }
            else { block_start = None }
        }
    }

    blocks.to_vec()
}