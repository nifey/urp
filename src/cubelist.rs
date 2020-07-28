use super::cube::Cube;

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
        unimplemented!();
    }

    /// Returns an empty CubeList
    pub fn new() -> Self {
        CubeList(vec![])
    }

    /// This function reads the cubelist from the PCN file
    /// that contains the information about the cubes in
    /// the following format
    ///
    /// * The first line contains the number of cubes in the cubelist
    /// * The second line contains N, the number of variable
    /// * The N following lines indicate for each variable the cubes
    /// it which the variable is present in. A negative value indicates
    /// that it is present as a negative literal
    pub fn read_from_file(file: &str) -> Result<Self, String> {
        unimplemented!();
    }

    /// Writes the boolean function represented by the CubeList into
    /// a file in the format specified in the read_from_file function
    pub fn write_to_file(&self, file: &str) {
        unimplemented!();
    }

    /// This function adds a Cube to the CubeList
    pub fn add_cube(&mut self, cube_x: Cube) {
        unimplemented!();
    }

    /// This function checks if the cubelist contains a particular cube
    pub fn contains_cube(&self, cube_x: &Cube) -> bool {
        unimplemented!();
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
        unimplemented!();
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
