use hello_cargo::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Something to write about");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("Something to write about", post.content());
}

