use std::io;

fn solution(x: u32, y: u32) -> u32 {
    #[inline]
    fn sum_to_n(n: u32) -> u32 {
        n * (n + 1) / 2
    }

    #[inline]
    fn count_to_distance(count: u32) -> u32 {
        if count % 2 == 0 {
            sum_to_n(count / 2) * 2
        } else {
            sum_to_n(count / 2) * 2 + ((count + 1) / 2)
        }
    }

    fn recursive_binary_search(current: u32, next: u32, target_distance: u32) -> u32 {
        let current_distance = count_to_distance(current);
        let previous_distance = count_to_distance(current - 1);

        if previous_distance < target_distance && target_distance <= current_distance {
            return current;
        }

        if current_distance < target_distance {
            recursive_binary_search(current + next, next / 2, target_distance)
        } else {
            recursive_binary_search(current - next, next / 2, target_distance)
        }
    }

    let distance = y - x;

    recursive_binary_search(65536, 32768, distance)
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<u32>);
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        println!("{}", solution(x, y));
    }
}
