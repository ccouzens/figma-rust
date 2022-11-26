use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GlobalProperties {
    id: String,
    name: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Color {
    #[serde(rename = "r")] 
    red: f64,
    #[serde(rename = "g")] 
    green: f64,
    #[serde(rename = "b")] 
    blue: f64,
    #[serde(rename = "a")] 
    alpha: f64,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Canvas {
    #[serde(flatten)] 
    global_properties: GlobalProperties,
    background_color: Color,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    #[serde(flatten)] 
    global_properties: GlobalProperties,
    children: Vec<Canvas>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct File {
    document: Document,
    name: String,
    schema_version: u8,
    version: String,
}

fn main() {
    let f: File = serde_json::from_reader(std::io::stdin()).unwrap();
    println!("{}", serde_json::to_string_pretty(&f).unwrap());
}
