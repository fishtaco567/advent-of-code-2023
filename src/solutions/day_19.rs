use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

pub fn solve(input: &str) {
    let (workflows_input, parts_input) = input.split_once("\r\n\r\n").unwrap();

    let mut workflows = HashMap::new();
    for wf in workflows_input.lines() {
        let (start, rest) = wf.split_once("{").unwrap();

        let workflow: Workflow = (&rest[..rest.len() - 1]).into();

        workflows.insert(start, workflow);
    }

    let parts: Vec<Part> = parts_input.lines().map(|s| s.into()).collect_vec();

    let mut sum = 0;

    for part in parts {
        let mut cur = Location::Workflow("in");

        while let Location::Workflow(s) = cur {
            let wf = &workflows[s];

            let next = wf.process(&part);

            cur = next;
        }

        if cur == Location::Accept {
            sum += part.x_cool + part.musical + part.aero + part.shiny;
        }
    }

    let mega_part = MegaPart {
        x_cool: 1..4001,
        musical: 1..4001,
        aero: 1..4001,
        shiny: 1..4001,
    };

    let mut mps = Vec::new();
    mps.push((mega_part, Location::Workflow("in")));

    let mut sum_2: u64 = 0;

    while let Some((mp, loc)) = mps.pop() {
        match loc {
            Location::Accept => {
                let s = sab(mp.x_cool) * sab(mp.musical) * sab(mp.aero) * sab(mp.shiny);
                sum_2 += s;
            }
            Location::Reject => {}
            Location::Workflow(w) => {
                let wf = &workflows[w];
                for (mp, loc) in wf.process_mega(mp) {
                    mps.push((mp, loc));
                }
            }
        }
    }

    println!("{sum}");
    println!("{sum_2}");
}

fn sab(r: Range<u32>) -> u64 {
    let m = r.end - r.start;
    m as u64
}

struct Part {
    x_cool: u32,
    musical: u32,
    aero: u32,
    shiny: u32,
}

#[derive(Clone)]
struct MegaPart {
    x_cool: Range<u32>,
    musical: Range<u32>,
    aero: Range<u32>,
    shiny: Range<u32>,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let v = &value[1..value.len() - 1];

        let mut x_cool = 0;
        let mut musical = 0;
        let mut aero = 0;
        let mut shiny = 0;

        for c in v.split(',') {
            let (l, n) = c.split_once('=').unwrap();
            let n = n.parse::<u32>().unwrap();

            match l {
                "x" => x_cool = n,
                "m" => musical = n,
                "a" => aero = n,
                "s" => shiny = n,
                _ => panic!(),
            }
        }

        Self {
            x_cool,
            musical,
            aero,
            shiny,
        }
    }
}

struct Workflow<'a> {
    instrs: Vec<Instr<'a>>,
    reject: Location<'a>,
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let csv = value.split(',').collect_vec();

        let mut instrs = Vec::new();
        for i in 0..csv.len() - 1 {
            instrs.push(csv[i].into());
        }

        let reject = csv.last().map(|x| *x).unwrap().into();

        Self { instrs, reject }
    }
}

impl<'a> Workflow<'a> {
    fn process(&'a self, part: &Part) -> Location<'a> {
        for instr in &self.instrs {
            if instr.passes(part) {
                return instr.location;
            }
        }

        return self.reject;
    }

    fn process_mega(&'a self, mut mega_part: MegaPart) -> Vec<(MegaPart, Location<'a>)> {
        let mut out = Vec::new();
        for instr in &self.instrs {
            match instr.passes_mega(&mega_part) {
                Passes::Fail => {}
                Passes::Pass(p) => {
                    out.push((p, instr.location));
                }
                Passes::Split(pass, fail) => {
                    out.push((pass, instr.location));
                    mega_part = fail;
                }
            }
        }
        out.push((mega_part, self.reject));

        out
    }
}

struct Instr<'a> {
    req: Req,
    comparison: Comparison,
    location: Location<'a>,
}

impl<'a> Instr<'a> {
    fn passes(&self, part: &Part) -> bool {
        let (r, q) = match self.req {
            Req::XCool(n) => (n, part.x_cool),
            Req::Musical(n) => (n, part.musical),
            Req::Aero(n) => (n, part.aero),
            Req::Shiny(n) => (n, part.shiny),
        };

        match self.comparison {
            Comparison::Lesser => q < r,
            Comparison::Greater => q > r,
        }
    }

    fn passes_mega(&self, part: &MegaPart) -> Passes {
        let (r, q) = match self.req {
            Req::XCool(n) => (n, &part.x_cool),
            Req::Musical(n) => (n, &part.musical),
            Req::Aero(n) => (n, &part.aero),
            Req::Shiny(n) => (n, &part.shiny),
        };

        match self.comparison {
            Comparison::Lesser => {
                if q.end - 1 < r {
                    Passes::Fail
                } else {
                    if q.end == r {
                        Passes::Pass(part.clone())
                    } else {
                        let mut c1 = part.clone();
                        let mut c2 = part.clone();
                        let lower = q.start..r;
                        let upper = r..q.end;
                        match self.req {
                            Req::XCool(_) => {
                                c1.x_cool = lower;
                                c2.x_cool = upper;
                            }
                            Req::Musical(_) => {
                                c1.musical = lower;
                                c2.musical = upper;
                            }
                            Req::Aero(_) => {
                                c1.aero = lower;
                                c2.aero = upper;
                            }
                            Req::Shiny(_) => {
                                c1.shiny = lower;
                                c2.shiny = upper;
                            }
                        }

                        Passes::Split(c1, c2)
                    }
                }
            }
            Comparison::Greater => {
                if q.end - 1 < r {
                    Passes::Fail
                } else {
                    if q.start == r + 1 {
                        Passes::Pass(part.clone())
                    } else {
                        let mut c1 = part.clone();
                        let mut c2 = part.clone();
                        let lower = q.start..(r + 1);
                        let upper = (r + 1)..q.end;
                        match self.req {
                            Req::XCool(_) => {
                                c1.x_cool = upper;
                                c2.x_cool = lower;
                            }
                            Req::Musical(_) => {
                                c1.musical = upper;
                                c2.musical = lower;
                            }
                            Req::Aero(_) => {
                                c1.aero = upper;
                                c2.aero = lower;
                            }
                            Req::Shiny(_) => {
                                c1.shiny = upper;
                                c2.shiny = lower;
                            }
                        }

                        Passes::Split(c1, c2)
                    }
                }
            }
        }
    }
}

enum Passes {
    Fail,
    Pass(MegaPart),
    Split(MegaPart, MegaPart),
}

enum Req {
    XCool(u32),
    Musical(u32),
    Aero(u32),
    Shiny(u32),
}

impl<'a> From<&'a str> for Instr<'a> {
    fn from(value: &'a str) -> Self {
        let (req, loc) = value.split_once(':').unwrap();

        let kind = &req[0..1];

        let comparison = &req[1..2];
        let comparison = match comparison {
            ">" => Comparison::Greater,
            "<" => Comparison::Lesser,
            _ => panic!(),
        };

        let num = &req[2..];
        let num = num.parse::<u32>().unwrap();

        let req = match kind {
            "x" => Req::XCool(num),
            "m" => Req::Musical(num),
            "a" => Req::Aero(num),
            "s" => Req::Shiny(num),
            _ => panic!(),
        };

        let location = loc.into();

        Instr {
            req,
            comparison,
            location,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Location<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

impl<'a> From<&'a str> for Location<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Location::Accept,
            "R" => Location::Reject,
            _ => Location::Workflow(value),
        }
    }
}

enum Comparison {
    Lesser,
    Greater,
}
