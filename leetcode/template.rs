#[cfg(test)]
mod tests {
    // wrap complex expr in ()
    macro_rules! to_vec {
        ([ [$($x:tt),* $(,)?] $(,)?] $(,)?) => {
            vec![vec![ $(to_vec!($x)),* ]]
        };
        ([ [$($x:tt),* $(,)?], $($y:tt),+ $(,)?] $(,)?) => {
            {
                let mut x = vec![vec![$(to_vec!($x)),* ]];
                x.extend([$(to_vec!($y)),+]);
                x
            }
        };
        ([ $($x:expr),* $(,)?] $(,)?) => {
            vec![ $(to_vec!($x)),* ]
        };
        ($x:expr) => {
            $x
        }
    }

    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}
