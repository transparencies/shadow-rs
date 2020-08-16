pub mod build;
pub mod channel;
pub mod env;
pub mod err;
pub mod git;

use build::*;
use env::*;
use err::*;
use git::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const SHADOW_RS: &str = "shadow.rs";

#[derive(Debug)]
pub struct Shadow {
    f: File,
    map: HashMap<ShadowConst, RefCell<ConstVal>>,
}

impl Shadow {
    pub fn new(src_path: String, out_path: String) -> SdResult<()> {
        let src_path = Path::new(src_path.as_str());

        let out = {
            let path = Path::new(out_path.as_str());
            if !out_path.ends_with("/") {
                path.join(format!("{}/{}", out_path, SHADOW_RS))
            } else {
                path.join(SHADOW_RS)
            }
        };


        let mut map = Git::new(&src_path);
        for (k, v) in Project::new() {
            map.insert(k, v);
        }
        for (k, v) in SystemEnv::new() {
            map.insert(k, v);
        }

        let mut shadow = Shadow {
            f: File::create(out)?,
            map,
        };
        shadow.gen_const()?;
        println!("shadow build success");
        Ok(())
    }

    fn gen_const(&mut self) -> SdResult<()> {
        self.write_header()?;
        for (k, v) in self.map.clone() {
            self.write_const(k, v)?;
        }
        Ok(())
    }

    fn write_header(&self) -> SdResult<()> {
        let desc = r#"// Code generated by shadow-rs generator. DO NOT EDIT."#;
        writeln!(&self.f, "{}\n\n", desc)?;
        Ok(())
    }

    fn write_const(&mut self, shadow_const: ShadowConst, val: RefCell<ConstVal>) -> SdResult<()> {
        let val = val.into_inner();
        let desc = format!("// {}", val.desc);
        let define = format!(
            "pub const {} :{} = \"{}\";",
            shadow_const.to_ascii_uppercase(),
            val.t.to_string(),
            val.v
        );
        writeln!(&self.f, "{}", desc)?;
        writeln!(&self.f, "{}\n", define)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() -> SdResult<()> {
        Shadow::new("./".into(), "./".into())?;
        Ok(())
    }
}
