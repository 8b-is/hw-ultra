pub struct Vector {
    pub id: u8,
    pub priority: u8,
}

impl Vector {
    pub fn new(id: u8, priority: u8) -> Self {
        Vector { id, priority }
    }
}
