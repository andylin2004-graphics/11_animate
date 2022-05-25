use crate::ReflectionValue;
use crate::Color;
use crate::CurveType;
use crate::Image;
use crate::Matrix;
use crate::gmath::*;
use std::f32;
use rand::Rng;

impl Image {
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut z0: f32, mut x1: i32, mut y1: i32, mut z1: f32, color: &Color) {
        // println!("x0: {}, y0: {}, x1: {}, y1: {}", x0, y0, x1, y1);
        if (x0 >= 500 &&  x1 >= 500) || (y0 >= 500 && y1 >= 500) || (x0 < 0 &&  x1 < 0) || (y0 < 0 && y1 < 0){
            return;
        }
        if x0 > x1 {
            let mut tmp = x0;
            x0 = x1;
            x1 = tmp;
            tmp = y0;
            y0 = y1;
            y1 = tmp;
            let tmp = z0;
            z0 = z1;
            z1 = tmp;
        }
        let slope: f32 = (y1 - y0) as f32 / (x1 - x0) as f32;
        if slope > 1.0 {
            // octant 2
            let mut x = x0;
            let mut y = y0;
            let mut z = z0;
            let z_rate = (z1 - z0) / (y1 - y0) as f32;
            let a = 2 * (y1 - y0);
            let b = -2 * (x1 - x0);
            let mut d = 1 / 2 * a + b; // emphasis on controlling y
            while y <= y1 {
                if self.plot(x, y, z, color){
                    if d < 0 {
                        // as b dominates a, and we need to hit 0
                        x += 1;
                        d += a;
                    }
                    y += 1;
                    z += z_rate;
                    d += b;
                }else{
                    break;
                }
            }
        } else if slope >= 0.0 {
            // octant 1
            let mut x = x0;
            let mut y = y0;
            let mut z = z0;
            let z_rate = (z1 - z0) / (x1 - x0) as f32;
            let a = 2 * (y1 - y0);
            let b = -2 * (x1 - x0);
            let mut d = a + 1 / 2 * b; // emphasis on controlling x
            while x <= x1 {
                if self.plot(x, y, z, color){
                    if d > 0 {
                        // as a dominates b, and we need to hit 0
                        y += 1;
                        d += b;
                    }
                    x += 1;
                    z += z_rate;
                    d += a;
                }else{
                    break;
                }
            }
        } else if slope < -1.0 {
            // octant 7
            let mut x = x0;
            let mut y = y0;
            let mut z = z0;
            let z_rate = (z1 - z0) / (y1 - y0) as f32;
            let a = 2 * (y1 - y0); // since this is negative, you dont need to make the next part negative
            let b = 2 * (x1 - x0);
            let mut d = 1 / 2 * a + b; // emphasis on controlling x
            while y >= y1 {
                if self.plot(x, y, z, color){
                    if d < 0 {
                        // as a dominates b, and we need to hit 0
                        x += 1;
                        d -= a; // basically adding
                    }
                    y -= 1;
                    z += z_rate;
                    d -= b; // basically adding
                }else{
                    break;
                }
            }
        } else {
            // octant 8
            let mut x = x0;
            let mut y = y0;
            let mut z = z0;
            let z_rate = (z1 - z0) / (x1 - x0) as f32;
            let a = 2 * (y1 - y0); // since this is negative, you dont need to make the next part negative
            let b = 2 * (x1 - x0);
            let mut d = a + 1 / 2 * b; // emphasis on controlling y
            while x <= x1 {
                if self.plot(x, y, z, color){
                    if d > 0 {
                        // as b dominates a, and we need to hit 0
                        y -= 1;
                        d -= b; // basically adding
                    }
                    x += 1;
                    z += z_rate;
                    d -= a; // basically adding
                }else{
                    break;
                }
            }
        }
    }

    pub fn draw_lines(&mut self, matrix: &Matrix, color: &Color) {
        for i in (0..matrix.matrix_array[0].len()).step_by(2) {
            self.draw_line(
                matrix.matrix_array[0][i] as i32,
                matrix.matrix_array[1][i] as i32,
                matrix.matrix_array[2][i] as f32,
                matrix.matrix_array[0][i + 1] as i32,
                matrix.matrix_array[1][i + 1] as i32,
                matrix.matrix_array[2][i + 1] as f32,
                color,
            );
        }
    }

    ///======== void draw_polygons() ==========
    ///
    ///Inputs:   struct matrix *polygons
    ///
    ///screen s
    ///
    ///color c
    ///
    ///Returns:
    ///
    ///Goes through polygons 3 points at a time, drawing
    ///lines connecting each points to create bounding triangles
    ///====================
    pub fn draw_polygons(&mut self, polygons: &Matrix, c: &Color, view: &mut Vec<f32>, ambient_color: &Color, point_light_vector: &mut Vec<f32>, point_light_color: &Color, ambient_reflect: &ReflectionValue, direct_reflect: &ReflectionValue, specular_reflect: &ReflectionValue) {
        for i in (0..polygons.matrix_array[0].len()).step_by(3) {
            let normal = &mut polygons.calculate_normal(i);
            if normal[2] > 0.0 {
                // self.draw_line(
                //     polygons.matrix_array[0][i] as i32,
                //     polygons.matrix_array[1][i] as i32,
                //     polygons.matrix_array[2][i] as f32,
                //     polygons.matrix_array[0][i + 1] as i32,
                //     polygons.matrix_array[1][i + 1] as i32,
                //     polygons.matrix_array[2][i + 1] as f32,
                //     c,
                // );
                // self.draw_line(
                //     polygons.matrix_array[0][i + 1] as i32,
                //     polygons.matrix_array[1][i + 1] as i32,
                //     polygons.matrix_array[2][i + 1] as f32,
                //     polygons.matrix_array[0][i + 2] as i32,
                //     polygons.matrix_array[1][i + 2] as i32,
                //     polygons.matrix_array[2][i + 2] as f32,
                //     c,
                // );
                // self.draw_line(
                //     polygons.matrix_array[0][i + 2] as i32,
                //     polygons.matrix_array[1][i + 2] as i32,
                //     polygons.matrix_array[2][i + 2] as f32,
                //     polygons.matrix_array[0][i] as i32,
                //     polygons.matrix_array[1][i] as i32,
                //     polygons.matrix_array[2][i] as f32,
                //     c,
                // );
                let color = &get_lighting(normal, view, ambient_color, point_light_color, point_light_vector, ambient_reflect, direct_reflect, specular_reflect);
                self.scanline_convert(
                    polygons.matrix_array[0][i],
                    polygons.matrix_array[1][i],
                    polygons.matrix_array[2][i],
                    polygons.matrix_array[0][i + 1],
                    polygons.matrix_array[1][i + 1],
                    polygons.matrix_array[2][i + 1],
                    polygons.matrix_array[0][i + 2],
                    polygons.matrix_array[1][i + 2],
                    polygons.matrix_array[2][i + 2],
                    color
                )
            }
        }
    }

    /*======== void scanline_convert() ==========
    Inputs: x0 y0 z0 x1 y1 z1 x2 y2 z2: i32
            self screen
    Returns:

    Fills in polygon i by drawing consecutive horizontal (or vertical) lines.

    Color should be set differently for each polygon.
    ====================*/
    fn scanline_convert(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, color: &Color) {
        let mut polygons = [(x0, y0, z0), (x1, y1, z1), (x2, y2, z2)];
        polygons.sort_by_key(|k| (k.1 as i32, k.0 as i32, k.2 as i32));
        // println!("{:?}", polygons);
        let mut past_midpoint = false;
        let mut x0 = polygons[0].0;
        let mut x1 = polygons[0].0;
        let mut z0 = polygons[0].2;
        let mut z1 = polygons[0].2;
        let d0 = (polygons[2].1 as i32 - polygons[0].1 as i32) + 1;
        let d1 = (polygons[1].1 as i32 - polygons[0].1 as i32) + 1;
        let d2 = (polygons[2].1 as i32 - polygons[1].1 as i32) + 1;
        let dx0 = if d0 > 0 {(polygons[2].0 - polygons[0].0) / d0 as f32} else {0.0};
        let dz0 = if d0 > 0 {(polygons[2].2 - polygons[0].2) / d0 as f32} else {0.0};
        let mut dx1 = if d1 > 0 {(polygons[1].0 - polygons[0].0) / d1 as f32} else {0.0};
        let mut dz1 = if d1 > 0 {(polygons[1].2 - polygons[0].2) / d1 as f32} else {0.0};
        let dx1_1 = if d2 > 0 {(polygons[2].0 - polygons[1].0) / d2 as f32} else {0.0};
        let dz1_1 = if d2 > 0 {(polygons[2].2 - polygons[1].2) / d2 as f32} else {0.0};
        if (polygons[2].1 - polygons[1].1) as i32 == 0{
            past_midpoint = true
        }
        // for y in y0 as i32..=y0 as i32+1 {
        for y in polygons[0].1 as i32..=polygons[2].1 as i32{
            if y >= polygons[1].1 as i32 && !past_midpoint{
                dx1 = dx1_1;
                dz1 = dz1_1;
                x1 = polygons[1].0;
                z1 = polygons[1].2;
                past_midpoint = true;
            }
            self.draw_line(x0 as i32, y, z0, x1 as i32, y, z1, color);
            x0 += dx0;
            x1 += dx1;
            z0 += dz0;
            z1 += dz1;
        }
    }
}

impl Matrix {
    pub fn add_edge(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
        if self.matrix_array.len() < 4 {
            *self = Matrix::new(4, 0);
        }
        self.add_point(x0, y0, z0);
        self.add_point(x1, y1, z1);
    }

    pub fn add_edge_int(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
        if self.matrix_array.len() < 4 {
            *self = Matrix::new(4, 0);
        }
        self.add_point(x0 as f32, y0 as f32, z0 as f32);
        self.add_point(x1 as f32, y1 as f32, z1 as f32);
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32) {
        if self.matrix_array.len() < 4 {
            *self = Matrix::new(4, 0);
        }
        self.matrix_array[0].push(x);
        self.matrix_array[1].push(y);
        self.matrix_array[2].push(z);
        self.matrix_array[3].push(1.0);
    }

    pub fn add_circle(&mut self, cx: f32, cy: f32, cz: f32, r: f32, step: i32) {
        let mut prev_x = r + cx;
        let mut prev_y = cy;
        for t in 0..step + 1 {
            let x = r * (2.0 * f32::consts::PI * (t as f32 / step as f32)).cos() + cx;
            let y = r * (2.0 * f32::consts::PI * (t as f32 / step as f32)).sin() + cy;
            self.add_edge(prev_x, prev_y, cz, x, y, cz);
            prev_x = x;
            prev_y = y;
        }
    }

    /// x2, y2, x3, y3 are rx0, ry0, rx1, ry1 respectively if hermier
    pub fn add_curve(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        step: i32,
        curve_type: &CurveType,
    ) {
        let matrix_x = Matrix::generate_curve_coefs(x0, x1, x2, x3, curve_type);
        let matrix_y = Matrix::generate_curve_coefs(y0, y1, y2, y3, curve_type);
        let mut prev_x = x0;
        let mut prev_y = y0;
        for t in 0..step + 1 {
            let x = (matrix_x.matrix_array[0][0] * (t as f32 / step as f32).powi(3))
                + (matrix_x.matrix_array[1][0] * (t as f32 / step as f32).powi(2))
                + (matrix_x.matrix_array[2][0] * t as f32 / step as f32)
                + matrix_x.matrix_array[3][0];
            let y = (matrix_y.matrix_array[0][0] * (t as f32 / step as f32).powi(3))
                + (matrix_y.matrix_array[1][0] * (t as f32 / step as f32).powi(2))
                + (matrix_y.matrix_array[2][0] * t as f32 / step as f32)
                + matrix_y.matrix_array[3][0];
            self.add_edge(prev_x, prev_y, 0.0, x, y, 0.0);
            prev_x = x;
            prev_y = y;
        }
    }

    /// add_box()
    /// Inputs:   matrix * edges
    ///
    ///             double x
    ///
    ///             double y
    ///
    ///             double z
    ///
    ///             double width
    ///
    ///             double height
    ///
    ///             double depth

    /// add the points for a rectagular prism whose
    /// upper-left-front corner is (x, y, z) with width,
    /// height and depth dimensions.
    pub fn add_box(&mut self, x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32) {
        // front
        self.add_polygon(x + width, y - height, z, x + width, y, z, x, y, z);
        self.add_polygon(x + width, y - height, z, x, y, z, x, y - height, z);
        // right
        self.add_polygon(
            x + width,
            y,
            z,
            x + width,
            y - height,
            z - depth,
            x + width,
            y,
            z - depth,
        );
        self.add_polygon(
            x + width,
            y - height,
            z,
            x + width,
            y - height,
            z - depth,
            x + width,
            y,
            z,
        );
        // back
        self.add_polygon(
            x + width,
            y - height,
            z - depth,
            x,
            y - height,
            z - depth,
            x + width,
            y,
            z - depth,
        );
        self.add_polygon(
            x,
            y - height,
            z - depth,
            x,
            y,
            z - depth,
            x + width,
            y,
            z - depth,
        );
        // left
        self.add_polygon(x, y - height, z - depth, x, y, z, x, y, z - depth);
        self.add_polygon(x, y - height, z - depth, x, y - height, z, x, y, z);
        // top
        self.add_polygon(x, y, z - depth, x, y, z, x + width, y, z);
        self.add_polygon(x + width, y, z, x + width, y, z - depth, x, y, z - depth);
        // bottom
        self.add_polygon(
            x,
            y - height,
            z,
            x,
            y - height,
            z - depth,
            x + width,
            y - height,
            z - depth,
        );
        self.add_polygon(
            x,
            y - height,
            z,
            x + width,
            y - height,
            z - depth,
            x + width,
            y - height,
            z,
        );
    }

    /// add_sphere()
    /// Inputs:   struct matrix * points
    /// double cx
    /// double cy
    /// double cz
    /// double r
    /// int step  

    /// adds all the points for a sphere with center (cx, cy, cz)
    /// and radius r using step points per circle/semicircle.

    /// Since edges are drawn using 2 points, add each point twice,
    /// or add each point and then another point 1 pixel away.

    /// should call generate_sphere to create the necessary points
    pub fn add_sphere(&mut self, cx: f32, cy: f32, cz: f32, r: f32, step: i32) {
        let lat_start: usize = 0;
        let lat_stop = step as usize;
        let long_start: usize = 0;
        let long_stop = step as usize;
        let points_matrix = Matrix::generate_sphere(cx, cy, cz, r, step);
        for lat in lat_start..lat_stop + 1 {
            for longt in long_start..long_stop + 1 {
                let index = lat * step as usize + longt;
                self.add_polygon(
                    points_matrix.matrix_array[0][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[0]
                        [(index + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1]
                        [(index + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2]
                        [(index + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[0]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                );
                self.add_polygon(
                    points_matrix.matrix_array[0][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2][index % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[0]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2]
                        [(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[0]
                        [(index + step as usize) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1]
                        [(index + step as usize) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2]
                        [(index + step as usize) % points_matrix.matrix_array[0].len()],
                );
            }
        }
    }

    /// generate_sphere()
    /// Inputs:   struct matrix * points
    ///         double cx
    ///         double cy
    ///         double cz
    ///         double r
    ///         int step
    ///
    /// Returns: Generates all the points along the surface
    ///         of a sphere with center (cx, cy, cz) and
    ///         radius r using step points per circle/semicircle.
    ///         Returns a matrix of those points
    pub fn generate_sphere(cx: f32, cy: f32, cz: f32, r: f32, step: i32) -> Matrix {
        let mut matrix = Matrix::new(0, 0);
        let rot_start = 0;
        let rot_stop = step;
        let circ_start = 0;
        let circ_stop = step;
        for rot_t in rot_start..rot_stop + 1 {
            for cir_t in circ_start..circ_stop + 1{
                let x = r * (f32::consts::PI * (cir_t as f32 / step as f32)).cos() + cx;
                let y = r
                    * (f32::consts::PI * (cir_t as f32 / step as f32)).sin()
                    * (f32::consts::PI * 2.0 * (rot_t as f32 / step as f32)).cos()
                    + cy;
                let z = r
                    * (f32::consts::PI * (cir_t as f32 / step as f32)).sin()
                    * (f32::consts::PI * 2.0 * (rot_t as f32 / step as f32)).sin()
                    + cz;
                matrix.add_point(x, y, z);
            }
        }
        return matrix;
    }

    /// add_torus()
    /// Inputs:   struct matrix * points
    ///             double cx
    ///             double cy
    ///             double cz
    ///             double r1
    ///             double r2
    ///             double step
    /// Returns:
    ///
    /// adds all the points required for a torus with center (cx, cy, cz),
    /// circle radius r1 and torus radius r2 using step points per circle.

    /// should call generate_torus to create the necessary points
    pub fn add_torus(&mut self, cx: f32, cy: f32, cz: f32, r1: f32, r2: f32, step: i32) {
        let points_matrix = Matrix::generate_torus(cx, cy, cz, r1, r2, step);
        let lat_start: usize = 0;
        let lat_stop = step as usize;
        let long_start: usize = 0;
        let long_stop = step as usize;
        for lat in lat_start..lat_stop + 1 {
            for longt in long_start..long_stop + 1 {
                let index = lat * step as usize + longt;
                self.add_polygon(
                    points_matrix.matrix_array[0][index],
                    points_matrix.matrix_array[1][index],
                    points_matrix.matrix_array[2][index],
                    points_matrix.matrix_array[0][index + 1],
                    points_matrix.matrix_array[1][index + 1],
                    points_matrix.matrix_array[2][index + 1],
                    points_matrix.matrix_array[0][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                );
                self.add_polygon(
                    points_matrix.matrix_array[0][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2][(index + step as usize + 1) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[0][index + 1],
                    points_matrix.matrix_array[1][index + 1],
                    points_matrix.matrix_array[2][index + 1],
                    points_matrix.matrix_array[0][(index + step as usize + 2) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[1][(index + step as usize + 2) % points_matrix.matrix_array[0].len()],
                    points_matrix.matrix_array[2][(index + step as usize + 2) % points_matrix.matrix_array[0].len()],
                );
            }
        }
    }

    /// generate_torus()
    ///
    /// Inputs:   struct matrix * points
    ///
    /// double cx
    ///
    /// double cy
    ///
    /// double cz
    ///
    /// double r
    ///
    /// int step
    ///
    /// Returns: Generates all the points along the surface
    /// of a torus with center (cx, cy, cz),
    /// circle radius r1 and torus radius r2 using
    /// step points per circle.
    /// Returns a matrix of those points
    pub fn generate_torus(
        cx: f32,
        cy: f32,
        cz: f32,
        circle_radius: f32,
        torus_radius: f32,
        step: i32,
    ) -> Matrix {
        let rot_start = 0;
        let rot_stop = step;
        let circ_start = 0;
        let circ_stop = step;
        let mut matrix = Matrix::new(0, 0);
        for phi in rot_start..rot_stop + 1 {
            for theta in circ_start..circ_stop + 1 {
                let x = (f32::consts::PI * 2.0 * phi as f32 / step as f32).cos()
                    * (circle_radius * (f32::consts::PI * 2.0 * theta as f32 / step as f32).cos()
                        + torus_radius)
                    + cx;
                let y =
                    circle_radius * (f32::consts::PI * 2.0 * theta as f32 / step as f32).sin() + cy;
                let z = (f32::consts::PI * 2.0 * phi as f32 / step as f32).sin()
                    * (circle_radius * (f32::consts::PI * 2.0 * theta as f32 / step as f32).cos()
                        + torus_radius)
                    + cz;
                matrix.add_point(x, y, z);
            }
        }
        return matrix;
    }

    ///======== void add_polygon() ==========
    ///
    ///Inputs:   struct matrix *polygons
    ///
    ///            x0: f32
    ///
    ///            y0: f32
    ///
    ///            z0: f32
    ///
    ///            x1: f32
    ///
    ///            y1: f32
    ///
    ///            z1: f32
    ///
    ///            x2: f32
    ///
    ///            y2: f32
    ///
    ///            z2: f32
    ///
    ///Returns:
    ///
    ///Adds the vertices (x0, y0, z0), (x1, y1, z1)
    ///and (x2, y2, z2) to the polygon matrix. They
    ///define a single triangle surface.
    ///====================
    pub fn add_polygon(
        &mut self,
        x0: f32,
        y0: f32,
        z0: f32,
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
    ) {
        // check for degen triangles, if it is, then don't add
        if (x0 as i32, y0 as i32, z0 as i32) != (x1 as i32, y1 as i32, z1 as i32)
            && (x0 as i32, y0 as i32, z0 as i32) != (x2 as i32, y2 as i32, z2 as i32)
            && (x1 as i32, y1 as i32, z1 as i32) != (x2 as i32, y2 as i32, z2 as i32)
        {
            self.add_point(x0, y0, z0);
            self.add_point(x1, y1, z1);
            self.add_point(x2, y2, z2);
        }
    }
}
