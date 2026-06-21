use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    anyhow::{Context, Result, anyhow},
    pulldown_cmark::{Options, Parser, html},
    std::{fs, path::Path},
    thiserror::Error,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogPostSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdjacentPost {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub html: String,
    pub previous: Option<AdjacentPost>,
    pub next: Option<AdjacentPost>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub status: String,
    pub stack: Vec<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum ContentError {
    #[error("{0} 内容目录不存在：{1}")]
    MissingDirectory(&'static str, String),
    #[error("没有找到 slug 为 {0} 的文章")]
    PostNotFound(String),
    #[error("解析 {0} 内容失败：{1}")]
    ParseFailure(&'static str, String),
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct BlogFrontMatter {
    title: String,
    summary: String,
    date: NaiveDate,
    tags: Vec<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct NoteFrontMatter {
    title: String,
    summary: String,
    date: NaiveDate,
    tags: Vec<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct ProjectFrontMatter {
    title: String,
    summary: String,
    status: String,
    stack: Vec<String>,
    repo_url: Option<String>,
    live_url: Option<String>,
}

#[cfg(feature = "ssr")]
const BLOG_DIR: &str = "content/blog";
#[cfg(feature = "ssr")]
const NOTES_DIR: &str = "content/notes";
#[cfg(feature = "ssr")]
const PROJECTS_DIR: &str = "content/projects";

#[cfg(feature = "ssr")]
pub async fn list_blog_posts() -> Result<Vec<BlogPostSummary>> {
    let mut posts = load_blog_posts()?
        .into_iter()
        .map(|post| BlogPostSummary {
            slug: post.slug,
            title: post.title,
            summary: post.summary,
            date: post.date,
            tags: post.tags,
        })
        .collect::<Vec<_>>();

    posts.sort_by(|left, right| right.date.cmp(&left.date));

    Ok(posts)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_blog_posts() -> Result<Vec<BlogPostSummary>, String> {
    unreachable!("list_blog_posts 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_blog_post(slug: &str) -> Result<BlogPost> {
    let mut posts = load_blog_posts()?;
    posts.sort_by(|left, right| right.date.cmp(&left.date));

    let index = posts
        .iter()
        .position(|post| post.slug == slug)
        .ok_or_else(|| ContentError::PostNotFound(slug.to_string()))?;

    let mut post = posts[index].clone();
    post.previous = posts
        .get(index.saturating_sub(1))
        .filter(|_| index > 0)
        .map(to_adjacent_post);
    post.next = posts.get(index + 1).map(to_adjacent_post);

    Ok(post)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_blog_post(_slug: &str) -> Result<BlogPost, String> {
    unreachable!("get_blog_post 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn list_note_entries() -> Result<Vec<NoteSummary>> {
    let mut notes = load_note_entries()?;
    notes.sort_by(|left, right| right.date.cmp(&left.date));
    Ok(notes)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_note_entries() -> Result<Vec<NoteSummary>, String> {
    unreachable!("list_note_entries 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>> {
    Ok(load_project_entries()?)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>, String> {
    unreachable!("list_project_entries 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn build_rss_xml(site_url: &str) -> Result<String> {
    let posts = list_blog_posts().await?;
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n<channel>\n",
    );
    xml.push_str(&format!("<title>{}</title>\n", xml_escape("Wen's Field Notes")));
    xml.push_str(&format!("<link>{}</link>\n", xml_escape(site_url)));
    xml.push_str(&format!(
        "<description>{}</description>\n",
        xml_escape("一个用 Leptos SSR 和 Markdown 构建的个人内容站。")
    ));

    for post in posts {
        let link = format!("{site_url}/blog/{}", post.slug);
        xml.push_str("<item>\n");
        xml.push_str(&format!("<title>{}</title>\n", xml_escape(&post.title)));
        xml.push_str(&format!("<link>{}</link>\n", xml_escape(&link)));
        xml.push_str(&format!(
            "<description>{}</description>\n",
            xml_escape(&post.summary)
        ));
        xml.push_str(&format!(
            "<pubDate>{}</pubDate>\n",
            post.date
                .and_hms_opt(0, 0, 0)
                .expect("日期应合法")
                .format("%a, %d %b %Y %H:%M:%S GMT")
        ));
        xml.push_str("</item>\n");
    }

    xml.push_str("</channel>\n</rss>\n");
    Ok(xml)
}

#[cfg(feature = "ssr")]
pub async fn build_sitemap_xml(site_url: &str) -> Result<String> {
    let mut urls = vec![
        format!("{site_url}/"),
        format!("{site_url}/blog"),
        format!("{site_url}/notes"),
        format!("{site_url}/projects"),
        format!("{site_url}/about"),
    ];

    for post in list_blog_posts().await? {
        urls.push(format!("{site_url}/blog/{}", post.slug));
    }

    let mut xml =
        String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    for url in urls {
        xml.push_str("<url>\n");
        xml.push_str(&format!("<loc>{}</loc>\n", xml_escape(&url)));
        xml.push_str("</url>\n");
    }

    xml.push_str("</urlset>\n");
    Ok(xml)
}

#[cfg(feature = "ssr")]
fn load_blog_posts() -> Result<Vec<BlogPost>> {
    let blog_dir = Path::new(BLOG_DIR);
    if !blog_dir.exists() {
        return Err(ContentError::MissingDirectory("博客", BLOG_DIR.to_string()).into());
    }

    let mut posts = Vec::new();
    for entry in fs::read_dir(blog_dir).with_context(|| format!("读取目录失败：{BLOG_DIR}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取文章失败：{}", path.display()))?;
        posts.push(parse_blog_post(&path, &raw)?);
    }

    Ok(posts)
}

#[cfg(feature = "ssr")]
fn load_note_entries() -> Result<Vec<NoteSummary>> {
    let notes_dir = Path::new(NOTES_DIR);
    if !notes_dir.exists() {
        return Err(ContentError::MissingDirectory("笔记", NOTES_DIR.to_string()).into());
    }

    let mut notes = Vec::new();
    for entry in fs::read_dir(notes_dir).with_context(|| format!("读取目录失败：{NOTES_DIR}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取笔记失败：{}", path.display()))?;
        notes.push(parse_note_summary(&path, &raw)?);
    }

    Ok(notes)
}

#[cfg(feature = "ssr")]
fn load_project_entries() -> Result<Vec<ProjectSummary>> {
    let projects_dir = Path::new(PROJECTS_DIR);
    if !projects_dir.exists() {
        return Err(ContentError::MissingDirectory("项目", PROJECTS_DIR.to_string()).into());
    }

    let mut projects = Vec::new();
    for entry in fs::read_dir(projects_dir).with_context(|| format!("读取目录失败：{PROJECTS_DIR}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取项目内容失败：{}", path.display()))?;
        projects.push(parse_project_summary(&path, &raw)?);
    }

    Ok(projects)
}

#[cfg(feature = "ssr")]
fn parse_blog_post(path: &Path, raw: &str) -> Result<BlogPost> {
    let (slug, front_matter_raw, markdown_raw) = split_markdown_file(path, raw, "博客")?;
    let front_matter: BlogFrontMatter = serde_yaml::from_str(front_matter_raw)
        .with_context(|| format!("front matter 解析失败：{}", path.display()))?;

    Ok(BlogPost {
        slug,
        title: front_matter.title,
        summary: front_matter.summary,
        date: front_matter.date,
        tags: front_matter.tags,
        html: render_markdown(markdown_raw.trim()),
        previous: None,
        next: None,
    })
}

#[cfg(feature = "ssr")]
fn parse_note_summary(path: &Path, raw: &str) -> Result<NoteSummary> {
    let (slug, front_matter_raw, _) = split_markdown_file(path, raw, "笔记")?;
    let front_matter: NoteFrontMatter = serde_yaml::from_str(front_matter_raw)
        .with_context(|| format!("front matter 解析失败：{}", path.display()))?;

    Ok(NoteSummary {
        slug,
        title: front_matter.title,
        summary: front_matter.summary,
        date: front_matter.date,
        tags: front_matter.tags,
    })
}

#[cfg(feature = "ssr")]
fn parse_project_summary(path: &Path, raw: &str) -> Result<ProjectSummary> {
    let (slug, front_matter_raw, _) = split_markdown_file(path, raw, "项目")?;
    let front_matter: ProjectFrontMatter = serde_yaml::from_str(front_matter_raw)
        .with_context(|| format!("front matter 解析失败：{}", path.display()))?;

    Ok(ProjectSummary {
        slug,
        title: front_matter.title,
        summary: front_matter.summary,
        status: front_matter.status,
        stack: front_matter.stack,
        repo_url: front_matter.repo_url,
        live_url: front_matter.live_url,
    })
}

#[cfg(feature = "ssr")]
fn split_markdown_file<'a>(
    path: &Path,
    raw: &'a str,
    kind: &'static str,
) -> Result<(String, &'a str, &'a str)> {
    let slug = path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!(ContentError::ParseFailure(kind, format!("无法解析文件名：{}", path.display()))))?
        .to_string();

    let Some(stripped) = raw.strip_prefix("---\n") else {
        return Err(ContentError::ParseFailure(kind, format!("front matter 缺失：{}", path.display())).into());
    };
    let Some((front_matter_raw, markdown_raw)) = stripped.split_once("\n---\n") else {
        return Err(ContentError::ParseFailure(kind, format!("front matter 结尾缺失：{}", path.display())).into());
    };

    Ok((slug, front_matter_raw, markdown_raw))
}

#[cfg(feature = "ssr")]
fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(feature = "ssr")]
fn to_adjacent_post(post: &BlogPost) -> AdjacentPost {
    AdjacentPost {
        slug: post.slug.clone(),
        title: post.title.clone(),
        date: post.date,
    }
}

#[cfg(feature = "ssr")]
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&apos;")
}
