use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut drops: Vec<(usize, usize, usize)> = Vec::new();

    for line in contents.lines() {
        let mut str_coords = line.split(',');
        let x: usize = str_coords.next().unwrap().parse().unwrap();
        let y: usize = str_coords.next().unwrap().parse().unwrap();
        let z: usize = str_coords.next().unwrap().parse().unwrap();
        drops.push((x, y, z));
    }

    let mut faces = drops.len() * 6;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for drop in &drops {
        if drop.0 > max_x { max_x = drop.0 };
        if drop.1 > max_y { max_y = drop.1 };
        if drop.2 > max_z { max_z = drop.2 };
        for anotherdrop in &drops {
            if (drop.0 == anotherdrop.0 + 1 && drop.1 == anotherdrop.1 && drop.2 == anotherdrop.2)
            || (drop.0 == anotherdrop.0 && drop.1 == anotherdrop.1 + 1 && drop.2 == anotherdrop.2)
            || (drop.0 == anotherdrop.0 && drop.1 == anotherdrop.1 && drop.2 == anotherdrop.2 + 1) {
                faces = faces - 2;
            }
        }
    }

    println!("Result 1: {} faces", faces);

    println!("Maxes are x:{} y:{} z:{}", max_x, max_y, max_z);

    let mut planes = [[[0; 24];24];24];

    for drop in &drops {
        planes[drop.0+1][drop.1+1][drop.2+1] = 1;
    }

    let mut pending: Vec<(usize, usize, usize)> = Vec::new();
    pending.push((0,0,0));

    while !pending.is_empty() {
        let d = pending.pop().unwrap();
        if planes[d.0][d.1][d.2] == 0 {
            planes[d.0][d.1][d.2] = 9;
            if d.0 > 0 {
                pending.push((d.0-1,d.1,d.2));
            }
            if d.0 < 23 {
                pending.push((d.0+1,d.1,d.2));
            }
            if d.1 > 0 {
                pending.push((d.0,d.1-1,d.2));
            }
            if d.1 < 23 {
                pending.push((d.0,d.1+1,d.2));
            }
            if d.2 > 0 {
                pending.push((d.0,d.1,d.2-1));
            }
            if d.2 < 23 {
                pending.push((d.0,d.1,d.2+1));
            }
        }
    }

    let mut faces = 0;

    for i in 1..24 {
        let mut layer_faces = 0;
        for j in 1..24 {
            for k in 1..24 {
                if planes[i][j][k] == 9 {
                    if planes[i-1][j][k] == 1 { layer_faces = layer_faces + 1 }
                    if planes[i][j-1][k] == 1 { layer_faces = layer_faces + 1 }
                    if planes[i][j][k-1] == 1 { layer_faces = layer_faces + 1 }
                } else if planes[i][j][k] == 1 {
                    if planes[i-1][j][k] == 9 { layer_faces = layer_faces + 1 }
                    if planes[i][j-1][k] == 9 { layer_faces = layer_faces + 1 }
                    if planes[i][j][k-1] == 9 { layer_faces = layer_faces + 1 }
                }
            }
        }
        faces = faces + layer_faces;
        print_plane(planes[i]);
        println!("Layer {} has {}", i, layer_faces);
    }

    //print_planes(planes);

    println!("Result 2: {} faces", faces);
}

fn print_planes(planes: [[[u32; 24]; 24]; 24]) {
    for i in 0..24 {
        println!("Plane x={}", i);
        print_plane(planes[i]);
    }
}

fn print_plane(plane: [[u32; 24]; 24]) {
    for j in 0..24 {
        for k in 0..24 {
            print!("{}", plane[j][k]);
        }
        println!();
    }
}