use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct ReflectionValue{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ReflectionValue{
    pub fn new() -> ReflectionValue{
        ReflectionValue{r: 0.0, g: 0.0, b: 0.0}
    }

    pub const fn new_values(r: f32, g: f32, b: f32) -> ReflectionValue{
        ReflectionValue{r: r, g: g, b: b}
    }
}

impl fmt::Display for ReflectionValue{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}