use std::fmt::Display;

use super::cat_box::CatBox;
use rand::Rng;

pub struct Boxes {
    boxes: Vec<Box<CatBox>>,
    pub box_with_cat: usize,
}

impl Boxes {
    pub fn create_boxes(count: usize) -> Boxes {
        Boxes {
            boxes: vec![Box::new(CatBox::new()); count],
            box_with_cat: 0,
        }
    }

    fn get_random_box_index(&self) -> usize {
        rand::thread_rng().gen_range(0..self.boxes.len())
    }

    pub fn place_cat_in_random_box(&mut self) {
        let box_index = self.get_random_box_index();
        self.boxes[box_index].box_a_cat();
        self.box_with_cat = box_index;
    }

    fn move_cat(&mut self) {
        let left_box_open = self.box_with_cat == 0;
        let right_box_open = self.box_with_cat == self.boxes.len() - 1;
        if left_box_open && right_box_open {
            return;
        }

        self.boxes[self.box_with_cat].moveout_cat();
        let mut move_left = rand::thread_rng().gen_bool(0.5);
        if left_box_open {
            move_left = false;
        } else if right_box_open {
            move_left = true;
        }
        if move_left {
            self.box_with_cat -= 1;
        } else {
            self.box_with_cat += 1
        }
        self.boxes[self.box_with_cat].box_a_cat();
    }

    pub fn open_box(&mut self, box_index: usize) -> bool {
        let found_cat = self.boxes[box_index].open_box();
        if !found_cat {
            self.move_cat();
        }
        found_cat
    }

    pub fn open_random_box(&mut self) -> (usize, bool) {
        let box_index = self.get_random_box_index();
        (box_index, self.open_box(box_index))
    }
}

impl Display for Boxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .boxes
            .iter()
            .map(|catbox| if catbox.has_cat { "1" } else { "1" })
            .collect::<Vec<&str>>()
            .join(" ");
        write!(f, "{}", str)
    }
}
