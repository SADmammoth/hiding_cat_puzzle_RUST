use super::boxes::Boxes;

fn try_random_strategy(boxes_count: usize, max_tries: usize) -> Option<(Vec<usize>, usize)> {
    let mut boxes = Boxes::create_boxes(boxes_count);
    boxes.place_cat_in_random_box();
    let mut tries = 0;
    let mut won;
    let mut index;
    let mut strat: Vec<usize> = vec![];
    for _ in 0..max_tries {
        (index, won) = boxes.open_random_box();
        strat.push(index);
        tries += 1;
        if won {
            return Some((strat, tries));
        }
    }
    return None;
}

fn try_strategy(boxes_count: usize, strategy: &Vec<usize>) -> Option<usize> {
    let mut boxes = Boxes::create_boxes(boxes_count);
    boxes.place_cat_in_random_box();
    let mut tries = 0;
    let mut won;
    for try_box in strategy {
        won = boxes.open_box(*try_box);

        tries += 1;
        if won {
            return Some(tries);
        }
    }
    return None;
}

pub fn find_best_strats(
    boxes_count: usize,
    tries_to_find_strat: usize,
    max_tries: usize,
) -> Vec<(Vec<usize>, usize)> {
    let mut confirmed_strats: Vec<(Vec<usize>, usize)> = vec![];
    'strats_checker: for _ in 0..tries_to_find_strat {
        if let Some((strat, tries)) = try_random_strategy(boxes_count, max_tries) {
            let mut worst_case = tries;
            for _ in 0..tries_to_find_strat {
                if let Some(tries) = try_strategy(boxes_count, &strat) {
                    if tries > worst_case {
                        worst_case = tries;
                    }
                } else {
                    continue 'strats_checker;
                }
            }
            confirmed_strats.push((strat, worst_case));
        }
    }

    confirmed_strats.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));
    confirmed_strats
}
