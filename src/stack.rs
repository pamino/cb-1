use crate::Stack;

impl Stack for Vec<i32> {
    fn init() -> Self {
        vec![]
    }

    fn push_val(&mut self, i: i32) {
        self.push(i)
    }

    fn top_val(&self) -> Option<&i32> {
        self.last()
    }

    fn pop_val(&mut self) -> Option<i32> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

use ListStack::Nil;
use ListStack::Val;

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            Val(value, other) => {
                match other {
                    Some(ele) => return (*ele).push_val(i),
                    None => *other = Some(Box::new(ListStack::Val(i, None))),
                }
            },
            Nil => *self = ListStack::Val(i, None),
        };
    }

    fn top_val(&self) -> Option<&i32> {
        match self {
            Val(value, other) => {
                match other {
                    Some(ele) => return (*ele).top_val(),
                    None => return Some(value),
                }
            },
            Nil => None
        }
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            Val(value, other) => {
                match other {
                    Some(ele) => {
                        let ret = (*ele).pop_val();
                        if (**ele) == Nil {
                            *other = None;
                        }
                        return ret;
                    },
                    None => {
                        let ret:i32 = *value;
                        *self = Nil;
                        return Some(ret);
                    },
                };
            },
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;

    #[test]
    fn vec_fill_and_clear() {
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    #[test]
    fn linked_fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }
}
