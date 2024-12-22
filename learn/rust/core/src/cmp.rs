use core::cmp;
pub trait Eq: cmp::Eq {
    fn eq(&self, other: &Self) -> bool {
        cmp::PartialEq::eq(self, other)
    }
}
pub trait Ord: cmp::Ord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        cmp::Ord::cmp(self, other)
    }
}
