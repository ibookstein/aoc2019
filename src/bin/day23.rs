use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;

const BROADCAST_ADDR: usize = 255;

#[derive(Debug)]
struct Network {
    computers: Vec<IntcodeMachine>,
}

impl Network {
    fn new(nic_program: &Tape, count: usize) -> Self {
        let mut computers = Vec::with_capacity(count);
        for i in 0..count {
            let machine = IntcodeMachine::new(nic_program.clone());
            machine.input.borrow_mut().push_back(i as isize);
            computers.push(machine);
        }
        Network { computers }
    }

    fn run(&mut self) -> (isize, isize) {
        loop {
            for i in 0..self.computers.len() {
                let comp = &mut self.computers[i];
                match comp.run().unwrap() {
                    StopStatus::Halted => continue,
                    StopStatus::BlockedOnInput => {
                        comp.input.borrow_mut().push_back(-1);
                    },
                }

                let output: Vec<_> = comp.output.borrow_mut().drain(..).collect();
                drop(comp);

                for packet in output.chunks(3) {
                    let dest_addr = packet[0] as usize;
                    let x = packet[1];
                    let y = packet[2];

                    if dest_addr == BROADCAST_ADDR {
                        return (x, y);
                    }

                    let mut dest_input = self.computers[dest_addr].input.borrow_mut();
                    dest_input.push_back(x);
                    dest_input.push_back(y);
                }
            }
        }
    }
}

fn main() {
    let input = get_input(23);
    let nic_program = parse_intcode_program(&input);
    let mut network = Network::new(&nic_program, 50);
    let broadcast_packet = network.run();
    println!(
        "First packet sent to {}: {:?}",
        BROADCAST_ADDR, broadcast_packet
    );
}
