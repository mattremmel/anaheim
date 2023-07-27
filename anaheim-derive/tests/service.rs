use anaheim_derive::{config, service};

#[service]
struct UserService {}

#[service]
impl UserService {
    pub fn test(&self) -> bool {
        true
    }

    pub fn test2(&self, x: u32, y: u32) -> bool {
        false
    }

    fn test3() {
        ()
    }

    pub fn test4() -> bool {
        true
    }

    pub async fn test5(&self, x: u32, y: u32, z: u32) {
        ()
    }

    async fn test6(&self) {
        ()
    }
}

#[test]
fn test() {
    let _ = MyStructImpl {};
}
