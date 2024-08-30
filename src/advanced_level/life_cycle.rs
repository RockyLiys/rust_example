use serde::de::Unexpected::Str;
use crate::main;

#[derive(Debug)]
struct Foo;
impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}
#[allow(unused)]
fn test_foo_main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    loan.share();
    println!("{:?}", loan);
}

#[allow(unused)]
fn test_hash() {
    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<K, V>(map: &mut HashMap<K, V>, key: K) -> &mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        if !map.contains_key(&key) {
            map.insert(key.clone(), V::default());
        }
        map.get_mut(&key).unwrap()

    }
    let mut map: HashMap<String, _> = HashMap::new();
    map.insert(String::from("name"), String::from("lisa"));
    map.insert(String::from("age"), String::from("34"));
    let v = get_default(&mut map, String::from("name"));
    println!("{}", v);
}

struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b, 'a>  {
    pub fn noop(self) {
        println!("interface consumed!");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}
impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b,'a>
    where 'a: 'b
    {
        Interface{
            manager: &mut self.manager
        }
    }
}
fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
#[allow(unused)]
fn use_list_test() {
    let mut list = List{
        manager: Manager{
            text: "hello"
        }
    };
    let n_list = list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

#[allow(unused)]
pub(crate) fn life_cycle() {
    // test_foo_main();
    // test_hash();
    use_list_test();
}

