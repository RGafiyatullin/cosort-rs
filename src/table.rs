use core::cmp::Ordering;

use crate::error::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct CoSortIndexEntry {
    pick_from: usize,
    move_to: usize,
}

pub trait CoSortTable: Sized {
    type PrimaryItem;
    type PrimaryColumn: AsRef<[Self::PrimaryItem]> + AsMut<[Self::PrimaryItem]>;

    fn add_column<'a, T>(self, column: &'a mut [T]) -> (Self, &'a mut [T]) {
        (self, column)
    }

    fn row_count(&self) -> Result<usize, Error>;

    fn primary_column(&self) -> &Self::PrimaryColumn;

    fn rows_swap(&mut self, a: usize, b: usize);

    fn cosort_unstable(&mut self, index: &mut [CoSortIndexEntry]) -> Result<(), Error>
    where
        Self::PrimaryItem: Ord,
    {
        self.cosort_unstable_by(index, Ord::cmp)
    }

    fn cosort_unstable_by<F>(
        &mut self,
        index: &mut [CoSortIndexEntry],
        compare: F,
    ) -> Result<(), Error>
    where
        F: Fn(&Self::PrimaryItem, &Self::PrimaryItem) -> Ordering,
    {
        let row_count = self.row_count()?;
        if index.len() < row_count {
            return Err(Error::IndexTooSmal);
        }

        let index = &mut index[0..row_count];
        (0..row_count).for_each(|idx| index[idx].pick_from = idx);

        let primary_column = self.primary_column();
        let primary_column = primary_column.as_ref();

        index.sort_unstable_by(|&left, &right| {
            compare(
                &primary_column[left.pick_from],
                &primary_column[right.pick_from],
            )
        });

        for i in 0..row_count {
            let p = index[i].pick_from;
            index[p].move_to = i;
        }

        for i in 0..row_count {
            while index[i].move_to != i {
                let j = index[i].move_to;
                index.swap(i, j);
                self.rows_swap(i, j);
            }
        }

        Ok(())
    }
}

impl<'a, T> CoSortTable for &'a mut [T] {
    type PrimaryItem = T;
    type PrimaryColumn = Self;

    fn row_count(&self) -> Result<usize, Error> {
        Ok(self.len())
    }

    fn primary_column(&self) -> &Self::PrimaryColumn {
        self
    }

    fn rows_swap(&mut self, a: usize, b: usize) {
        self.swap(a, b);
    }
}

impl<'a, L, T> CoSortTable for (L, &'a mut [T])
where
    L: CoSortTable,
{
    type PrimaryColumn = L::PrimaryColumn;
    type PrimaryItem = L::PrimaryItem;

    fn row_count(&self) -> Result<usize, Error> {
        let that_len = self.0.row_count()?;
        let this_len = self.1.len();

        if this_len == that_len {
            Ok(this_len)
        } else {
            Err(Error::ElementCountMismatch)
        }
    }

    fn primary_column(&self) -> &Self::PrimaryColumn {
        self.0.primary_column()
    }

    fn rows_swap(&mut self, a: usize, b: usize) {
        self.1.swap(a, b);
        self.0.rows_swap(a, b);
    }
}

// impl<'a, T, const SIZE: usize> CoSortTable for &'a mut [T; SIZE] {
//     type PrimaryItem = T;
//     type PrimaryColumn = Self;

//     fn row_count(&self) -> Result<usize, Error> {
//         Ok(self.len())
//     }

//     fn primary_column(&self) -> &Self::PrimaryColumn {
//         self
//     }

//     fn rows_swap(&mut self, a: usize, b: usize) {
//         #[cfg(test)]
//         println!("     {} swap {} <-> {}", core::any::type_name::<Self>(), a, b);
//         self.swap(a, b);
//     }
// }
