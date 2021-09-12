use rustway::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("i ate salad");
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("i ate salad", post.content());
}
