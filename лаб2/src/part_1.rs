struct New;
struct Unmoderated;
struct Published;
struct Deleted;

struct Post<State>{
    content: String,
    state: std::marker::PhantomData<State>,
}

impl Post<New> {
    fn publish(self) -> Post<Unmoderated> {
        Post { content: self.content, state: std::marker::PhantomData }
    }
}

impl Post<Unmoderated> {
    fn allow(self) -> Post<Published> {
        Post { content: self.content, state: std::marker::PhantomData }
    }

    fn deny(self) -> Post<Deleted> {
        Post { content: self.content, state: std::marker::PhantomData }
    }
}

impl Post<Published> {
    fn delete(self) -> Post<Deleted> {
        Post { content: self.content, state: std::marker::PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_to_unmoderated() {
        let post = Post { 
            content: "hello".to_string(), 
            state: std::marker::PhantomData::<New> 
        };
        let unmoderated = post.publish();
        assert_eq!(unmoderated.content, "hello");
    }

    #[test]
    fn unmoderated_to_published() {
        let post = Post { 
            content: "hello".to_string(), 
            state: std::marker::PhantomData::<New> 
        };
        let unmoderated = post.publish();
        let published = unmoderated.allow();
        assert_eq!(published.content, "hello");
    }

    #[test]
    fn unmoderated_to_deleted() {
        let post = Post { 
            content: "test".to_string(), 
            state: std::marker::PhantomData::<New> 
        };
        let unmoderated = post.publish();
        let deleted = unmoderated.deny();
        assert_eq!(deleted.content, "test");
    }

    #[test]
    fn published_to_deleted() {
        let post = Post { 
            content: "bye".to_string(), 
            state: std::marker::PhantomData::<New> 
        };
        let unmoderated = post.publish();
        let published = unmoderated.allow();
        let deleted = published.delete();
        assert_eq!(deleted.content, "bye");
    }
}
