use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node<K> {
    key: K,
    next: Vec<Option<Rc<RefCell<Node<K>>>>>,
}

impl<K> Node<K> {
    fn new(key: K, height: usize) -> Self {
        Node {
            key,
            next: vec![None; height],
        }
    }
}

struct SkipList<K: Default, C> {
    head: Rc<RefCell<Node<K>>>,
    max_height: usize,
    compare: C,
}

impl<K: Default, C> SkipList<K, C>
where
    K: Ord + Clone,
    C: Fn(&K, &K) -> std::cmp::Ordering,
{
    const MAX_HEIGHT: usize = 12;

    fn new(compare: C) -> Self {
        let head = Rc::new(RefCell::new(Node::new(K::default(), Self::MAX_HEIGHT)));
        SkipList {
            head,
            max_height: 1,
            compare,
        }
    }

    fn random_height() -> usize {
        let mut rng = rand::thread_rng();
        let mut height = 1;
        while height < Self::MAX_HEIGHT && rng.gen_range(0..4) == 0 {
            height += 1;
        }
        height
    }

    fn insert(&mut self, key: K) {
        let mut prev = vec![self.head.clone(); Self::MAX_HEIGHT];
        let mut x = self.find_greater_or_equal(&key, &mut prev);

        if x.is_some()
            && (self.compare)(&x.as_ref().unwrap().borrow().key, &key) == std::cmp::Ordering::Equal
        {
            return; // Key already exists
        }

        let height = Self::random_height();
        if height > self.max_height {
            for i in self.max_height..height {
                prev[i] = self.head.clone();
            }
            self.max_height = height;
        }

        let new_node = Rc::new(RefCell::new(Node::new(key, height)));
        for i in 0..height {
            new_node.borrow_mut().next[i] = prev[i].borrow_mut().next[i].take();
            prev[i].borrow_mut().next[i] = Some(new_node.clone());
        }
    }

    fn contains(&self, key: &K) -> bool {
        let x = self.find_greater_or_equal(key, &mut vec![self.head.clone(); Self::MAX_HEIGHT]);
        x.is_some()
            && (self.compare)(&x.as_ref().unwrap().borrow().key, key) == std::cmp::Ordering::Equal
    }

    fn find_greater_or_equal(
        &self,
        key: &K,
        prev: &mut Vec<Rc<RefCell<Node<K>>>>,
    ) -> Option<Rc<RefCell<Node<K>>>> {
        let mut x = self.head.clone();
        let mut level = self.max_height - 1;
        loop {
            let next = x.borrow().next[level].clone();
            if let Some(ref next_node) = next {
                if (self.compare)(&next_node.borrow().key, key) == std::cmp::Ordering::Less {
                    x = next_node.clone();
                } else {
                    prev[level] = x.clone();
                    if level == 0 {
                        return next;
                    } else {
                        level -= 1;
                    }
                }
            } else {
                prev[level] = x.clone();
                if level == 0 {
                    return None;
                } else {
                    level -= 1;
                }
            }
        }
    }
}
