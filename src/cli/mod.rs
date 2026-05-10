pub mod welcome;

// include unit tests module file
#[cfg(test)]
mod verify_tests;

/// Run the CLI. Currently only shows the welcome message.
pub fn run() {
    // ensure required folders exist in $HOME/.nine-cli
    if let Err(e) = ensure_dirs() {
        eprintln!("建立資料夾時出錯: {}", e);
        return;
    }

    // dispatch based on argv: if no args -> show user guide; else try to run skill
    let mut args = std::env::args().skip(1);
    match args.next() {
        None => {
            // only show welcome when user ran the CLI with no arguments
            welcome::show();
            show_user_guide()
        },
        Some(first) => {
            // support `nine-cli skill ...` subcommands for install/list/uninstall
            if first == "skill" {
                let rest: Vec<String> = args.collect();
                if let Err(e) = handle_skill_cmd(&rest) {
                    eprintln!("{}", e);
                }
                return;
            }

            let skill_name = first;
            let rest: Vec<String> = args.collect();
            if let Err(e) = dispatch_skill(&skill_name, &rest) {
                eprintln!("{}", e);
            }
        }
    }
}

fn ensure_dirs() -> std::io::Result<()> {
    use std::fs;
    use std::path::PathBuf;

    // prefer dirs::home_dir for a more reliable home directory
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let base: PathBuf = home.join(".nine-cli");
    let skills = base.join("skills");
    let config = base.join("config");

    fs::create_dir_all(&skills)?;
    fs::create_dir_all(&config)?;
    Ok(())
}

fn dispatch_skill(skill: &str, args: &[String]) -> Result<(), String> {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use std::fs;

    let home = dirs::home_dir()
        .or_else(|| std::env::var("HOME").ok().map(std::path::PathBuf::from))
        .ok_or_else(|| "未能取得家目錄".to_string())?;
    let skill_dir: PathBuf = home.join(".nine-cli").join("skills").join(skill);

    if !skill_dir.is_dir() {
        return Err("Skill唔存在".to_string());
    }

    let run_path = skill_dir.join("bin").join("run");
    if !run_path.is_file() {
        return Err("Skill唔存在".to_string());
    }

    // ensure executable permission on unix-like systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(&run_path) {
            let mut perm = meta.permissions();
            let mode = perm.mode();
            if mode & 0o111 == 0 {
                // set to rwxr-xr-x
                perm.set_mode(0o755);
                let _ = fs::set_permissions(&run_path, perm);
            }
        }
    }

    // spawn the skill binary and forward stdio/stderr/stdin
    let mut cmd = Command::new(run_path);
    if !args.is_empty() {
        cmd.args(args);
    }
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().map_err(|e| format!("執行 skill 時出錯: {}", e))?;
    let status = child
        .wait()
        .map_err(|e| format!("等待 skill 完成時出錯: {}", e))?;

    // propagate exit code
    match status.code() {
        Some(code) => std::process::exit(code),
        None => Err("Skill 被訊號終止".to_string()),
    }
}

fn show_user_guide() {
    use std::fs;
    use std::path::PathBuf;

    // read default language from src/languages/default.toml using toml crate
    let default_toml_path: PathBuf = ["src", "languages", "default.toml"].iter().collect();
    let default_lang = fs::read_to_string(&default_toml_path)
        .ok()
        .and_then(|s| toml::from_str::<toml::Value>(&s).ok())
        .and_then(|v| v.get("default").and_then(|d| d.as_str().map(|s| s.to_string())))
        .unwrap_or_else(|| "zh-hk".to_string());

    let user_guide_path: PathBuf = ["src", "languages", &default_lang, "user_guide.txt"].iter().collect();
    let guide = fs::read_to_string(&user_guide_path).unwrap_or_else(|_| "請往Git repo查下最新教學".to_string());

    println!("{}", guide);
}

fn load_message(key: &str, vars: Option<&std::collections::HashMap<&str, String>>) -> String {
    use once_cell::sync::OnceCell;
    use std::fs;
    use std::path::PathBuf;

    // Cache parsed messages.toml for the default language to avoid repeated file IO
    static MESSAGES: OnceCell<Option<toml::Value>> = OnceCell::new();

    // lazily initialize cache
    let _ = MESSAGES.get_or_init(|| {
        // read default language
        let default_toml_path: PathBuf = ["src", "languages", "default.toml"].iter().collect();
        let default_lang = fs::read_to_string(&default_toml_path)
            .ok()
            .and_then(|s| toml::from_str::<toml::Value>(&s).ok())
            .and_then(|v| v.get("default").and_then(|d| d.as_str().map(|s| s.to_string())))
            .unwrap_or_else(|| "zh-hk".to_string());

        let msg_path: PathBuf = ["src", "languages", &default_lang, "messages.toml"].iter().collect();
        let content = fs::read_to_string(&msg_path).ok();
        if let Some(ref s) = content {
            if let Ok(v) = toml::from_str::<toml::Value>(s) {
                return Some(v);
            }
        }
        None
    });

    let template = MESSAGES.get().and_then(|opt| opt.as_ref()).and_then(|v| v.get(key)).and_then(|x| x.as_str()).map(|s| s.to_string());

    let mut out = template.unwrap_or_else(|| key.to_string());
    if let Some(map) = vars {
        for (k, v) in map.iter() {
            // support templates like {name} and {name} with optional spaces
            out = out.replace(&format!("{{{}}}", k), v);
            out = out.replace(&format!("{{ {} }}", k), v);
            out = out.replace(&format!("{{{} }}", k), v);
            out = out.replace(&format!("{{ {} }}", k), v);
        }
    }
    out
}

// handle `nine-cli skill <sub>`
fn handle_skill_cmd(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("請指定 skill 子命令: install|list|uninstall".to_string());
    }
    // detect --json/-j flag anywhere in the args and strip it
    let mut json_mode = false;
    let mut cleaned: Vec<String> = Vec::new();
    for a in args.iter() {
        if a == "--json" || a == "-j" {
            json_mode = true;
            continue;
        }
        cleaned.push(a.clone());
    }

    match cleaned.get(0).map(|s| s.as_str()).unwrap_or("") {
        // support both new `add` and legacy `install`
        "add" | "install" => {
            if cleaned.len() < 2 {
                return Err(load_message("please_provide_install_path", None));
            }
            let path = &cleaned[1];
            install_skill(path, json_mode)
        }
        "list" => list_skills(),
        // verify a local skill folder without installing it
        "verify" => {
            if args.len() < 2 {
                return Err(load_message("please_provide_install_path", None));
            }
            let path = &args[1];
            verify_skill_cmd(path)
        }
        // support both new `remove` and legacy `uninstall`
        "remove" | "uninstall" => {
            if cleaned.len() < 2 {
                return Err(load_message("please_provide_uninstall_name", None));
            }
            // check for -y flag
            let mut yes = false;
            let name = cleaned[1].clone();
            if cleaned.len() > 2 {
                for a in &cleaned[2..] {
                    if a == "-y" {
                        yes = true;
                    }
                }
            }
            uninstall_skill(&name, yes, json_mode)
        }
        other => {
            let other = other;
            let mut vars = std::collections::HashMap::new();
            vars.insert("name", other.to_string());
            Err(load_message("unknown_skill_subcommand", Some(&vars)))
        }
    }
}

fn install_skill(src_path: &str, json_mode: bool) -> Result<(), String> {
    use std::fs;
    use std::path::Path;

    let src = Path::new(src_path);
    if !src.exists() || !src.is_dir() {
        return Err(load_message("skill_path_not_exist", None));
    }

    // validate required files and folders according to spec + our requirements
    // required: cli/run (file), CLI.md (file), TEST.md (file), tests (dir)
    let cli_run = src.join("cli").join("run");
    let cli_md = src.join("CLI.md");
    let test_md = src.join("TEST.md");
    let tests_dir = src.join("tests");

    if !cli_run.is_file() || !cli_md.is_file() || !test_md.is_file() || !tests_dir.is_dir() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("reason".into(), "需要 cli/run, CLI.md, TEST.md 同 tests folder".into());
        return Err(load_message("skill_md_read_failed", Some(&vars)));
    }

    // call verifySkill - validates SKILL.md per agentskills.io and ensure bin/run is binary
    if let Err(e) = verify_skill(src) {
        let mut vars = std::collections::HashMap::new();
        vars.insert("reason", e.clone());
        return Err(load_message("skill_md_read_failed", Some(&vars)));
    }

    // derive skill folder name from source folder name
    let skill_name = src.file_name().and_then(|s| s.to_str()).ok_or_else(|| "無效嘅 skill folder 名稱".to_string())?;

    // enforce agentskills.io folder name rules: 1-64 chars, lowercase letters, numbers and hyphens only,
    // must not start/end with hyphen, must not contain consecutive hyphens
    if let Some(err) = validate_agentskills_name(skill_name) {
        return Err(format!("skill folder name 唔合規: {}", err));
    }

    // copy to $HOME/.nine-cli/skills/<skill_name>
    let home = dirs::home_dir().ok_or_else(|| "未能取得家目錄".to_string())?;
    let dest = home.join(".nine-cli").join("skills").join(skill_name);
    if dest.exists() {
        return Err(load_message("skill_already_exists", None));
    }

    copy_dir_all(src, &dest).map_err(|e| format!("複製 skill 時出錯: {}", e))?;

    // set permissions: cli/run executable; scripts/* executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let run_path = dest.join("cli").join("run");
        if let Ok(meta) = fs::metadata(&run_path) {
            let mut perm = meta.permissions();
            perm.set_mode(0o755);
            let _ = fs::set_permissions(&run_path, perm);
        }

        let scripts_dir = dest.join("scripts");
        if scripts_dir.exists() && scripts_dir.is_dir() {
            for entry in fs::read_dir(&scripts_dir).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
                if let Ok(ent) = entry {
                    let p = ent.path();
                    if p.is_file() {
                        if let Ok(meta) = fs::metadata(&p) {
                            let mut perm = meta.permissions();
                            perm.set_mode(0o755);
                            let _ = fs::set_permissions(&p, perm);
                        }
                    }
                }
            }
        }
    }

    let mut vars = std::collections::HashMap::new();
    vars.insert("name", skill_name.to_string());
    vars.insert("path", dest.display().to_string());
    let msg = load_message("skill_install_success", Some(&vars));
    if json_mode {
        let mut map = serde_json::Map::new();
        map.insert("success".to_string(), serde_json::Value::Bool(true));
        map.insert("name".to_string(), serde_json::Value::String(skill_name.to_string()));
        map.insert("path".to_string(), serde_json::Value::String(dest.display().to_string()));
        map.insert("message".to_string(), serde_json::Value::String(msg));
        println!("{}", serde_json::Value::Object(map).to_string());
    } else {
        println!("{}", msg);
    }
    Ok(())
}

fn list_skills() -> Result<(), String> {
    use std::fs;
    let home = dirs::home_dir().ok_or_else(|| "未能取得家目錄".to_string())?;
    let skills = home.join(".nine-cli").join("skills");
    if !skills.is_dir() {
        println!("");
        return Ok(());
    }
    let mut entries: Vec<String> = Vec::new();
    for entry in fs::read_dir(&skills).map_err(|e| format!("讀取 skills 目錄失敗: {}", e))? {
        if let Ok(ent) = entry {
            if ent.path().is_dir() {
                if let Some(name) = ent.file_name().to_str() {
                    entries.push(name.to_string());
                }
            }
        }
    }
    entries.sort();
    for e in entries {
        println!("{}", e);
    }
    Ok(())
}

fn uninstall_skill(name: &str, yes: bool, json_mode: bool) -> Result<(), String> {
    use std::fs;
    let home = dirs::home_dir().ok_or_else(|| "未能取得家目錄".to_string())?;
    let path = home.join(".nine-cli").join("skills").join(name);
    if !path.exists() {
        if json_mode {
            let mut map = serde_json::Map::new();
            map.insert("success".to_string(), serde_json::Value::Bool(false));
            map.insert("message".to_string(), serde_json::Value::String(load_message("skill_not_found", None)));
            println!("{}", serde_json::Value::Object(map).to_string());
            return Ok(());
        }
        return Err(load_message("skill_not_found", None));
    }
    if !yes {
        // ask for confirmation
        use std::io::{self, Write};
        let mut vars = std::collections::HashMap::new();
        vars.insert("name", name.to_string());
        print!("{}", load_message("skill_remove_confirm", Some(&vars)));
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| format!("讀入回應失敗: {}", e))?;
        let resp = input.trim().to_lowercase();
        if resp != "y" && resp != "yes" {
            println!("{}", load_message("skill_remove_cancelled", None));
            return Ok(());
        }
    }

    fs::remove_dir_all(&path).map_err(|e| format!("移除時出錯: {}", e))?;
    let mut vars = std::collections::HashMap::new();
    vars.insert("name", name.to_string());
    let msg = load_message("skill_remove_success", Some(&vars));
    if json_mode {
        let mut map = serde_json::Map::new();
        map.insert("success".to_string(), serde_json::Value::Bool(true));
        map.insert("name".to_string(), serde_json::Value::String(name.to_string()));
        map.insert("message".to_string(), serde_json::Value::String(msg));
        println!("{}", serde_json::Value::Object(map).to_string());
    } else {
        println!("{}", msg);
    }
    Ok(())
}

fn verify_skill_cmd(path: &str) -> Result<(), String> {
    use std::path::Path;
    let p = Path::new(path);
    let mut out = serde_json::Map::new();
    match verify_skill(p) {
        Ok(()) => {
            out.insert("success".to_string(), serde_json::Value::Bool(true));
            out.insert("message".to_string(), serde_json::Value::String("OK".to_string()));
        }
        Err(e) => {
            out.insert("success".to_string(), serde_json::Value::Bool(false));
            out.insert("message".to_string(), serde_json::Value::String(e));
        }
    }
    let j = serde_json::Value::Object(out);
    println!("{}", serde_json::to_string(&j).unwrap_or_else(|_| "{\"success\":false,\"message\":\"json_encode_error\"}".to_string()));
    Ok(())
}

// stub verifySkill - currently always true
fn verify_skill(skill: &std::path::Path) -> Result<(), String> {
    // implement minimal checks based on agentskills.io spec:
    // - SKILL.md exists and has YAML frontmatter with name and description
    // - name follows constraints (lowercase letters, numbers, hyphens only, 1-64 chars, no leading/trailing hyphen, no consecutive hyphens)
    // - bin/run exists and appears to be a binary (not plain text, jpg etc.)
    use std::fs;

    let skill_md = skill.join("SKILL.md");
    if !skill_md.is_file() {
        return Err("SKILL.md not found".to_string());
    }

    // read SKILL.md and extract YAML frontmatter between lines starting with ---
    let content = match fs::read_to_string(&skill_md) {
        Ok(s) => s,
        Err(_) => return Err("failed to read SKILL.md".to_string()),
    };
    let mut lines = content.lines();
    let first = lines.next().unwrap_or("");
    if !first.trim().starts_with("---") {
        return Err("SKILL.md missing YAML frontmatter".to_string());
    }
    // collect frontmatter
    let mut front = String::new();
    for line in lines.by_ref() {
        if line.trim().starts_with("---") {
            break;
        }
        front.push_str(line);
        front.push('\n');
    }

    // parse YAML frontmatter
    let yaml: serde_yaml::Value = match serde_yaml::from_str(&front) {
        Ok(v) => v,
        Err(_) => return Err("failed to parse SKILL.md frontmatter as YAML".to_string()),
    };

    // check name and description
    let name = yaml.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let description = yaml.get("description").and_then(|v| v.as_str()).unwrap_or("");
    if name.is_empty() || description.is_empty() {
        return Err("SKILL.md frontmatter must include name and description".to_string());
    }

    if !(1..=64).contains(&name.len()) {
        return Err("SKILL.md name length must be between 1 and 64".to_string());
    }
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err("SKILL.md name must only contain lowercase letters, numbers and hyphens".to_string());
    }
    if name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return Err("SKILL.md name must not start/end with hyphen or contain consecutive hyphens".to_string());
    }

    // ensure cli/run exists (developer may provide binary or script)
    let run = skill.join("cli").join("run");
    if !run.is_file() {
        return Err("cli/run not found".to_string());
    }

    Ok(())
}

fn validate_agentskills_name(name: &str) -> Option<String> {
    if name.len() < 1 || name.len() > 64 {
        return Some("length must be between 1 and 64".to_string());
    }
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Some("only lowercase letters, numbers and hyphens are allowed".to_string());
    }
    if name.starts_with('-') || name.ends_with('-') {
        return Some("must not start or end with a hyphen".to_string());
    }
    if name.contains("--") {
        return Some("must not contain consecutive hyphens".to_string());
    }
    None
}

// recursive copy utility
fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    use std::fs;
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else if ty.is_file() {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
// no custom TOML parser needed anymore; using `toml` crate above
