use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use std::collections::HashSet;

const NAT_ADDRESS: usize = 255;

type Payload = (isize, isize);

#[derive(Debug)]
struct Computer {
    machine: IntcodeMachine,
    idle_count: usize,
}

impl Computer {
    fn new(nic_program: &Tape, address: usize) -> Self {
        let machine = IntcodeMachine::new(nic_program.clone());
        machine.input.borrow_mut().push_back(address as isize);
        Computer {
            machine,
            idle_count: 0,
        }
    }
}

#[derive(Debug)]
struct Network {
    computers: Vec<Computer>,
    first_nat_packet: Option<Payload>,
    last_nat_packet: Option<Payload>,
}

impl Network {
    fn new(nic_program: &Tape, count: usize) -> Self {
        let computers: Vec<_> = (0..count)
            .map(|addr| Computer::new(nic_program, addr))
            .collect();
        Network {
            computers,
            first_nat_packet: None,
            last_nat_packet: None,
        }
    }

    fn enqueue(&mut self, dest_addr: usize, payload: Payload) {
        if dest_addr == NAT_ADDRESS {
            self.first_nat_packet.get_or_insert(payload);
            self.last_nat_packet = Some(payload);
        } else {
            let comp = &mut self.computers[dest_addr];
            comp.idle_count = 0;
            let mut dest_input = comp.machine.input.borrow_mut();
            dest_input.push_back(payload.0);
            dest_input.push_back(payload.1);
        }
    }

    fn run(&mut self) {
        let mut nat_packet_history = HashSet::<Payload>::new();

        loop {
            for i in 0..self.computers.len() {
                let comp = &mut self.computers[i];
                match comp.machine.run().unwrap() {
                    StopStatus::Halted => panic!("NICs should run forever"),
                    StopStatus::BlockedOnInput => {
                        comp.idle_count += 1;
                        comp.machine.input.borrow_mut().push_back(-1);
                    }
                }

                let output: Vec<_> = comp.machine.output.borrow_mut().drain(..).collect();
                assert!(output.len() % 3 == 0);
                drop(comp);

                for packet in output.chunks(3) {
                    let dest_addr = packet[0] as usize;
                    let payload = (packet[1], packet[2]);
                    self.enqueue(dest_addr, payload);
                }
            }

            if self.computers.iter().any(|c| c.idle_count <= 5) {
                continue;
            }

            let packet = self.last_nat_packet.unwrap();
            self.enqueue(0, packet);
            if !nat_packet_history.insert(packet) {
                return;
            }
        }
    }
}

fn main() {
    let input = get_input(23);
    let nic_program = parse_intcode_program(&input);
    let mut network = Network::new(&nic_program, 50);
    network.run();
    println!(
        "First packet sent to NAT: {:?}",
        network.first_nat_packet.unwrap()
    );
    println!(
        "First duplicate NAT packet: {:?}",
        network.last_nat_packet.unwrap()
    );
}
