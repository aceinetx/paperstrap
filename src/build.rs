use crate::PaperstrapConfig;
use nickel_lang::Context;

pub fn read_config() -> String {
    let source = r#"
{
    build_path = "build/",
    version = "26.2",
    build = 112,
    hash = "bd3a58cf96874e5ea6643f5f6fe9b4f5bf9e34b795fa078c2f0ee8b98b2f907e",
    server_properties = {
        online_mode = false,
    },
    paper_global_config = {},
}
    "#;
    String::from(source)
}

pub fn build() {
    let mut context = Context::new();

    let source = read_config();

    let value = match context.eval_deep(&source) {
        Ok(v) => v,
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };

    let json_str = match context.expr_to_json(&value) {
        Ok(v) => v,
        Err(e) => {
            println!("{:#?}", e);
            return;
        }
    };

    let cfg: PaperstrapConfig = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    cfg.initialize().unwrap();
    cfg.download_paper_verify().unwrap();
    cfg.add_startup_scripts();
    cfg.add_eula();
    cfg.add_server_properties();
    cfg.add_paper_global_config();
    cfg.symlink_world();
    cfg.symlink_plugins();
}
