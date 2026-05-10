#[cfg(test)]
mod tests {
    use super::super::verify_skill;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn verify_valid_skill() {
        let d = tempdir().unwrap();
        let p = d.path();
        fs::create_dir_all(p.join("cli")).unwrap();
        fs::create_dir_all(p.join("tests")).unwrap();
        fs::write(p.join("SKILL.md"), "---\nname: my-skill\ndescription: ok\n---\n").unwrap();
        fs::write(p.join("cli/run"), "#!/bin/bash\necho hi").unwrap();
        let res = verify_skill(p);
        assert!(res.is_ok());
    }

    #[test]
    fn verify_missing_skill_md() {
        let d = tempdir().unwrap();
        let p = d.path();
        fs::create_dir_all(p.join("cli")).unwrap();
        fs::create_dir_all(p.join("tests")).unwrap();
        fs::write(p.join("cli/run"), "#!/bin/bash\necho hi").unwrap();
        let res = verify_skill(p);
        assert!(res.is_err());
    }

    #[test]
    fn install_replace_flow_unit() {
        use super::super::install_skill_with_confirm;
        let tmpdir = tempdir().unwrap();
        let src = tmpdir.path().join("replace-skill");
        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(src.join("cli")).unwrap();
        fs::create_dir_all(src.join("tests")).unwrap();
        fs::write(src.join("SKILL.md"), "---\nname: replace-skill\ndescription: ok\n---\n").unwrap();
        fs::write(src.join("CLI.md"), "usage: replace-skill").unwrap();
        fs::write(src.join("TEST.md"), "test: run").unwrap();
        fs::write(src.join("cli/run"), "#!/bin/sh\necho hi").unwrap();

        let homed = tempdir().unwrap();
        let home = homed.path();
        let dest = home.join(".nine-cli").join("skills").join("replace-skill");
        fs::create_dir_all(dest.join("cli")).unwrap();
        fs::write(dest.join("SKILL.md"), "---\nname: replace-skill\ndescription: old\n---\n").unwrap();
        fs::write(dest.join("CLI.md"), "old").unwrap();
        fs::write(dest.join("TEST.md"), "old").unwrap();
        fs::write(dest.join("cli/run"), "#!/bin/sh\necho old").unwrap();

        // override HOME for the duration of the call by setting env var used by dirs::home_dir
        std::env::set_var("HOME", home);
        // call with confirm input = "y" to simulate user confirming replacement
        let res = install_skill_with_confirm(src.to_str().unwrap(), false, Some("y"));
        if res.is_err() {
            eprintln!("install error: {:?}", res);
        }
        assert!(res.is_ok());
        let content = fs::read_to_string(dest.join("SKILL.md")).unwrap();
        assert!(content.contains("description: ok"));
    }
}
