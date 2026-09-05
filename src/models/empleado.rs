use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Empleado {
    pub id: i32,
    pub nombre: String,
    pub salario: f64,
}
