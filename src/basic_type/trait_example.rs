use std::fmt::Debug;

pub(crate) trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub(crate) struct Post {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) content: String,
}

impl Summary for Post{
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("title: {}, author: {}", self.title, self.author)
    }
}



#[derive(Debug)]
pub(crate) struct Weibo {
    pub(crate) username: String,
    pub(crate) content: String
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{} send weibo: {}", self.username, self.content)
    // }
}

pub fn notify<T: Summary + Debug>(entry: &T) {
    println!("Breaking news! {}", entry.summarize())
}

pub(crate) fn returns_summarize(switch: bool) -> Box<dyn Summary>{
    if switch {
        Box::new(Post {
            title: String::from("post title"),
            author: String::from("Ice_burgh"),
            content: String::from("The Pittsburgh Penguins once again"),
        })
    }else {
        Box::new(Weibo{
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people")
        })
    }
}
