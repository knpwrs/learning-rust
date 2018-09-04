pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
  where T: Messenger {
      pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
          LimitTracker {
              messenger,
              value: 0,
              max,
          }
      }

      pub fn set_value(&mut self, value: usize) {
          self.value = value;

          let percentage_value = value as f64 / self.max as f64;

          if percentage_value >= 0.75 && percentage_value < 0.9 {
              self.messenger.send("Warning: You have used more than 75% of your quota!");
          } else if percentage_value >= 0.9 && percentage_value < 1.0 {
              self.messenger.send("Urgent Warning: You have used more than 90% of your quota!");
          } else if percentage_value >= 1.0 {
              self.messenger.send("Error: You are over your quota!");
          }
      }
  }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
            // The following will panic at runtime:
            // let mut ref1 = self.sent_messages.borrow_mut();
            // let mut ref2 = self.sent_messages.borrow_mut();
            // ref1.push(String::from(msg));
            // ref2.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 10);
        limit_tracker.set_value(8);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow()[0], "Warning: You have used more than 75% of your quota!");
    }
}
