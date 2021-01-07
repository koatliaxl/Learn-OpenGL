pub fn _load_model() {
    let (models, materials) = tobj::load_obj(
        "assets/backpack/backpack.obj",
        true, /* Rustfmt force vertical formatting */
    )
    .expect("Failed to load model");
    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    for model in models {
        let mesh = &model.mesh;
        let pos_len = mesh.positions.len();
        let norm_len = mesh.normals.len();
        let txc_len = mesh.texcoords.len();
        let idx_len = mesh.indices.len();
        println!("{}, {}, {}, {}", pos_len, norm_len, txc_len, idx_len);
    }
}
