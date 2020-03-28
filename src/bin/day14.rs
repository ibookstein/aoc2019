use aoc2019::aoc_input::get_input;
use num_integer::Integer;
use std::cmp::min;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseError {
    BadComponentCount,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}

#[derive(Debug, Clone)]
struct ReactionElement {
    chemical: String,
    quantity: usize,
}

impl FromStr for ReactionElement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(' ').collect();
        match &parts[..] {
            [quantity, chemical] => Ok(ReactionElement {
                chemical: chemical.to_string(),
                quantity: quantity.parse()?,
            }),
            _ => Err(ParseError::BadComponentCount),
        }
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    output: ReactionElement,
    inputs: Vec<ReactionElement>,
}

impl FromStr for Reaction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(" => ").collect();
        match &parts[..] {
            [inputs, output] => {
                let output: ReactionElement = output.parse()?;
                let inputs: Result<Vec<ReactionElement>, _> =
                    inputs.split(", ").map(|s| s.parse()).collect();
                Ok(Reaction {
                    output: output,
                    inputs: inputs?,
                })
            }
            _ => Err(ParseError::BadComponentCount),
        }
    }
}

#[derive(Debug, Clone)]
struct ReactionsMap(HashMap<String, Reaction>);

impl FromStr for ReactionsMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reactions: Result<Vec<Reaction>, _> = s.trim().lines().map(|s| s.parse()).collect();
        Ok(ReactionsMap(
            reactions?
                .drain(..)
                .map(|r| (r.output.chemical.clone(), r))
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Inventory(HashMap<String, usize>);

impl Inventory {
    fn with_chemical(chemical: &str, quantity: usize) -> Self {
        let mut map = HashMap::new();
        map.insert(chemical.to_owned(), quantity);
        Inventory(map)
    }

    fn current_amount(&self, chemical: &str) -> usize {
        self.0.get(chemical).cloned().unwrap_or(0)
    }
}

static ORE: &str = "ORE";
static FUEL: &str = "FUEL";

#[derive(Debug)]
struct NanoFactory {
    reactions: ReactionsMap,
    inventory: Inventory,
}

impl NanoFactory {
    fn new(reactions: ReactionsMap, inventory: Inventory) -> Self {
        NanoFactory {
            reactions,
            inventory,
        }
    }

    fn try_produce(&mut self, chemical: &str, quantity: usize) -> Option<()> {
        if quantity == 0 {
            return Some(());
        }

        let mut requirements = vec![(chemical.to_string(), quantity)];
        let mut inventory = self.inventory.clone();

        while !requirements.is_empty() {
            let (chemical, mut required_quantity) = requirements.pop().unwrap();
            assert_ne!(required_quantity, 0);

            if let Some(available) = inventory.0.get_mut(&chemical) {
                let take = min(*available, required_quantity);
                *available -= take;
                required_quantity -= take;
            }

            if required_quantity == 0 {
                continue;
            }

            if chemical == ORE {
                // Out of ore
                return None;
            }

            let reaction = self.reactions.0.get(&chemical).unwrap();
            let reaction_count = required_quantity.div_ceil(&reaction.output.quantity);

            requirements.extend(
                reaction
                    .inputs
                    .iter()
                    .map(|entry| (entry.chemical.clone(), entry.quantity * reaction_count)),
            );

            let leftovers = reaction_count * reaction.output.quantity - required_quantity;
            *inventory.0.entry(chemical).or_insert(0) += leftovers;
        }

        self.inventory = inventory;
        Some(())
    }
}

fn compute_fuel_ore_cost(reactions: ReactionsMap) -> usize {
    let initial_ore = std::usize::MAX;
    let inventory = Inventory::with_chemical(ORE, initial_ore);
    let mut factory = NanoFactory::new(reactions, inventory);
    factory.try_produce(FUEL, 1).unwrap();
    initial_ore - factory.inventory.current_amount(ORE)
}

fn maximum_fuel_for_ore_quantity(reactions: ReactionsMap, ore_quantity: usize) -> usize {
    let inventory = Inventory::with_chemical(ORE, ore_quantity);
    let mut factory = NanoFactory::new(reactions, inventory);

    let mut produced = 0;
    let mut batch = 0x8000;
    while batch != 0 {
        while let Some(()) = factory.try_produce(FUEL, batch) {
            produced += batch;
        }

        batch /= 2;
    }

    produced
}

fn main() {
    let input = get_input(14);
    let reactions: ReactionsMap = input.parse().expect("Malformed ReactionsMap");
    println!(
        "Fuel ore cost: {}",
        compute_fuel_ore_cost(reactions.clone())
    );

    let collected_ore = 1000000000000;
    let max_fuel = maximum_fuel_for_ore_quantity(reactions, collected_ore);
    println!("Maximum fuel for {} ore: {}", collected_ore, max_fuel);
}
