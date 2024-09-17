use macroquad::prelude::*;

const RADIAN: f64 = std::f64::consts::PI / 180.0;

#[derive(Clone)]
struct Wireframe {
  vertices: Vec<(f64, f64, f64)>,
  edges: Vec<(u8, u8)>
}

fn round(x: f64) -> i32 {
  if x < 0.0 {
    return (x - 0.5).ceil() as i32;
  } else {
    (x + 0.5).floor() as i32
  }
}

fn draw_wireframe(wireframe: Wireframe, focal_length: f64, thickness: f32) {
  let mut points: Vec<(i32, i32)> = Vec::with_capacity(wireframe.vertices.len() as usize);

  for i in 0..wireframe.vertices.len() {
    if focal_length == -wireframe.vertices[i].2 { //prevent divide by zero errors
      points.push((0, 0));
      continue;
    }
    points.push((
      round(
        (focal_length * wireframe.vertices[i].0) / (focal_length + wireframe.vertices[i].2)
      ),       
      round(
        (focal_length * wireframe.vertices[i].1) / (focal_length + wireframe.vertices[i].2)
      )
    ));
  }

  for i in 0..wireframe.edges.len() {
    let points0 = points[wireframe.edges[i].0 as usize];
    let points1 = points[wireframe.edges[i].1 as usize];
    macroquad::shapes::draw_line(
      points0.0 as f32 * screen_width() / 1000.0 + (screen_width()) / 2.0,
      points0.1 as f32 * screen_width() / 1000.0 + (screen_height()) / 2.0,
      points1.0 as f32 * screen_width() / 1000.0 + (screen_width()) / 2.0,
      points1.1 as f32 * screen_width() / 1000.0 + (screen_height()) / 2.0,
      thickness, WHITE
    );
  }
}

fn rotate(wireframe: Wireframe, roll: f64, pitch: f64, yaw: f64) -> Wireframe {
  let mut rotated_wireframe: Wireframe = Wireframe {vertices: vec![], edges: vec![]};

  for i in 0..wireframe.vertices.len() {
    
    let roll_angles: (f64, f64) = (roll * RADIAN).sin_cos();
    let pitch_angles: (f64, f64) = (pitch * RADIAN).sin_cos();
    let yaw_angles: (f64, f64) = (yaw * RADIAN).sin_cos();

    let mut x = wireframe.vertices[i].0;
    let mut y = wireframe.vertices[i].1;
    let mut z = wireframe.vertices[i].2;

    let mut new_x = (x * roll_angles.1) - (y * roll_angles.0);
    let mut new_y = (x * roll_angles.0) + (y * roll_angles.1);
    let mut new_z;

    x = new_x;
    y = new_y;

    new_y = (y * pitch_angles.1) - (z * pitch_angles.0);
    new_z = (y * pitch_angles.0) + (z * pitch_angles.1);

    y = new_y;
    z = new_z;

    new_x = (x * yaw_angles.1) - (z * yaw_angles.0);
    new_z = (x * yaw_angles.0) + (z * yaw_angles.1);

    x = new_x;
    z = new_z;
    
    rotated_wireframe.vertices.push((x, y, z));
  }

  for i in 0..wireframe.edges.len() {
    rotated_wireframe.edges.push(wireframe.edges[i]);
  }

  return rotated_wireframe;
}

#[macroquad::main("Test")]
async fn main() {
  let cube: Wireframe = Wireframe {
    vertices: vec![
      (50.0, 50.0, 50.0),
      (50.0, -50.0, 50.0),
      (-50.0, -50.0, 50.0),
      (-50.0, 50.0, 50.0),
      (50.0, 50.0, -50.0),
      (50.0, -50.0, -50.0),
      (-50.0, -50.0, -50.0),
      (-50.0, 50.0, -50.0)
    ],
    edges: vec![
      (0, 1),
      (1, 2),
      (2, 3),
      (3, 0),
      (4, 5),
      (5, 6),
      (6, 7),
      (7, 4),
      (0, 4),
      (1, 5),
      (2, 6),
      (3, 7)
    ]
  };

  let mut roll = 0.0;
  let mut pitch = 0.0;
  let mut yaw = 0.0;
  
  loop {
    draw_wireframe(rotate(cube.clone(), roll, pitch, yaw), -200.0, 2.0);
    roll += 1.0;
    pitch += 1.0;
    yaw += 1.0;
    next_frame().await;
  }
}