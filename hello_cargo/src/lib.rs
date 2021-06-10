pub struct Post {
    content: String
}

impl Post {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }
}

pub struct DraftPost {
    content: String
}

impl DraftPost {
    pub fn add_text(&mut self, p0: &str) {
        self.content.push_str(p0);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }

}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}
