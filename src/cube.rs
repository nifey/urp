use super::cubelist::CubeList;

/// Literal represents the state of a variable in a (product term) Cube
#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    /// The varibale is present as a positive literal
    Positive,
    /// The varibale is present as a negative literal
    Negative,
    /// The varibale is not present in the Cube (product term)
    Dontcare,
}

/// Cube represents a product term and can contain a number of variables
/// It is vector of Literals and stores the status of each variable in the product term
#[derive(PartialEq, Debug, Clone)]
pub struct Cube(Vec<Literal>);

impl Cube {
    /// Returns a new cube of length specified by num_var
    /// All the variables are in Dontcare state when returned
    pub fn new(num_var: usize) -> Self {
        Cube(vec![Literal::Dontcare; num_var])
    }

    /// Returns the length of the vector of Literal values
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the state of the variable in the Cube
    ///
    /// # Arguments
    /// * var_num - variable, whose value is needed
    pub fn get_literal(&self, var_num: usize) -> Result<Literal, String> {
        if var_num <= self.len() && var_num != 0 {
            Ok(self.0[var_num - 1].clone())
        } else {
            Err(format!("variable {} not present", var_num))
        }
    }

    /// Sets the state of the variable in the Cube
    ///
    /// # Arguments
    /// * var_num - variable, whose value is set
    /// * value - the Literal value to set
    ///
    /// Note: This function ignores if the variable number is
    /// wrong, i.e. 0 or greater than the length of the Cube
    pub fn set_literal(&mut self, var_num: usize, value: Literal) {
        if var_num <= self.len() && var_num != 0 {
            self.0[var_num - 1] = value;
        }
    }

    /// Returns the complement of the Cube as a CubeList
    pub fn complement(&self) -> CubeList {
        let mut cubelist = CubeList::new(self.0.len());
        if *self != Cube::new(self.len()) {
            for i in 0..self.0.len() {
                if self.0[i] == Literal::Positive {
                    cubelist.add_cube(Cube::get_var_cube(self.len(), i + 1, false));
                } else if self.0[i] == Literal::Negative {
                    cubelist.add_cube(Cube::get_var_cube(self.len(), i + 1, true));
                }
            }
        }
        cubelist
    }

    /// Returns a cube that contains only one literal
    pub fn get_var_cube(num_var: usize, var_num: usize, positive: bool) -> Self {
        let mut cube = Cube::new(num_var);
        if positive {
            cube.set_literal(var_num, Literal::Positive);
        } else {
            cube.set_literal(var_num, Literal::Negative);
        }
        cube
    }

    /// Returns self AND cube_x
    ///
    /// If result of AND operatin is 0, it returns None
    pub fn and(&self, cube_x: &Cube) -> Option<Cube> {
        let mut result = Cube::new(self.len());
        for i in 1..=self.len() {
            if self.get_literal(i).unwrap() == Literal::Dontcare {
                result.set_literal(i, cube_x.get_literal(i).unwrap());
            } else if cube_x.get_literal(i).unwrap() == Literal::Dontcare {
                result.set_literal(i, self.get_literal(i).unwrap());
            } else if self.get_literal(i).unwrap() == cube_x.get_literal(i).unwrap() {
                result.set_literal(i, self.get_literal(i).unwrap());
            } else {
                return None;
            }
        }
        Some(result)
    }
}

/// This implements conversion from a vector of i32 to a Cube
///
/// A positive value indicates that the corresponding variable is present as a positive literal
/// A negative value indicates that the corresponding variable is present as a negative literal
/// A zero indicates that the corresponding variable is not present in the Cube
///
/// ```
/// let cube = Cube::from(vec![1, 0, -1]);
/// ```
impl From<Vec<i32>> for Cube {
    fn from(vector: Vec<i32>) -> Self {
        let mut cube = Cube::new(vector.len());
        for i in 0..vector.len() {
            if vector[i] > 0 {
                cube.set_literal(i + 1, Literal::Positive);
            } else if vector[i] < 0 {
                cube.set_literal(i + 1, Literal::Negative);
            } else {
                cube.set_literal(i + 1, Literal::Dontcare);
            }
        }
        cube
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut non_dont_care_count = 0;
        for value in &self.0 {
            if *value != Literal::Dontcare {
                non_dont_care_count += 1;
            }
        }
        write!(f, "{} ", non_dont_care_count);
        for i in 0..self.0.len() {
            if self.0[i] == Literal::Positive {
                write!(f, "{} ", (i + 1));
            } else if self.0[i] == Literal::Negative {
                write!(f, "{} ", -((i + 1) as i32));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_cube() {
        let cube = Cube::new(3);
        assert_eq!(cube.len(), 3);
        assert_eq!(cube.get_literal(1).unwrap(), Literal::Dontcare);
        assert_eq!(cube.get_literal(2).unwrap(), Literal::Dontcare);
        assert_eq!(cube.get_literal(3).unwrap(), Literal::Dontcare);
        assert_eq!(cube.get_literal(0).is_err(), true);
        assert_eq!(cube.get_literal(4).is_err(), true);
    }

    #[test]
    fn set_variable() {
        let mut cube = Cube::new(3);
        cube.set_literal(2, Literal::Positive);
        assert_eq!(cube.get_literal(2).unwrap(), Literal::Positive);
    }

    #[test]
    fn from_vector() {
        let cube = Cube::from(vec![0, -1, 1]);
        assert_eq!(cube.len(), 3);
        assert_eq!(cube.get_literal(1).unwrap(), Literal::Dontcare);
        assert_eq!(cube.get_literal(2).unwrap(), Literal::Negative);
        assert_eq!(cube.get_literal(3).unwrap(), Literal::Positive);
    }

    #[test]
    fn cube_equal() {
        let mut cube_x = Cube::from(vec![1, 0, -1]);
        let mut cube_y = Cube::from(vec![1, 0, -1]);
        assert_eq!(cube_x, cube_y);
        cube_x.set_literal(2, Literal::Negative);
        assert_ne!(cube_x, cube_y);
    }

    #[test]
    fn complement() {
        // Complement of 1 is 0 and so it returns an empty CubeList
        assert_eq!(Cube::new(3).complement().len(), 0);

        // Complement of a single variable
        let cube_x = Cube::from(vec![1, 0]);
        let cube_y = Cube::from(vec![-1, 0]);
        assert_eq!(cube_x.complement().len(), 1);
        assert_eq!(cube_x.complement().contains_cube(&cube_y), true);

        // Complement of a cube with multiple variables
        let cube_z = Cube::from(vec![1, 0, -1]);
        let cube_a = Cube::from(vec![-1, 0, 0]);
        let cube_b = Cube::from(vec![0, 0, 1]);
        println!("{:?}", cube_z.complement());
        assert_eq!(cube_z.complement().len(), 2);
        assert_eq!(cube_z.complement().contains_cube(&cube_a), true);
        assert_eq!(cube_z.complement().contains_cube(&cube_b), true);
    }

    #[test]
    fn and() {
        // Normal AND of two Cubes
        let mut cube_x = Cube::from(vec![1, 0, 0]);
        let mut cube_y = Cube::from(vec![0, 0, -1]);
        let mut cube_z = Cube::from(vec![1, 0, -1]);
        assert_eq!(cube_x.and(&cube_y).unwrap(), cube_z);

        // AND of complementing literals
        cube_y.set_literal(3, Literal::Positive);
        assert_eq!(cube_z.and(&cube_y).is_none(), true);
    }
}
