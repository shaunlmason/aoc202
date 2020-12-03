use itertools::Itertools;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
  input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

fn calc_product(v1: &i32, v2: &i32) -> i32 {
  return v1 * v2;
}

fn find_correct_permutation(values: &[i32]) -> (i32, i32) {
  let mut matches: Vec<(i32, i32)> = vec![];
  let combinations = values.iter().combinations(2);
  combinations.for_each(|p| {
    let (v1, v2) = (p[0], p[1]);
    let sum = v1 + v2;

    if sum == 2020 {
      matches.push((*v1, *v2));
    }
  });

  if matches.len() == 0 {
    return (-1, -1);
  }

  let (v1, v2) = matches[0];
  return (v1, v2);
}

#[aoc(day1, part1)]
fn solve_part_one(input: &[i32]) -> i32 {
  let (v1, v2) = find_correct_permutation(input);

  calc_product(&v1, &v2)
}

// #[aoc(day1, part2)]
// fn solve_part_two(input: &[i32]) -> i32 {
//   // input.iter().map(calc_total_mass).sum()
//   11
// }

#[cfg(test)]
pub mod tests {
  use super::{calc_product, find_correct_permutation};

  #[test]
  fn day1_find_correct_permutation() {
    assert_eq!(find_correct_permutation(&[0, 1, 2019, 10]), (1, 2019));
    assert_ne!(find_correct_permutation(&[0, -2]), (2, 2020));
  }

  #[test]
  fn day1_sum() {
    assert_eq!(calc_product(&10, &10), 100);
    assert_eq!(calc_product(&1, &0), 0);
    assert_eq!(calc_product(&2020, &2020), 4080400);
  }
}
