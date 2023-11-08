use ordered_float::OrderedFloat;

// The giant-tour TSP for VRP
//
// Local Search for Vehicle Routing and Scheduling Problems: Review and
// Conceptual Integration, Funke et al. (2005)
pub fn solve(m: usize, xs: &[(f64, f64)]) -> (f64, Vec<Vec<usize>>) {
    let n = xs.len();
    let mut dp = vec![vec![vec![f64::INFINITY; n]; m]; 1 << n];

    for i in 0..n {
        dp[0][0][i] = 0.0;
    }

    for i in 0..1 << n {
        for j in 0..m {
            for k in 0..n {
                if dp[i][j][k].is_infinite() {
                    continue;
                }

                for l in 0..n {
                    if 1 << l & i > 0 {
                        continue;
                    }

                    let ii = i | 1 << l;

                    dp[ii][j][l] = dp[ii][j][l].min(dp[i][j][k] + distance(k, l, xs));

                    if j + 1 < m {
                        // We change a vehicle and either:
                        // - Stay at the same stop.
                        // - "Warp" to a new stop.
                        for (ii, kk) in [(i, k), (ii, l)] {
                            dp[ii][j + 1][kk] = dp[ii][j + 1][kk].min(dp[i][j][k]);
                        }
                    }
                }
            }
        }
    }

    let (k, &y) = dp
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .min_by_key(|(_, &x)| OrderedFloat(x))
        .unwrap();

    (y, reconstruct(xs, &dp, k, y))
}

fn distance(i: usize, j: usize, xs: &[(f64, f64)]) -> f64 {
    ((xs[i].0 - xs[j].0).powi(2) + (xs[i].1 - xs[j].1).powi(2)).sqrt()
}

fn reconstruct(
    xs: &[(f64, f64)],
    dp: &[Vec<Vec<f64>>],
    mut k: usize,
    mut y: f64,
) -> Vec<Vec<usize>> {
    if xs.is_empty() || dp[0].is_empty() {
        return Default::default();
    }

    let mut zs = vec![];
    let mut i = dp.len() - 1;
    let mut j = dp[0].len() - 1;

    while i > 0 {
        i &= !(1 << k);

        (j, k, y) = dp[i]
            .iter()
            .enumerate()
            .filter(|(jj, _)| *jj == j.saturating_sub(1) || *jj == j)
            .flat_map(|(j, ys)| {
                ys.iter()
                    .enumerate()
                    .filter(|(kk, _)| i & 1 << kk > 0)
                    .map(move |(kk, x)| (j, kk, OrderedFloat((y - x - distance(kk, k, xs)).abs())))
            })
            .min_by_key(|&(_, _, x)| x)
            .map(|(j, k, y)| (j, k, y.0))
            .unwrap();

        zs.push((j, k));
    }

    zs.reverse();

    let mut vs = vec![];
    let mut jj = usize::MAX;

    for (j, k) in zs {
        if j != jj {
            vs.push(vec![]);
            jj = j;
        }

        vs.last_mut().unwrap().push(k);
    }

    vs
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    mod single_vehicle {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_stop() {
            assert_eq!(solve(1, &[(0.0, 0.0)]).0, 0.0);
        }

        #[test]
        fn two_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(1, &stops).0, 1.0);
            }
        }

        #[test]
        fn three_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(1, &stops).0, 2.0);
            }
        }

        #[test]
        fn four_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(1, &stops).0, 3.0);
            }
        }

        #[test]
        fn five_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(1, &stops).0, 4.0);
            }
        }
    }

    mod two_vehicles {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_stop() {
            assert_eq!(solve(2, &[(0.0, 0.0)]).0, 0.0);
        }

        #[test]
        fn two_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(2, &stops).0, 0.0);
            }
        }

        #[test]
        fn three_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(2, &stops).0, 1.0);
            }
        }

        #[test]
        fn four_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(2, &stops).0, 2.0);
            }
        }

        #[test]
        fn five_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(2, &stops).0, 3.0);
            }
        }
    }

    mod three_vehicles {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_stop() {
            assert_eq!(solve(3, &[(0.0, 0.0)]).0, 0.0);
        }

        #[test]
        fn two_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(3, &stops).0, 0.0);
            }
        }

        #[test]
        fn three_stops() {
            let stops = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(3, &stops).0, 0.0);
            }
        }

        #[test]
        fn four_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(3, &stops).0, 1.0);
            }
        }

        #[test]
        fn five_stops() {
            let stops = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), (2.0, 0.0)];

            for stops in stops.into_iter().permutations(stops.len()) {
                assert_eq!(solve(3, &stops).0, 2.0);
            }
        }
    }

    mod reconstruct {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn one_stop() {
            assert_eq!(solve(1, &[(0.0, 0.0)]), (0.0, vec![vec![0]]));
        }

        #[test]
        fn two_stops() {
            assert_eq!(solve(1, &[(0.0, 0.0), (1.0, 0.0)]), (1.0, vec![vec![0, 1]]));
        }
    }
}
