use std::collections::HashMap;

use itertools::Itertools;

use super::boxes::Boxes;

//* WRONG STRATEGY
pub struct Solver {
    strategies: HashMap<Vec<usize>, f32>,
}

fn try_strategy(strategy: &Vec<usize>) -> usize {
    let mut boxes = Boxes::create_boxes(strategy.len());
    boxes.place_cat_in_random_box();
    let mut tries = 0;
    let mut won;
    for try_box in strategy {
        won = boxes.open_box(*try_box);

        tries += 1;
        if won {
            return tries;
        }
    }
    return 0;
}

fn find_strat_score(strategy: &Vec<usize>, tries_count: usize) -> f32 {
    let mut tries_sum = 0;
    for _ in 0..tries_count {
        tries_sum += try_strategy(strategy);
    }
    tries_sum as f32 / tries_count as f32
}

impl Solver {
    pub fn new(boxes_count: usize) -> Solver {
        let items: Vec<usize> = (0..boxes_count).collect();
        let strats = items
            .into_iter()
            .permutations(boxes_count)
            .collect::<Vec<Vec<usize>>>();
        let mut strategies: HashMap<Vec<usize>, f32> = HashMap::new();
        for strat in strats {
            strategies.insert(strat, 0.0);
        }

        println!("{:?}", strategies.keys().len());

        Solver { strategies }
    }

    pub fn find_best_strat(&mut self, tries_count: usize) -> (&Vec<usize>, &f32) {
        self.strategies = self
            .strategies
            .iter()
            .map(|(strat, _)| (strat.clone(), find_strat_score(&strat, tries_count)))
            .collect();

        let mut chart: Vec<_> = self.strategies.iter().collect();
        chart.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

        chart[0]
    }
}
