struct Door {
    is_open: bool
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open: is_open }
    }

    fn echo(& self) {
        println!("state={:?}", self.is_open);
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn door_open() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }
}

fn main() {
    let mut door = Door::new(false);
    door.echo();
    door.open();
    door.echo();
}

