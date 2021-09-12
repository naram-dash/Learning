use statep::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("i ate salad");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("i ate salad", post.content());
}
