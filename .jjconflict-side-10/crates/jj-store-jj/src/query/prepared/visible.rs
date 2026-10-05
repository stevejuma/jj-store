//! Exact visible JJ order with compact lexical ranks; the owner admits storage.
use super::super::{Error, Result};

pub(crate) struct Visible<Id> {
    order: Vec<Id>,
    sorted: Vec<u32>,
}

impl<Id: Copy + Ord> Visible<Id> {
    /// Caller reserves the ID vector and four bytes per inverse rank before growth.
    pub fn new(order: Vec<Id>, mut work: impl FnMut(usize) -> Result<()>) -> Result<Self> {
        if order.len() > u32::MAX as usize {
            return Err(Error::Limit);
        }
        work(order.len())?;
        let mut sorted = (0..order.len() as u32).collect::<Vec<_>>();
        // In-place heapsort checks cancellation and work every 1024 comparisons.
        fn sift<Id: Ord>(
            order: &[Id],
            rows: &mut [u32],
            mut root: usize,
            work: &mut impl FnMut(usize) -> Result<()>,
        ) -> Result<()> {
            while root < rows.len() / 2 {
                let mut child = root * 2 + 1;
                if child + 1 < rows.len() {
                    work(1)?;
                    if order[rows[child] as usize] < order[rows[child + 1] as usize] {
                        child += 1;
                    }
                }
                work(1)?;
                if order[rows[root] as usize] >= order[rows[child] as usize] {
                    break;
                }
                rows.swap(root, child);
                root = child;
            }
            Ok(())
        }
        let mut pending = 0;
        {
            let mut charge = |amount| {
                pending += amount;
                if pending >= 1024 {
                    work(pending)?;
                    pending = 0;
                }
                Ok(())
            };
            for root in (0..sorted.len() / 2).rev() {
                sift(&order, &mut sorted, root, &mut charge)?;
            }
            for end in (1..sorted.len()).rev() {
                sorted.swap(0, end);
                sift(&order, &mut sorted[..end], 0, &mut charge)?;
            }
        }
        work(pending)?;
        for pair in sorted.windows(2) {
            work(1)?;
            if order[pair[0] as usize] >= order[pair[1] as usize] {
                return Err(Error::Corrupt);
            }
        }
        Ok(Self { order, sorted })
    }
    fn rank(&self, id: &Id) -> Option<usize> {
        self.sorted
            .binary_search_by(|rank| self.order[*rank as usize].cmp(id))
            .ok()
            .map(|offset| self.sorted[offset] as usize)
    }
    pub fn contains(&self, id: &Id) -> bool {
        self.rank(id).is_some()
    }
    pub fn window(
        &self,
        after: Option<Id>,
        limit: usize,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<(&[Id], Option<Id>)> {
        if limit == 0 {
            return Err(Error::Limit);
        }
        work(self.order.len().max(1).ilog2() as usize + 2)?;
        let start = match after {
            Some(id) => self.rank(&id).ok_or(Error::Invalid)? + 1,
            None => 0,
        };
        let end = start.saturating_add(limit).min(self.order.len());
        work(end - start + 1)?;
        let next = (end < self.order.len()).then(|| self.order[end - 1]);
        Ok((&self.order[start..end], next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_visible_windows_seek_exact_order_under_fixed_work() {
        let order = (0..10_000u32).map(|n| n * 997 % 10_000).collect::<Vec<_>>();
        let visible = Visible::new(order.clone(), |_| Ok(())).unwrap();
        for offset in [0, 32, 9000, 9990, 9999] {
            for _ in 0..2 {
                let mut work = 0;
                let (ids, next) = visible
                    .window(Some(order[offset]), 8, |n| {
                        work += n;
                        if work > 32 { Err(Error::Limit) } else { Ok(()) }
                    })
                    .unwrap();
                let end = (offset + 9).min(order.len());
                assert_eq!(ids, &order[offset + 1..end]);
                assert_eq!(next, (end < order.len()).then(|| order[end - 1]));
                assert!(work <= 32);
            }
        }
        assert!(matches!(
            visible.window(Some(10_001), 1, |_| Ok(())),
            Err(Error::Invalid)
        ));
        assert!(visible.contains(&0));
        assert!(!visible.contains(&10_001));
    }

    #[test]
    fn visible_order_rejects_duplicates_work_exhaustion_and_zero_pages() {
        assert!(matches!(
            Visible::new(vec![2, 1, 2], |_| Ok(())),
            Err(Error::Corrupt)
        ));
        assert!(matches!(
            Visible::new(vec![2, 1], |_| Err(Error::Limit)),
            Err(Error::Limit)
        ));
        let empty = Visible::<u32>::new(vec![], |_| Ok(())).unwrap();
        assert_eq!(empty.window(None, 1, |_| Ok(())).unwrap(), (&[][..], None));
        assert!(matches!(
            empty.window(None, 0, |_| Ok(())),
            Err(Error::Limit)
        ));
    }
}
