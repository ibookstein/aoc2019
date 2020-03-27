use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use itertools::Itertools;
use std::iter::FromIterator;

struct MachineState {
    machine: IntcodeMachine,
    status: StopStatus,
}

impl MachineState {
    fn new(machine: IntcodeMachine) -> Self {
        MachineState {
            machine,
            status: StopStatus::Running,
        }
    }

    fn resume(&mut self) {
        self.status = self.machine.run().expect("Error while running machine");
    }
}

struct Pipeline {
    pipeline: Vec<MachineState>,
}

impl Pipeline {
    fn new(
        program: Tape,
        phase_settings: Vec<&isize>,
        first_input: StreamRef,
        last_output: StreamRef,
    ) -> Self {
        let mut streams = Vec::<StreamRef>::new();
        for (i, setting) in phase_settings.iter().enumerate() {
            let stream = if i == 0 {
                first_input.clone()
            } else {
                new_stream_ref()
            };
            stream.borrow_mut().push_back(**setting);
            streams.push(stream);
        }
        streams.push(last_output);

        let mut pipeline = Vec::<MachineState>::new();
        for w in streams.windows(2) {
            let machine = IntcodeMachine::new_io(program.clone(), w[0].clone(), w[1].clone());
            pipeline.push(MachineState::new(machine));
        }

        Pipeline { pipeline }
    }

    fn completed(&self) -> bool {
        self.pipeline
            .iter()
            .all(|ms| ms.status == StopStatus::Halted)
    }

    fn run_to_completion(&mut self) {
        while !self.completed() {
            for ms in self.pipeline.iter_mut() {
                ms.resume();
            }
        }
    }
}

fn calculate_thruster_signal_linear(program: Tape, phase_settings: Vec<&isize>) -> isize {
    let input = new_stream_ref();
    let output = new_stream_ref();
    let mut pipeline = Pipeline::new(program, phase_settings, input.clone(), output.clone());

    input.borrow_mut().push_back(0);
    pipeline.run_to_completion();

    let signal = output.borrow_mut().pop_back().unwrap();
    signal
}

fn calculate_thruster_signal_feedback(program: Tape, phase_settings: Vec<&isize>) -> isize {
    let inout = new_stream_ref();
    let mut pipeline = Pipeline::new(program, phase_settings, inout.clone(), inout.clone());

    inout.borrow_mut().push_back(0);
    pipeline.run_to_completion();

    let signal = inout.borrow_mut().pop_back().unwrap();
    signal
}

fn calculate_max_thruster_signal(
    program: &Tape,
    phase_settings: Vec<isize>,
    func: impl Fn(Tape, Vec<&isize>) -> isize,
) -> isize {
    phase_settings
        .iter()
        .permutations(phase_settings.len())
        .map(|perm| func(program.clone(), perm))
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
