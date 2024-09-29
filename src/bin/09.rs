advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let mut answer_sum = 0;
    for (_row, input) in input.lines().map(|line| line.split(" ").map(|item| item.parse::<isize>().unwrap()).collect::<Vec<_>>()).enumerate() {
        let mut add_stack: Vec<isize> = Vec::new();
        let mut diff_values = get_diff_vector(&input);
        while diff_values.iter().any(|val| *val != 0) {
            add_stack.push(diff_values.last().unwrap().clone());
            diff_values = get_diff_vector(&diff_values);

        }
        let add_stack_sum: isize = add_stack.iter().sum();
        answer_sum += input.last().unwrap() + add_stack_sum;
        
    }
    Some(answer_sum)
}

fn get_diff_vector(input_vector:&Vec<isize>) -> Vec<isize> {
    let mut diff_vec = Vec::new();

    let mut input_iter = input_vector.iter();
    let mut last_val = input_iter.next().unwrap();
    let mut next_result = input_iter.next();
    while next_result != None {
        let next_val = next_result.unwrap();
        diff_vec.push(next_val - last_val);
        last_val = next_val;
        next_result = input_iter.next();
    }

    diff_vec
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut answer_sum = 0;
    for (_row, input) in input.lines().map(|line| line.split(" ").map(|item| item.parse::<isize>().unwrap()).collect::<Vec<_>>()).enumerate() {
        let mut add_stack: Vec<isize> = Vec::new();
        let mut diff_values = get_diff_vector(&input);
        while diff_values.iter().any(|val| *val != 0) {
            add_stack.push(diff_values.first().unwrap().clone());
            diff_values = get_diff_vector(&diff_values);

        }
        let mut add_stack_sum: isize = add_stack.pop().unwrap();
        let mut next_val = add_stack.pop();
        while next_val != None {
            add_stack_sum = next_val.unwrap() - add_stack_sum;
            next_val = add_stack.pop();
        }

        answer_sum += input.first().unwrap() - add_stack_sum;
        
    }
    Some(answer_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
