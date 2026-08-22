//use paperstrap::build::build;
use paperstrap::modrinth;

fn main() {
    //build();
    let meta = modrinth::fetch_plugin("viaversion").unwrap();
    match modrinth::find_matching_version(&meta, Some("5.10.0"), "1.8.9", "release") {
        Ok(v) => {
            dbg!(v);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
