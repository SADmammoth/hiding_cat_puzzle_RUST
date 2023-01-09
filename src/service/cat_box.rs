#[derive(Debug, Clone)]
pub struct CatBox {
    pub has_cat: bool,
}
impl CatBox {
    pub fn new() -> CatBox {
        CatBox { has_cat: false }
    }
    pub fn box_a_cat(&mut self) {
        self.has_cat = true;
    }
    pub fn moveout_cat(&mut self) {
        self.has_cat = false;
    }
    pub fn open_box(&mut self) -> bool {
        self.has_cat
    }
}
