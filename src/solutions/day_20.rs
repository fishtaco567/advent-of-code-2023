use core::panic;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use itertools::Itertools;

pub fn solve(input: &str) {
    let mut module_set: ModuleSet = input.into();

    let mut low = 0;
    let mut high = 0;

    let two_away_inputs = vec!["tp", "bk", "pt", "vd"];
    let mut two_away_counts = HashMap::new();

    for i in 0..u64::MAX {
        let mut signal_queue = VecDeque::new();
        signal_queue.push_front(Signal::new("button", "broadcaster", SignalKind::Low));

        while let Some(signal) = signal_queue.pop_front() {
            if i < 1000 {
                match signal.kind {
                    SignalKind::High => high += 1,
                    SignalKind::Low => low += 1,
                }
            }
            module_set.signal(&mut signal_queue, signal);

            for input in &two_away_inputs {
                if !two_away_counts.contains_key(*input)
                    && module_set.modules[*input].last_output == Some(SignalKind::Low)
                {
                    two_away_counts.insert(*input, i + 1);
                }
            }
        }

        let mut failed = false;
        for x in &two_away_inputs {
            failed |= !two_away_counts.contains_key(*x);
        }

        if !failed {
            break;
        }
    }

    println!(
        "{}",
        two_away_counts
            .values()
            .inspect(|s| println!("{s}"))
            .product::<u64>()
    );

    let product = high * low;
    println!("{product}");
}

enum ModuleKind<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, SignalKind>),
}

impl<'a> Hash for ModuleKind<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            ModuleKind::Broadcast => {}
            ModuleKind::FlipFlop(b) => b.hash(state),
            ModuleKind::Conjunction(c) => {
                for kv in c {
                    kv.hash(state);
                }
            }
        }
    }
}

#[derive(Hash)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    last_output: Option<SignalKind>,
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(value: &'a str) -> Self {
        let (me, outputs) = value.split_once(" -> ").unwrap();

        let (name, kind) = if me.starts_with("%") {
            (&me[1..], ModuleKind::FlipFlop(false))
        } else if me.starts_with("&") {
            (&me[1..], ModuleKind::Conjunction(HashMap::new()))
        } else {
            (me, ModuleKind::Broadcast)
        };

        let outputs = outputs.split(",").map(|s| s.trim()).collect_vec();

        Module {
            name,
            kind,
            inputs: Vec::new(),
            outputs,
            last_output: None,
        }
    }
}

impl<'a> Module<'a> {
    fn signal(&mut self, signal: Signal<'a>, signal_queue: &mut VecDeque<Signal<'a>>) {
        match &mut self.kind {
            ModuleKind::Broadcast => {
                self.outputs
                    .iter()
                    .for_each(|s| signal_queue.push_back(Signal::new(self.name, *s, signal.kind)));
                self.last_output = Some(signal.kind);
            }
            ModuleKind::FlipFlop(f) => match signal.kind {
                SignalKind::High => {}
                SignalKind::Low => {
                    let out_signal = if *f {
                        SignalKind::Low
                    } else {
                        SignalKind::High
                    };
                    *f = !*f;
                    self.outputs.iter().for_each(|s| {
                        signal_queue.push_back(Signal::new(self.name, *s, out_signal))
                    });
                    self.last_output = Some(out_signal);
                }
            },
            ModuleKind::Conjunction(c) => {
                if let Some(s) = c.get_mut(signal.origin) {
                    *s = signal.kind;
                } else {
                    panic!();
                }

                let out_signal = if c.values().all(|v| *v == SignalKind::High) {
                    SignalKind::Low
                } else {
                    SignalKind::High
                };

                self.outputs
                    .iter()
                    .for_each(|s| signal_queue.push_back(Signal::new(self.name, *s, out_signal)));
                self.last_output = Some(out_signal);
            }
        }
    }
}

struct ModuleSet<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Hash for ModuleSet<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for module in self.modules.values() {
            module.hash(state);
        }
    }
}

impl<'a> From<&'a str> for ModuleSet<'a> {
    fn from(value: &'a str) -> Self {
        let mut modules = HashMap::new();

        for line in value.lines() {
            let module: Module<'a> = line.into();
            let name = module.name;

            modules.insert(name, module);
        }

        let mut s = Self { modules };

        s.setup_inputs();

        s
    }
}

impl<'a> ModuleSet<'a> {
    fn signal(&mut self, signal_queue: &mut VecDeque<Signal<'a>>, signal: Signal<'a>) {
        let module = self.modules.get_mut(signal.destination);

        //println!("{} -{}-> {}", signal.origin, if signal.kind == SignalKind::High { "high" } else { "low" }, signal.destination);
        if let Some(module) = module {
            module.signal(signal, signal_queue);
        } else {
        }
    }

    fn setup_inputs(&mut self) {
        let all_names = self.modules.keys().map(|x| *x).collect_vec();

        for name in &all_names {
            let mut inputs = Vec::new();

            for module in self.modules.values() {
                for o in &module.outputs {
                    if *o == *name {
                        inputs.push(module.name);
                    }
                }
            }

            let module = self.modules.get_mut(*name).unwrap();
            if let ModuleKind::Conjunction(c) = &mut module.kind {
                for input in &inputs {
                    c.insert(*input, SignalKind::Low);
                }
            }
            module.inputs = inputs;
        }
    }
}

struct Signal<'a> {
    kind: SignalKind,
    origin: &'a str,
    destination: &'a str,
}

impl<'a> Signal<'a> {
    fn new(origin: &'a str, destination: &'a str, kind: SignalKind) -> Self {
        Self {
            kind,
            origin,
            destination,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SignalKind {
    High,
    Low,
}
