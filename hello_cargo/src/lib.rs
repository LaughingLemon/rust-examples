pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let fraction_of_max = value as f64 / self.max as f64;

        if fraction_of_max > 1.0 {
            println!("error: you are over your max limit of {}", self.max);
        } else if fraction_of_max > 0.67 {
            println!("warning: you are nearing your max limit of {}", self.max);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn test_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 5);

        limit_tracker.set_value(4);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 0);
    }

}
