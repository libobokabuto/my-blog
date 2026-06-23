use app::content;

fn main() {
    match content::validate_content_tree() {
        Ok(errors) if errors.is_empty() => {
            println!("内容校验通过：没有发现 front matter、slug 或站内链接问题。");
        }
        Ok(errors) => {
            eprintln!("内容校验失败，共发现 {} 个问题：", errors.len());
            for error in errors {
                eprintln!("- {}", error);
            }
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("内容校验执行失败：{error}");
            std::process::exit(1);
        }
    }
}
