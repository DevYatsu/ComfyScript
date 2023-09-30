// standard library http package
use lazy_static::lazy_static;

use hashbrown::HashMap;

lazy_static! {
    static ref HTTP_FUNCTIONS: HashMap<&'static str, &'static str> = {
        let mut hash = HashMap::new();
        hash.insert("test", "test");
        hash
    };
}
