use std::rc::Rc;

//  Stack(ish) immutable linked-list useless
pub enum StackImmList<'a> {
    Empty,
    Elem(i32, &'a StackImmList<'a>),
}

// Heap-allocated but Box'd, single-onwership, not dynamic memership
pub enum BoxList {
    Empty,
    Elem(i32, Box<BoxList>), // Uses Box for heap allocation
}

pub enum RcList {
    Empty,
    Elem(i32, Rc<RcList>),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rc_list() {
        let list = RcList::Elem(1, Rc::new(RcList::Elem(2, Rc::new(RcList::Empty))));

        match &list {
            RcList::Elem(val, next) => {
                assert_eq!(*val, 1);
                match &**next {
                    RcList::Elem(val, next) => {
                        assert_eq!(*val, 2);
                        match &**next {
                            RcList::Empty => assert!(true),
                            _ => assert!(false, "Expected Empty"),
                        }
                    }
                    _ => assert!(false, "Expected Elem"),
                }
            }
            _ => assert!(false, "Expected Elem"),
        }
    }

    #[test]
    fn test_box_list() {
        let box_list = BoxList::Elem(1, Box::new(BoxList::Elem(2, Box::new(BoxList::Empty))));
        match &box_list {
            BoxList::Elem(val, next) => {
                assert_eq!(*val, 1);
                match &**next {
                    BoxList::Elem(val, next) => {
                        assert_eq!(*val, 2);
                        match &**next {
                            BoxList::Empty => assert!(true),
                            _ => assert!(false, "Expected Empty"),
                        }
                    }
                    _ => assert!(false, "Expected Elem"),
                }
            }
            _ => assert!(false, "Expected Elem"),
        }
    }

    #[test]
    fn test_stack_imm_list() {
        let stack_list = StackImmList::Elem(1, &StackImmList::Elem(2, &StackImmList::Empty));
        match &stack_list {
            StackImmList::Elem(val, next) => {
                assert_eq!(*val, 1);
                match &**next {
                    StackImmList::Elem(val, next) => {
                        assert_eq!(*val, 2);
                        match &**next {
                            StackImmList::Empty => assert!(true),
                            _ => assert!(false, "Expected Empty"),
                        }
                    }
                    _ => assert!(false, "Expected Elem"),
                }
            }
            _ => assert!(false, "Expected Elem"),
        }
    }
}
