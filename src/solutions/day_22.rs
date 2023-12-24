use itertools::Itertools;

use crate::iter_helper::IterHelper;

pub fn solve(input: &str) {
    let mut cubes: Vec<Option<Cube>> = input.lines().map(|s| Some(s.into())).collect();

    let (c, _) = settle(&mut cubes);

    println!("Part 1 is {c}");

    let mut all_fallen = 0;

    for i in 0..cubes.len() {
        let mut c2 = cubes.clone();
        c2.remove(i);

        let (_, f) = settle(&mut c2);
        all_fallen += f;
    }

    println!("Part 2 is {all_fallen}");
}

const S1: usize = 100000;
const S2: usize = 100001;

fn settle(cubes: &mut Vec<Option<Cube>>) -> (usize, usize) {
    cubes.sort_by(|a, b| a.as_ref().unwrap().pz.cmp(&b.as_ref().unwrap().pz));
    
    let mut v = vec![S1; cubes.len()];

    let mut fallen = 0;

    for i in 0..cubes.len() {
        let mut active_cube = cubes[i].clone().unwrap();
        cubes[i] = None;

        let mut max_below = 1;

        for j in 0..i {
            let cube = cubes.get(j).unwrap();

            if let Some(cube) = cube {
                if cube.max() > active_cube.pz {
                    continue;
                }

                'outer:
                for x in active_cube.px..active_cube.px + active_cube.ex {
                    for y in active_cube.py..active_cube.py + active_cube.ey {
                        if let Some(q) = cube.max_in_column(x, y) {
                            if q > max_below {
                                v[i] = j;
                                max_below = max_below.max(q);
                            } else if q == max_below {
                                v[i] = S2;
                            }
                            break 'outer;
                        }                            
                    }
                }
            }
        }
        
        if active_cube.pz != max_below {
            fallen += 1;
            active_cube.pz = max_below;
        }

        cubes[i] = Some(active_cube);
    }

    let mut solo_support = vec![false; cubes.len()];

    for vv in &v {
        if *vv < S1 {
            solo_support[*vv] = true;
        }
    }

    let c = solo_support.iter().filter(|b| !**b).count();

    (c, fallen)
}

#[derive(Clone)]
struct Cube {
    px: u32,
    py: u32,
    pz: u32,
    ex: u32,
    ey: u32,
    ez: u32,
}

impl Cube {
    fn max(&self) -> u32 {
        self.pz + self.ez
    }

    fn max_in_column(&self, x: u32, y: u32) -> Option<u32> {
        if self.px <= x && x < self.px + self.ex && self.py <= y && y < self.py + self.ey {
            Some(self.max())
        } else {
            None
        }
    } 
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let (p1, p2) = value.split_once("~").unwrap();

        let p1 = p1.split(",").parse_all::<u32>().collect_vec();
        let p2 = p2.split(",").parse_all::<u32>().collect_vec();

        let px = p1[0].min(p2[0]);
        let py = p1[1].min(p2[1]);
        let pz = p1[2].min(p2[2]);

        let mx = p1[0].max(p2[0]);
        let my = p1[1].max(p2[1]);
        let mz = p1[2].max(p2[2]);

        let ex = (mx - px) + 1;
        let ey = (my - py) + 1;
        let ez = (mz - pz) + 1;

        Cube {
            px,
            py,
            pz,
            ex,
            ey,
            ez
        }
    }
}