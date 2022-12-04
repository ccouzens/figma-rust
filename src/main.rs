mod figma_api;
fn main() {
    let f: figma_api::File = serde_json::from_reader(std::io::stdin()).unwrap();
    for c in f.document.depth_first_iter() {
        if let figma_api::NodeType::Frame { .. } = &c.node_type {
            if c.name.starts_with("_tokens/motion") {
                println!("{}", c.name);
                for c in c.depth_first_iter() {
                    if c.name.starts_with("motion") {
                        if let (Some(duration), Some(easing)) = (
                            c.frame_props().and_then(|f| f.transition_duration),
                            c.frame_props().and_then(|f| f.transition_easing.as_ref()),
                        ) {
                            println!("{}, {:?} {:?}", c.name, duration, easing);
                        }
                    }
                }
            }
        }
    }
    // println!("{}", serde_json::to_string_pretty(&f).unwrap());
}
