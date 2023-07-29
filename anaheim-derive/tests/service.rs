use anaheim_derive::{config, service};

struct Config {}

#[service]
struct UserService {
    config: Config,
    x: u32,
    #[new(default)]
    y: u32,
    #[new(value = "1")]
    z: u32,
}

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

#[service]
trait UserRepository {
    async fn test();
    fn test2();
}

#[test]
fn test() {
    let actual = UserServiceImpl::new(Config {}, 10);
    let _ = UserService::from(actual);
}
