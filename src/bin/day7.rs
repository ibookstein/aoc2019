use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use itertools::Itertools;
use std::iter::FromIterator;

struct MachineState {
    machine: IntcodeMachine,
    status: StopStatus,
}

impl MachineState {
    fn create_with_phase(program: &Tape, phase: isize) -> MachineState {
        let mut input = Stream::new();
        input.push_back(phase);
        MachineState {
            machine: IntcodeMachine::new(program.clone(), input),
            status: StopStatus::Running,
        }
    }

    fn pump_input_from(&mut self, stream: &mut Stream) {
        while let Some(value) = stream.pop_front() {
            self.machine.input.push_back(value);
        }
    }

    fn resume(&mut self) {
        self.status = self.machine.run().expect("Error while running machine");
    }
}

fn calculate_thruster_signal_linear(program: &Tape, phase_settings: Vec<&isize>) -> isize {
    let mut signal = 0isize;

    for phase_setting in phase_settings {
        let input = Stream::from_iter([*phase_setting, signal].iter().cloned());
        let mut machine = IntcodeMachine::new(program.clone(), input);
        machine
            .run_to_completion()
            .expect("Did not run to completion");
        signal = machine.output[0];
    }

    signal
}

fn calculate_thruster_signal_feedback(program: &Tape, phase_settings: Vec<&isize>) -> isize {
    let mut machine_states: Vec<_> = phase_settings
        .iter()
        .map(|&phase| MachineState::create_with_phase(&program, *phase))
        .collect();

    let count = phase_settings.len();
    machine_states[0].pump_input_from(&mut Stream::from(vec![0isize]));

    while !machine_states
        .iter()
        .all(|ms| ms.status == StopStatus::Halted)
    {
        for i in 0..count {
            let prev_idx = if i == 0 { count - 1 } else { i - 1 };
            let mut prev_output: Stream =
                machine_states[prev_idx].machine.output.drain(..).collect();

            machine_states[i].pump_input_from(&mut prev_output);
            machine_states[i].resume();
        }
    }

    let last_out = &machine_states.last().unwrap().machine.output;
    assert_eq!(last_out.len(), 1);
    last_out[0]
}

fn calculate_max_thruster_signal(
    program: &Tape,
    phase_settings: Vec<isize>,
    func: impl Fn(&Tape, Vec<&isize>) -> isize,
) -> isize {
    phase_settings
        .iter()
        .permutations(phase_settings.len())
        .map(|perm| func(&program, perm))
        .max()
        .unwrap()
}

fn main() {
    let program = parse_intcode_program(&get_input(7));

    println!(
        "Maximum thruster signal (no feedback): {}",
        calculate_max_thruster_signal(
            &program,
            Vec::from_iter(0..=4),
            calculate_thruster_signal_linear
        )
    );
    println!(
        "Maximum thruster signal (with feedback): {}",
        calculate_max_thruster_signal(
            &program,
            Vec::from_iter(5..=9),
            calculate_thruster_signal_feedback
        )
    );
}
