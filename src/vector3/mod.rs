struct Vector3 {
  x: f32,
  y: f32,
  z: f32,
}

impl Config {

  fn add(v: Vector3) {
    x = x + v.x;
    y = y + v.y;
    z = z + v.z;
  }

}
