#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2D(pub f32, pub f32);

impl Vector2D {
    #[must_use]
    pub const fn of(x: f32, y: f32) -> Self {
        Self(x, y)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3D(pub f32, pub f32, pub f32);

impl Vector3D {
    #[must_use]
    pub const fn of(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector3D,
    pub normal: Vector3D,
    pub texture_coordinate: Vector2D,
}

impl Vertex {
    #[must_use]
    pub const fn of(position: Vector3D, normal: Vector3D, texture_coordinate: Vector2D) -> Self {
        Self {
            position,
            normal,
            texture_coordinate,
        }
    }
}

pub struct Primitive(Vec<Vertex>);

impl Primitive {
    #[must_use]
    pub fn of(vertices: &[Vertex]) -> Self {
        Self(vertices.to_owned())
    }

    /// Return interleaved vertex data as Vec<f32> in the same layout expected by the GL code:
    /// [x, y, z, nx, ny, nz, u, v, ...]
    #[must_use]
    pub fn to_f32_vec(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.0.len() * 8);
        for v in &self.0 {
            out.push((v.position).0);
            out.push((v.position).1);
            out.push((v.position).2);

            out.push((v.normal).0);
            out.push((v.normal).1);
            out.push((v.normal).2);

            out.push((v.texture_coordinate).0);
            out.push((v.texture_coordinate).1);
        }
        out
    }

    /// Number of vertices in this primitive.
    #[must_use]
    pub const fn vertex_count(&self) -> usize {
        self.0.len()
    }

    /// Return a borrowed f32 slice over the interleaved vertex data ([x,y,z,nx,ny,nz,u,v,...])
    ///
    /// Safety: this uses an unsafe reinterpret-cast of the internal `Vec<Vertex>` memory. It is
    /// safe here because `Vertex` is #[repr(C)] and contains only f32 fields (3+3+2 = 8 f32s)
    /// with no padding, and f32 alignment matches. The returned slice borrows from `self` and
    /// must not outlive the `Primitive` instance.
    #[must_use]
    pub fn as_f32_slice(&self) -> &[f32] {
        // Sanity checks to ensure Vertex layout matches 8 f32s (no unexpected padding)
        debug_assert_eq!(size_of::<Vertex>(), 8 * size_of::<f32>());
        debug_assert_eq!(align_of::<Vertex>(), align_of::<f32>());

        let ptr = self.0.as_ptr().cast::<f32>();
        let len = self.0.len() * 8;
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    /// Return a borrowed byte slice over the underlying vertex memory. Useful for APIs that
    /// expect &[u8]. The same safety considerations as `as_f32_slice` apply.
    #[must_use]
    pub fn as_u8_slice(&self) -> &[u8] {
        debug_assert_eq!(size_of::<Vertex>(), 8 * size_of::<f32>());

        let ptr = self.0.as_ptr().cast::<u8>();
        let len = self.0.len() * 8 * size_of::<f32>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

#[must_use]
pub fn cube() -> Primitive {
    load_vertices_from_csv(include_str!("cube.csv"))
}

#[must_use]
pub fn triangle() -> Primitive {
    load_vertices_from_csv(include_str!("triangle.csv"))
}

pub fn load_vertices_from_csv(csv: &str) -> Primitive {
    let mut vertices: Vec<Vertex> = Vec::new();

    for (i, raw_line) in csv.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        // Split by comma, trim whitespace and ignore empty tokens (in case of trailing commas)
        let parts: Vec<&str> = line
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() != 8 {
            panic!(
                "invalid vertex data on line {}: expected 8 values, got {}: '{}'",
                i + 1,
                parts.len(),
                raw_line
            );
        }

        let mut vals = [0.0f32; 8];
        for (j, token) in parts.iter().enumerate() {
            vals[j] = token.parse::<f32>().unwrap_or_else(|e| {
                panic!(
                    "failed to parse float on line {} token {} ('{}'): {}",
                    i + 1,
                    j + 1,
                    token,
                    e
                )
            });
        }

        let position = Vector3D::of(vals[0], vals[1], vals[2]);
        let normal = Vector3D::of(vals[3], vals[4], vals[5]);
        let tex = Vector2D::of(vals[6], vals[7]);

        vertices.push(Vertex::of(position, normal, tex));
    }

    Primitive(vertices)
}
