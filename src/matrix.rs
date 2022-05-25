use std::fmt;
use std::f32;

#[derive(Clone)]
pub struct Matrix{
    pub(in crate) matrix_array: Vec<Vec<f32>>,
}

pub enum CurveType{
    Bezier,
    Hermite
}

impl Matrix{
    pub fn new(row: usize, col: usize) -> Matrix{
        Matrix{matrix_array: vec![vec![0.0; col]; row],}
    }
    
    /// multiply m1 by the object that this is called on, modifying this object to be the product
    /// 
    /// m1 * this object -> this object
    pub fn multiply_matrixes(&mut self, m1: &Matrix){
        let mut matrix_result = Matrix::new(m1.matrix_array.len(), self.matrix_array[0].len());
        for result_i in 0..matrix_result.matrix_array.len(){
            for result_v in 0..matrix_result.matrix_array[result_i].len(){
                for m2_down_num in 0..self.matrix_array.len(){
                    matrix_result.matrix_array[result_i][result_v] += self.matrix_array[m2_down_num][result_v] * m1.matrix_array[result_i][m2_down_num];
                }
            }
        }
        
        *self = matrix_result;
    }
    
    pub fn identity() -> Matrix{
        let mut identity_matrix = Matrix::new(4,4);
        for i in 0..identity_matrix.matrix_array.len(){
            for v in 0..identity_matrix.matrix_array[0].len(){
                if i == v{
                    identity_matrix.matrix_array[i][v] = 1.0;
                }else{
                    identity_matrix.matrix_array[i][v] = 0.0;
                }
            }
        }
        return identity_matrix;
    }
    
    pub fn print_matrix(&self){
        println!("{}", self);
    }
    
    pub fn make_translate(x: i32, y: i32, z: i32) -> Matrix{
        let mut matrix = Matrix::identity();
        matrix.matrix_array[0][3] = x as f32;
        matrix.matrix_array[1][3] = y as f32;
        matrix.matrix_array[2][3] = z as f32;
        return matrix;
    }
    
    pub fn make_scale( x: f32, y: f32, z: f32) -> Matrix{
        let mut matrix = Matrix::identity();
        matrix.matrix_array[0][0] = x as f32;
        matrix.matrix_array[1][1] = y as f32;
        matrix.matrix_array[2][2] = z as f32;
        matrix.matrix_array[3][3] = 1.0;
        return matrix;
    }
    
    pub fn make_rot_x(mut theta: f32 ) -> Matrix{
        let mut matrix = Matrix::identity();
        theta = theta.to_radians();
        matrix.matrix_array[1][1] = theta.cos();
        matrix.matrix_array[1][2] = theta.sin() * -1.0;
        matrix.matrix_array[2][1] = matrix.matrix_array[1][2] * -1.0;
        matrix.matrix_array[2][2] = matrix.matrix_array[1][1];
        return matrix;
    }
    
    pub fn make_rot_y( mut theta: f32 ) -> Matrix{
        let mut matrix = Matrix::identity();
        theta = theta.to_radians();
        matrix.matrix_array[0][0] = theta.cos();
        matrix.matrix_array[0][2] = theta.sin();
        matrix.matrix_array[2][0] = matrix.matrix_array[0][2] * -1.0;
        matrix.matrix_array[2][2] = matrix.matrix_array[0][0];
        return matrix;
    }
    
    pub fn make_rot_z( mut theta: f32 ) -> Matrix{
        let mut matrix = Matrix::identity();
        theta = theta.to_radians();
        matrix.matrix_array[0][0] = theta.cos();
        matrix.matrix_array[0][1] = theta.sin() * -1.0;
        matrix.matrix_array[1][0] = matrix.matrix_array[0][1] * -1.0;
        matrix.matrix_array[1][1] = matrix.matrix_array[0][0];
        return matrix;
    }
    
    ///Returns: The correct 4x4 matrix that can be used
    ///to generate the coefiecients for a bezier curve
    pub fn make_bezier() -> Matrix{
        let mut matrix = Matrix::new(4,4);
        matrix.matrix_array[0][0] = -1.0;
        matrix.matrix_array[0][1] = 3.0;
        matrix.matrix_array[0][2] = -3.0;
        matrix.matrix_array[0][3] = 1.0;
        matrix.matrix_array[1][0] = 3.0;
        matrix.matrix_array[1][1] = -6.0;
        matrix.matrix_array[1][2] = 3.0;
        matrix.matrix_array[2][0] = -3.0;
        matrix.matrix_array[2][1] = 3.0;
        matrix.matrix_array[3][0] = 1.0;
        return matrix;
    }
    ///Returns: The correct 4x4 matrix that can be used
    ///to generate the coefiecients for a hermite curve
    pub fn make_hermite() -> Matrix{
        let mut matrix = Matrix::new(4,4);
        matrix.matrix_array[0][0] = 2.0;
        matrix.matrix_array[0][1] = -2.0;
        matrix.matrix_array[0][2] = 1.0;
        matrix.matrix_array[0][3] = 1.0;
        matrix.matrix_array[1][0] = -3.0;
        matrix.matrix_array[1][1] = 3.0;
        matrix.matrix_array[1][2] = -2.0;
        matrix.matrix_array[1][3] = -1.0;
        matrix.matrix_array[2][2] = 1.0;
        matrix.matrix_array[3][0] = 1.0;
        return matrix;
    }
    
    /// Inputs:   double p1
    /// 
    /// double p2
    /// 
    /// double p3
    /// 
    /// double p4
    /// 
    /// enum CurveType type
    /// 
    /// Returns:
    /// 
    /// A matrix containing the values for a, b, c and d of the
    /// equation at^3 + bt^2 + ct + d for the curve defined
    /// by p1, p2, p3 and p4.
    
    pub fn generate_curve_coefs( p0: f32, p1: f32, p2: f32, p3: f32, t: &CurveType ) -> Matrix{
        let mut matrix = Matrix::new(4,1);
        matrix.matrix_array[0][0] = p0;
        matrix.matrix_array[1][0] = p1;
        matrix.matrix_array[2][0] = p2; // r0 if hermite
        matrix.matrix_array[3][0] = p3; // r1 if hermite
        let curve_matrix: Matrix;
        match t{
            CurveType::Bezier=>{
                curve_matrix =  Matrix::make_bezier();
            }
            CurveType::Hermite=>{
                curve_matrix = Matrix::make_hermite();
            }
        }
        matrix.multiply_matrixes(&curve_matrix);
        return matrix;
    }
}

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let mut result: String = "".to_owned();
        for i in 0..self.matrix_array.len(){
            for v in 0..self.matrix_array[i].len(){
                result.push_str(&(format!("{} ",self.matrix_array[i][v]).to_string()));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}