pub struct Ship {
    id: char,
    length: u32,
    damage: u32,
    afloat: bool,
}

impl Ship {
    pub fn new(id:char, length: u32) -> Ship {
        Ship {  id: id,
                length: length,
                damage: 0,
                afloat: true,
        }
    }

    pub fn get_id(&slef) -> char {
        self.id
    }

    pub fn hit(&mut self) {
        self.damage = self.damage + 1;
        if self.damage == self.length {
            self.afloat = false;
        }
    }

    pub fn is_afloat(&self) -> bool {
        self.afloat
    }
}



