use itertools::Itertools;

#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
  input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

fn calc_product(vec: &Vec<i32>) -> i32 {
  return vec.iter().product();
}

fn calc_sum(vec: &Vec<i32>) -> i32 {
  return vec.iter().sum();
}

fn find_correct_permutation(data: &Vec<i32>, n: usize) -> Vec<i32> {
  let mut correct_permutation: Vec<i32> = Vec::new();

  data.iter().combinations(n).for_each(|pair| {
    let mapped: Vec<i32> = pair.iter().map(|&&v| v).collect();
    let sum = calc_sum(&mapped);

    if sum == 2020 {
      correct_permutation = mapped;
    }
  });

  return correct_permutation;
}

#[aoc(day1, part1)]
fn solve_part_one(input: &[i32]) -> i32 {
  let data = input.to_vec();
  let result = find_correct_permutation(&data, 2);

  calc_product(&result)
}

#[aoc(day1, part2)]
fn solve_part_two(input: &[i32]) -> i32 {
  let data = input.to_vec();
  let result = find_correct_permutation(&data, 3);

  calc_product(&result)
}

#[cfg(test)]
pub mod tests {
  use super::{calc_product, calc_sum, find_correct_permutation};

  #[test]
  fn validate_find_correct_permutation() {
    assert_ne!(find_correct_permutation(&vec![0, -2], 2), vec!(2, 2020));
    assert_eq!(
      find_correct_permutation(&vec![0, 1, 2019, 10], 2),
      vec!(1, 2019)
    );
  }

  #[test]
  fn validate_product() {
    assert_eq!(calc_product(&vec![-1, 0, 1]), 0);
    assert_eq!(calc_product(&vec![10, 10]), 100);
    assert_eq!(calc_product(&vec![10, 10, 10]), 1000);
  }

  #[test]
  fn validate_sum() {
    assert_eq!(calc_sum(&vec![-1, 0, 1]), 0);
    assert_eq!(calc_sum(&vec![10, 10, 10]), 30);
  }
}
