use cgmath::Vector3;

use crate::render::Vertex;

pub fn octahedron() -> (Vec<Vertex>, Vec<u16>) {
    let vertexes = [Vector3::unit_x(), Vector3::unit_y(), Vector3::unit_z()]
        .into_iter()
        .flat_map(|dir| [dir, -dir].into_iter())
        .map(|dir| Vertex {
            position: dir.into(),
            tex_coords: [dir.x, dir.z].map(|c| (c + 1.0) / 2.0),
        })
        .collect();

    const PX: u16 = 0;
    const NX: u16 = 1;
    const PY: u16 = 2;
    const NY: u16 = 3;
    const PZ: u16 = 4;
    const NZ: u16 = 5;
    #[rustfmt::skip]
    let indexes = vec![
        PX, PZ, PY,
        PZ, NX, PY,
        NX, NZ, PY,
        NZ, PX, PY,
        PX, NY, PZ,
        PZ, NY, NX,
        NX, NY, NZ,
        NZ, NY, PX,
    ];

    (vertexes, indexes)
}
