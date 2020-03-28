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

#[derive(Debug)]
struct NanoFactory {
    reactions: HashMap<String, Reaction>,
}

impl FromStr for NanoFactory {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reactions: Result<Vec<Reaction>, _> = s.trim().lines().map(|s| s.parse()).collect();
        Ok(NanoFactory {
            reactions: reactions?
                .drain(..)
                .map(|r| (r.output.chemical.clone(), r))
                .collect(),
        })
    }
}

static ORE: &str = "ORE";
const INITIAL_ORE: usize = std::usize::MAX;
static FUEL: &str = "FUEL";

impl NanoFactory {
    fn ore_cost(&self, chemical: &str) -> usize {
        let mut requirements = vec![(chemical, 1)];
        let mut inventory: HashMap<&str, usize> = HashMap::new();
        inventory.insert(ORE, INITIAL_ORE);

        while !requirements.is_empty() {
            let (chemical, mut required_quantity) = requirements.pop().unwrap();

            if let Some(available) = inventory.get_mut(chemical) {
                let take = min(*available, required_quantity);
                *available -= take;
                required_quantity -= take;
            }

            if required_quantity == 0 {
                continue;
            }

            let reaction = self.reactions.get(chemical).unwrap();
            let reaction_count = required_quantity.div_ceil(&reaction.output.quantity);

            requirements.extend(
                reaction
                    .inputs
                    .iter()
                    .map(|entry| (entry.chemical.as_str(), entry.quantity * reaction_count)),
            );

            let leftovers = reaction_count * reaction.output.quantity - required_quantity;
            *inventory.entry(chemical).or_insert(0) += leftovers;
        }

        INITIAL_ORE - inventory.get(ORE).unwrap()
    }
}

fn main() {
    let input = get_input(14);
    let factory: NanoFactory = input.parse().expect("Malformed NanoFactory input");
    println!("Ore cost of {}: {}", FUEL, factory.ore_cost(FUEL));
}
