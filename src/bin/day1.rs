fn required_fuel(module_mass: u64) -> u64 {
    (module_mass / 3).saturating_sub(2)
}

fn total_required_fuel(module_mass: u64) -> u64 {
    let mut prev_fuel = module_mass;
    let mut total_fuel = 0u64;
    while prev_fuel != 0 {
        prev_fuel = required_fuel(prev_fuel);
        total_fuel += prev_fuel;
    }

    total_fuel
}

fn main() {
    let input = aoc2019::get_input(1).expect("Failed getting input");
    let module_masses: Vec<_> = input.lines().map(|n| n.parse::<u64>().unwrap()).collect();

    let modules_fuel = module_masses
        .iter()
        .cloned()
        .map(required_fuel)
        .sum::<u64>();
    println!("Module fuel requirements: {}", modules_fuel);

    let total_fuel = module_masses
        .iter()
        .cloned()
        .map(total_required_fuel)
        .sum::<u64>();
    println!("Total fuel requirements: {}", total_fuel);
}
