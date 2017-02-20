
#[derive(Debug, PartialEq)]
pub enum DimError {
    IndexMismatch,
    IndexOutOfRange,
}

#[derive(Debug, Clone)]
pub struct MultiDim<T: Clone> {
    dimensions: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone> MultiDim<T> {
    pub fn new(dim: &[usize], init: T) -> Self {
        let vect_dim: Vec<usize> = dim.iter().cloned().map(|x| x + 1).collect();
        let length: usize = vect_dim.iter().product();

        if dim.iter().min() == Some(&0) {
            panic!("Cannot have a 0-length dimension");
        }

        // println!("MultiDim new: {:?}", dim);
        MultiDim {
            dimensions: vect_dim,
            data: vec![init; length],
        }
    }

    fn out_of_range(&self, dim: &[usize]) -> bool {
        let iter = dim.iter().zip(self.dimensions.iter());
        for (a, b) in iter {
            if a > b {
                return true;
            }
        }
        false
    }

    fn check(&self, dim: &[usize]) -> Result<(), DimError> {
        if self.dimensions.len() != dim.len() {
            return Err(DimError::IndexMismatch);
        }

        if self.out_of_range(dim) {
            return Err(DimError::IndexOutOfRange);
        }

        Ok(())
    }

    fn index(&self, dim: &[usize]) -> usize {
        let mut n = 0;
        let mut mult = 1;
        for i in 0..dim.len() {
            n = n + (dim[i] * mult);
            mult = mult * self.dimensions[i];
        }
        n
    }

    pub fn set(&mut self, dim: &[usize], val: T) -> Result<(), DimError> {
        match self.check(dim) {
            Ok(_) => {
                let i = self.index(dim);
                self.data[i] = val;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get(&self, dim: &[usize]) -> Result<&T, DimError> {
        // println!("MultiDim get {:?}", dim);
        match self.check(dim) {
            Ok(_) => {
                let i = self.index(dim);
                Ok(&self.data[i])
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut md = MultiDim::<u32>::new(&[10, 10, 10], 0);
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    md.set(&[i, j, k], (i + (j * 11) + (k * 121)) as u32).unwrap();
                }
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    match md.get(&[i, j, k]) {
                        Ok(n) => assert_eq!(*n, (i + (j * 11) + (k * 121)) as u32),
                        Err(e) => panic!("Got error: {:?}", e),
                    }
                }
            }
        }
    }

    #[test]
    fn string_value() {
        let mut md = MultiDim::<String>::new(&[2, 2], String::from(""));
        md.set(&[0, 1], String::from("hello world")).unwrap();
        assert_eq!("", *md.get(&[0, 0]).unwrap());
        assert_eq!("hello world", *md.get(&[0, 1]).unwrap());
    }

    #[test]
    fn index_errors() {
        let mut md = MultiDim::<u32>::new(&[2, 2], 0);
        match md.set(&[5, 5], 0) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, DimError::IndexOutOfRange),
        }

        match md.set(&[0, 0, 2], 0) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, DimError::IndexMismatch),
        }

        match md.get(&[5, 5]) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, DimError::IndexOutOfRange),
        }

        match md.get(&[0, 0, 2]) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, DimError::IndexMismatch),
        }
    }

    #[test]
    #[should_panic]
    fn construct_failure() {
        MultiDim::<u32>::new(&[2, 0], 0);
    }
}
