pub mod build;
pub mod channel;
pub mod env;
pub mod err;
pub mod git;

use build::*;
use env::*;
use err::*;
use git::*;

use std::fs::File;
use std::io::Write;
use std::path::Path;

const SHADOW_RS: &str = "shadow.rs";

#[derive(Debug)]
pub struct Shadow {
    f: File,
    project: Project,
    sys_env: SystemEnv,
    git: Git,
}

impl Shadow {
    pub fn new<P: Into<String>>(path: P) -> SdResult<Shadow> {
        let path = path.into();
        let build_path = format!("{}/{}", path, SHADOW_RS);
        let shadow_path = Path::new(&build_path);
        Ok(Shadow {
            f: File::create(shadow_path)?,
            project: Project::new(),
            sys_env: SystemEnv::new()?,
            git: Git::new(Path::new(&path))?,
        })
    }

    fn gen_const(&mut self) -> SdResult<()> {
        self.write_header()?;
        for v in self.git.const_msg.clone() {
            self.write_const(v)?;
        }

        for v in self.project.const_msg.clone() {
            self.write_const(v)?;
        }

        for v in self.sys_env.const_msg.clone() {
            self.write_const(v)?;
        }
        Ok(())
    }

    fn write_header(&self) -> SdResult<()> {
        let desc = r#"// Code generated by shadow-rs generator. DO NOT EDIT."#;
        writeln!(&self.f, "{}\n\n", desc)?;
        Ok(())
    }

    fn write_const(&mut self, msg: ConstMessage) -> SdResult<()> {
        let desc = format!("// {}", msg.desc);
        let define = format!(
            "pub const {} :{} = \"{}\";",
            msg.key.to_ascii_uppercase(),
            msg.t.to_string(),
            msg.val
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
    fn test_build() {
        let mut shadow = Shadow::new("./").unwrap();
        shadow.gen_const().unwrap();
    }
}
