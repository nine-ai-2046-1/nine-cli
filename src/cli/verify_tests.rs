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
}
