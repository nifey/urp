use super::cube::{Cube, Literal};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

/// CubeList represents a Boolean function.
/// It is a vector of Cubes. Each Cube represents a product term and
/// the function is obtained by summing (performing logical OR of) all
/// the product terms
#[derive(Debug)]
pub struct CubeList(Vec<Cube>);

impl CubeList {
    /// This function returns the number of Cube or Product terms present
    /// in the CubeList or Boolean function
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an empty CubeList
    pub fn new() -> Self {
        CubeList(vec![])
    }

    /// This function reads the cubelist from the PCN file
    /// that contains the information about the cubes in
    /// the following format
    ///
    /// * The first line contains the number of variable
    /// * The second line contains N, the number of cubes in the cubelist
    /// * Each of the N following lines shows which variables are present
    /// in each cube. First number is the number of Non Dont care variables
    /// in th cube followed by the variable numbers. A positive number indicates
    /// that it is present as a positive literal and a negative number indicates
    /// that it is present as a negative literal
    pub fn read_from_file(file: &str) -> Self {
        let mut reader = BufReader::new(File::open(file).expect("File could not be read"));
        let mut buffer = String::new();
        reader.read_line(&mut buffer);
        let num_var = buffer
            .trim()
            .parse::<u32>()
            .expect("Number of variables is invalid");
        buffer.clear();
        reader.read_line(&mut buffer);
        let num_cubes = buffer
            .trim()
            .parse::<u32>()
            .expect("Number of cubes is invalid");
        let mut vectors = Vec::new();
        for _ in 0..num_cubes {
            let mut cube_vector: Vec<i32> = vec![0; num_var as usize];
            buffer.clear();
            reader.read_line(&mut buffer);
            for var in buffer.trim().split_whitespace().skip(1) {
                let var_num = var
                    .trim()
                    .parse::<i32>()
                    .expect("Expected an integer value");
                if var_num < 0 {
                    cube_vector[(var_num.abs() - 1) as usize] = -1;
                } else if var_num > 0 {
                    cube_vector[(var_num - 1) as usize] = 1;
                }
            }
            vectors.push(cube_vector);
        }
        CubeList::from(vectors)
    }

    /// Writes the boolean function represented by the CubeList into
    /// a file in the format specified in the read_from_file function
    pub fn write_to_file(&self, file: &str) {
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)
            .expect("Cannot open file for writing");
        write!(output_file, "{}", self);
    }

    /// This function adds a Cube to the CubeList
    pub fn add_cube(&mut self, cube_x: Cube) {
        self.0.push(cube_x);
    }

    /// This function checks if the cubelist contains a particular cube
    pub fn contains_cube(&self, cube_x: &Cube) -> bool {
        self.0.contains(cube_x)
    }

    /// This funcitons performs Logical AND of the boolean function
    /// with another boolean function represented as a CubeList
    pub fn and(&self, cubelist_x: &CubeList) -> CubeList {
        unimplemented!();
    }

    /// This funcitons performs Logical OR of the boolean function
    /// with another boolean function represented as a CubeList
    pub fn or(&self, cubelist_x: &CubeList) -> CubeList {
        unimplemented!();
    }

    /// This funcitons returns the complement of the boolean function
    pub fn complement(&self) -> CubeList {
        unimplemented!();
    }

    /// This function returns the Shannon Cofactor with respect to variable
    /// indicated by var_num. It returns both the positive and negative cofactor
    /// as a tuple
    pub fn cofactor(&self, var_num: usize) -> (CubeList, CubeList) {
        let mut pos_cofactor = CubeList::new();
        let mut neg_cofactor = CubeList::new();
        for cube in &self.0 {
            if cube.get_literal(var_num).unwrap() == Literal::Positive {
                // Add the cube to positive cofactor
                let mut new_cube = cube.clone();
                new_cube.set_literal(var_num, Literal::Dontcare);
                pos_cofactor.add_cube(new_cube);
            } else if cube.get_literal(var_num).unwrap() == Literal::Negative {
                // Add the cube to negative cofactor
                let mut new_cube = cube.clone();
                new_cube.set_literal(var_num, Literal::Dontcare);
                neg_cofactor.add_cube(new_cube);
            } else {
                // Add the cube to both cofactors
                pos_cofactor.add_cube(cube.clone());
                neg_cofactor.add_cube(cube.clone());
            }
        }
        (pos_cofactor, neg_cofactor)
    }

    /// This function returns a boolean value that indicates if the function
    /// represented by the CubeList is a tautology
    pub fn is_tautology(&self) -> bool {
        unimplemented!();
    }

    /// This function returns a boolean value that indicates if the function
    /// represented by the CubeList is a unate function
    pub fn is_unate(&self) -> bool {
        unimplemented!();
    }
}

/// This implements conversion from a vector of vectors of i32 to a CubeList
///
/// Each vector present in the vector represents a cube
///
impl From<Vec<Vec<i32>>> for CubeList {
    fn from(vector: Vec<Vec<i32>>) -> Self {
        let mut cubelist = CubeList::new();
        for i in 0..vector.len() {
            cubelist.add_cube(Cube::from(vector[i].clone()));
        }
        cubelist
    }
}

impl std::fmt::Display for CubeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.0.len() > 0 {
            writeln!(f, "{}", self.0[0].len());
            writeln!(f, "{}", self.0.len());
            for cube in &self.0 {
                writeln!(f, "{}", cube);
            }
        }
        Ok(())
    }
}

mod test {
    use super::*;

    #[test]
    fn cubelist_from_vec_vec_i32_and_contains_cube() {
        let cubelist = CubeList::from(vec![vec![1, 0, -1], vec![1, 1, 0], vec![0, 0, 1]]);
        assert_eq!(cubelist.len(), 3);
        assert_eq!(cubelist.contains_cube(&Cube::from(vec![1, 0, -1])), true);
        assert_eq!(cubelist.contains_cube(&Cube::from(vec![1, 1, 0])), true);
        assert_eq!(cubelist.contains_cube(&Cube::from(vec![0, 0, 1])), true);
    }

    #[test]
    fn cofactor() {
        let cubelist = CubeList::from(vec![vec![1, -1, -1], vec![-1, 1, -1], vec![0, 1, 1]]);
        let (pos_cubelist, neg_cubelist) = cubelist.cofactor(1);
        assert_eq!(pos_cubelist.len(), 2);
        assert_eq!(
            pos_cubelist.contains_cube(&Cube::from(vec![0, -1, -1])),
            true
        );
        assert_eq!(pos_cubelist.contains_cube(&Cube::from(vec![0, 1, 1])), true);
        assert_eq!(neg_cubelist.len(), 2);
        assert_eq!(
            neg_cubelist.contains_cube(&Cube::from(vec![0, 1, -1])),
            true
        );
        assert_eq!(neg_cubelist.contains_cube(&Cube::from(vec![0, 1, 1])), true);
    }

    #[test]
    fn and() {}

    #[test]
    fn or() {}

    #[test]
    fn complement() {}

    #[test]
    fn is_unate() {}

    #[test]
    fn is_tautology() {
        let mut cubelist1 = CubeList::from(vec![vec![0, 1, 1], vec![1, 0, 0]]);
        assert_eq!(cubelist1.is_tautology(), false);
        cubelist1.add_cube(Cube::from(vec![0, 0, 0]));
        assert_eq!(cubelist1.is_tautology(), true);

        let mut cubelist2 = CubeList::from(vec![vec![1, 0, 0]]);
        assert_eq!(cubelist2.is_tautology(), false);
        cubelist2.add_cube(Cube::from(vec![-1, 0, 0]));
        assert_eq!(cubelist2.is_tautology(), true);
    }
}
