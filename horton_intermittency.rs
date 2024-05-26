use nalgebra::Vector3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::f64;

fn horton(xyz: &Vector3<f64>, c1: f64, c2: f64, c3: f64, b3: f64) -> Vector3<f64> {
    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];

    let x_dot = -y;
    let y_dot = c1 * x - b3 * y - z;
    let z_dot = -c2 - c3 * x.tanh();

    Vector3::new(x_dot, y_dot, z_dot)
}

fn main() -> std::io::Result<()> {
    let c1 = 2.6676;
    let c2 = 7000.0;
    let c3 = 20000.0;
    let b3 = 1.06;
    let dt = 0.1;
    let num_steps = 10000;

    let mut xyzs = Vec::with_capacity(num_steps + 1);
    xyzs.push(Vector3::new(0.0, 1.0, 1.05));

    for i in 0..num_steps {
        let next = xyzs[i] + horton(&xyzs[i], c1, c2, c3, b3) * dt;
        xyzs.push(next);
    }

    let max_vals = xyzs.iter().fold(Vector3::new(f64::MIN, f64::MIN, f64::MIN), |acc, v| {
        Vector3::new(acc[0].max(v[0].abs()), acc[1].max(v[1].abs()), acc[2].max(v[2].abs()))
    });

    let xyzs_normalized: Vec<Vector3<f64>> = xyzs.iter().map(|v| v.component_div(&max_vals)).collect();
    let xyzs_rounded: Vec<Vector3<f64>> = xyzs_normalized.iter().map(|v| v.map(|x| (x * 1e9).round() / 1e9)).collect();

    let file = File::create("horton_intermittency.dat")?;
    let mut writer = BufWriter::new(file);

    for v in xyzs_rounded {
        writeln!(writer, "{:.9}\t{:.9}\t{:.9}", v[0], v[1], v[2])?;
    }

    Ok(())
}
