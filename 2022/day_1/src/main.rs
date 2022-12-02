fn main() {
    let mut calories = std::fs::read_to_string("day_1/input")
        .expect("Input must exist")
        .split("\n\n")
        .map(|calory_lines| {
            calory_lines
                .split('\n')
                .map(|calory| calory.parse().unwrap_or(0u64))
                .sum::<u64>()
        })
        .collect::<Vec<_>>();

    calories.sort_by(|a, b| b.cmp(a));

    // Part one
    println!("Find the Elf carrying the most Calories: {}", calories[0]);

    // Part two
    println!(
        "Find the top three Elves carrying the most Calories: {}",
        calories.iter().take(3).sum::<u64>()
    );
}
