use std::{
    cell::Cell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub enum MergeInsertNode {
    Root(Rc<Cell<(u64, u64)>>),
    Indirection(usize, usize),
}

#[derive(Debug, Default)]
pub struct MergeInsert {
    data: HashMap<(usize, usize), MergeInsertNode>,
}

impl MergeInsert {
    pub fn insert(&mut self, index: (usize, usize), value: (u64, u64)) {
        self.data
            .insert(index, MergeInsertNode::Root(Rc::new(Cell::new(value))));
    }

    pub fn merge(&mut self, li: (usize, usize), ri: (usize, usize)) {
        let Some(l) = self.data.get(&li).cloned() else {
            unreachable!("Should never be called with invalid l.")
        };
        let Some(r) = self.data.get(&ri).cloned() else {
            unreachable!("Should never be called with invalid r.")
        };

        match (l, r) {
            (MergeInsertNode::Root(l), MergeInsertNode::Root(r)) => {
                if !std::ptr::addr_eq(l.as_ptr(), r.as_ptr()) {
                    let r_old = r.get();
                    let l_old = l.get();
                    r.set((r_old.0 + l_old.0, r_old.1 + l_old.1));
                    self.data
                        .insert(li, MergeInsertNode::Indirection(ri.0, ri.1));
                }
            }
            (MergeInsertNode::Root(_), MergeInsertNode::Indirection(line, col)) => {
                self.merge(li, (line, col));
            }
            (MergeInsertNode::Indirection(line, col), MergeInsertNode::Root(_)) => {
                self.merge((line, col), ri);
            }
            (
                MergeInsertNode::Indirection(lline, lcol),
                MergeInsertNode::Indirection(rline, rcol),
            ) => self.merge((lline, lcol), (rline, rcol)),
        }
    }
}

impl Deref for MergeInsert {
    type Target = HashMap<(usize, usize), MergeInsertNode>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for MergeInsert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
