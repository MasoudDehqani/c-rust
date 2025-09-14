/*
  This challenge is an example of sliding window problem

  - The sliding window technique is a technique that can be devised to solve
  some problems like this.
*/

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|x| x == second_list);

    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|x| x == first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

fn _sublist_v2(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.is_empty(), second_list.is_empty()) {
        (true, true) => return Comparison::Equal,
        (true, false) => return Comparison::Sublist,
        (false, true) => return Comparison::Superlist,
        (false, false) if first_list.cmp(second_list) == Ordering::Equal => {
            return Comparison::Equal;
        }
        (false, false) => {
            let (larger_list, is_first_list_larger) = if first_list.len() > second_list.len() {
                (first_list, true)
            } else {
                (second_list, false)
            };

            let smaller_list = if is_first_list_larger {
                second_list
            } else {
                first_list
            };

            if larger_list.windows(smaller_list.len()).any(|slice| {
                slice
                    .iter()
                    .enumerate()
                    .all(|(i, el)| *el == smaller_list[i])
            }) {
                return if is_first_list_larger {
                    Comparison::Superlist
                } else {
                    Comparison::Sublist
                };
            }
        }
    }

    Comparison::Unequal
}

fn _sublist_v1(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.cmp(second_list) == Ordering::Equal {
        return Comparison::Equal;
    }

    if first_list.len() > second_list.len() {
        for (i, _) in first_list.iter().enumerate() {
            let (_, slice) = first_list.split_at(i);

            let is_superlist = slice
                .iter()
                .zip(second_list)
                .fold(true, |acc, (f, s)| if f != s { false } else { acc });

            if is_superlist {
                return Comparison::Superlist;
            }
        }
    }

    if second_list.len() > first_list.len() {
        for (i, _) in second_list.iter().enumerate() {
            let (_, slice) = second_list.split_at(i);

            let is_sublist = slice
                .iter()
                .zip(first_list)
                .fold(true, |acc, (f, s)| if f != s { false } else { acc });

            if is_sublist {
                return Comparison::Sublist;
            }
        }
    }

    Comparison::Unequal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn false_start() {
        let list_one: &[i32] = &[1, 2, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_start() {
        let list_one: &[i32] = &[0, 1, 2];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_in_middle() {
        let list_one: &[i32] = &[2, 3, 4];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_at_end() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_start_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn in_middle_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn at_end_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_element_from_second_list() {
        let list_one: &[i32] = &[1, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn second_list_missing_element_from_first_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_additional_digits_from_second_list() {
        let list_one: &[i32] = &[1, 2];
        let list_two: &[i32] = &[1, 22];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn order_matters_to_a_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[3, 2, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn same_digits_but_different_numbers() {
        let list_one: &[i32] = &[1, 0, 1];
        let list_two: &[i32] = &[10, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}
