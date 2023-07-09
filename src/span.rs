use std::ops::Bound;
use std::ops::RangeBounds;

#[derive(Copy, Clone, Debug)]
pub struct Span {
    begin: usize,
    end: usize,
}

impl Span {
    pub fn new<R: RangeBounds<usize>>(range: R) -> Self {
        Self {
            begin: match range.start_bound() {
                Bound::Included(i) => *i,
                Bound::Excluded(i) => i.checked_sub(1).expect("Cannot exclude <= 0"),
                Bound::Unbounded => 0,
            },
            end: match range.end_bound() {
                Bound::Included(i) => *i + 1,
                Bound::Excluded(i) => *i,
                Bound::Unbounded => panic!("Cannot know max bound"),
            },
        }
    }

    pub fn point(point: usize) -> Self {
        Self {
            begin: point,
            end: point,
        }
    }

    pub fn begin(&self) -> usize {
        self.begin
    }

    pub fn end(&self) -> usize {
        self.end
    }
}
