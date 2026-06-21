use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    anyhow::{anyhow, Context, Result},
    pulldown_cmark::{html, Options, Parser},
    std::{cmp::Reverse, fs, path::Path},
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
pub struct AdjacentNote {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub html: String,
    pub previous: Option<AdjacentNote>,
    pub next: Option<AdjacentNote>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub status: String,
    pub stack: Vec<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub html: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagArchiveItem {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
    pub href: String,
    pub content_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagArchive {
    pub tag: String,
    pub posts: Vec<TagArchiveItem>,
    pub notes: Vec<TagArchiveItem>,
    pub related_tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    pub content_type: String,
    pub title: String,
    pub summary: String,
    pub href: String,
    pub context: String,
    pub match_hint: String,
    pub keywords: Vec<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum ContentError {
    #[error("{0} 内容目录不存在：{1}")]
    MissingDirectory(&'static str, String),
    #[error("没有找到 slug 为 {0} 的文章")]
    PostNotFound(String),
    #[error("没有找到 slug 为 {0} 的笔记")]
    NoteNotFound(String),
    #[error("没有找到 slug 为 {0} 的项目")]
    ProjectNotFound(String),
    #[error("没有找到标签 {0} 对应的内容")]
    TagNotFound(String),
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

    posts.sort_by_key(|post| Reverse(post.date));

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
    posts.sort_by_key(|post| Reverse(post.date));

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
    let mut notes = load_note_entries()?
        .into_iter()
        .map(|note| NoteSummary {
            slug: note.slug,
            title: note.title,
            summary: note.summary,
            date: note.date,
            tags: note.tags,
        })
        .collect::<Vec<_>>();

    notes.sort_by_key(|note| Reverse(note.date));
    Ok(notes)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_note_entries() -> Result<Vec<NoteSummary>, String> {
    unreachable!("list_note_entries 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>> {
    Ok(load_project_entries()?
        .into_iter()
        .map(|project| ProjectSummary {
            slug: project.slug,
            title: project.title,
            summary: project.summary,
            status: project.status,
            stack: project.stack,
            repo_url: project.repo_url,
            live_url: project.live_url,
        })
        .collect())
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>, String> {
    unreachable!("list_project_entries 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_project_entry(slug: &str) -> Result<ProjectEntry> {
    let projects = load_project_entries()?;
    projects
        .into_iter()
        .find(|project| project.slug == slug)
        .ok_or_else(|| ContentError::ProjectNotFound(slug.to_string()).into())
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_project_entry(_slug: &str) -> Result<ProjectEntry, String> {
    unreachable!("get_project_entry 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_note_entry(slug: &str) -> Result<NoteEntry> {
    let mut notes = load_note_entries()?;
    notes.sort_by_key(|note| Reverse(note.date));

    let index = notes
        .iter()
        .position(|note| note.slug == slug)
        .ok_or_else(|| ContentError::NoteNotFound(slug.to_string()))?;

    let mut note = notes[index].clone();
    note.previous = notes
        .get(index.saturating_sub(1))
        .filter(|_| index > 0)
        .map(to_adjacent_note);
    note.next = notes.get(index + 1).map(to_adjacent_note);

    Ok(note)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_note_entry(_slug: &str) -> Result<NoteEntry, String> {
    unreachable!("get_note_entry 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_tag_archive(tag: &str) -> Result<TagArchive> {
    let posts = list_blog_posts().await?;
    let notes = list_note_entries().await?;
    let normalized_tag = normalize_text(tag);

    let mut post_matches = posts
        .into_iter()
        .filter(|post| {
            post.tags
                .iter()
                .any(|item| normalize_text(item) == normalized_tag)
        })
        .map(|post| TagArchiveItem {
            slug: post.slug.clone(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            date: post.date,
            tags: post.tags.clone(),
            href: format!("/blog/{}", post.slug),
            content_type: "博客".to_string(),
        })
        .collect::<Vec<_>>();

    let mut note_matches = notes
        .into_iter()
        .filter(|note| {
            note.tags
                .iter()
                .any(|item| normalize_text(item) == normalized_tag)
        })
        .map(|note| TagArchiveItem {
            slug: note.slug.clone(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            date: note.date,
            tags: note.tags.clone(),
            href: format!("/notes/{}", note.slug),
            content_type: "笔记".to_string(),
        })
        .collect::<Vec<_>>();

    post_matches.sort_by_key(|item| Reverse(item.date));
    note_matches.sort_by_key(|item| Reverse(item.date));

    if post_matches.is_empty() && note_matches.is_empty() {
        return Err(ContentError::TagNotFound(tag.to_string()).into());
    }

    let mut related_tags = post_matches
        .iter()
        .chain(note_matches.iter())
        .flat_map(|item| item.tags.iter().cloned())
        .filter(|item| normalize_text(item) != normalized_tag)
        .collect::<Vec<_>>();
    related_tags.sort();
    related_tags.dedup();

    Ok(TagArchive {
        tag: resolve_tag_display_name(tag, &post_matches, &note_matches),
        posts: post_matches,
        notes: note_matches,
        related_tags,
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_tag_archive(_tag: &str) -> Result<TagArchive, String> {
    unreachable!("get_tag_archive 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn search_content(query: &str) -> Result<Vec<SearchResult>> {
    let normalized_query = normalize_text(query);
    if normalized_query.is_empty() {
        return Ok(Vec::new());
    }

    let posts = load_blog_posts()?;
    let notes = load_note_entries()?;
    let projects = load_project_entries()?;

    let mut results = Vec::new();

    for post in posts {
        let body_text = strip_html(&post.html);
        let matches = collect_match_fields(
            &normalized_query,
            &[&post.title, &post.summary, &body_text, &post.tags.join(" ")],
        );
        if matches.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type: "博客".to_string(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            href: format!("/blog/{}", post.slug),
            context: format_meta_line(post.date, &post.tags),
            match_hint: format!("命中：{}", matches.join("、")),
            keywords: post.tags.clone(),
        });
    }

    for note in notes {
        let body_text = strip_html(&note.html);
        let matches = collect_match_fields(
            &normalized_query,
            &[&note.title, &note.summary, &body_text, &note.tags.join(" ")],
        );
        if matches.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type: "笔记".to_string(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            href: format!("/notes/{}", note.slug),
            context: format_meta_line(note.date, &note.tags),
            match_hint: format!("命中：{}", matches.join("、")),
            keywords: note.tags.clone(),
        });
    }

    for project in projects {
        let body_text = strip_html(&project.html);
        let keyword_text = project.stack.join(" ");
        let matches = collect_match_fields(
            &normalized_query,
            &[
                &project.title,
                &project.summary,
                &body_text,
                &keyword_text,
                &project.status,
            ],
        );
        if matches.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type: "项目".to_string(),
            title: project.title.clone(),
            summary: project.summary.clone(),
            href: format!("/projects/{}", project.slug),
            context: format!("{} · {}", project.status, project.stack.join(" / ")),
            match_hint: format!("命中：{}", matches.join("、")),
            keywords: project.stack.clone(),
        });
    }

    results.sort_by(|left, right| left.content_type.cmp(&right.content_type));

    Ok(results)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn search_content(_query: &str) -> Result<Vec<SearchResult>, String> {
    unreachable!("search_content 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn build_rss_xml(site_url: &str) -> Result<String> {
    let posts = list_blog_posts().await?;
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n<channel>\n",
    );
    xml.push_str(&format!(
        "<title>{}</title>\n",
        xml_escape("Wen's Field Notes")
    ));
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
        format!("{site_url}/search"),
        format!("{site_url}/about"),
    ];

    for post in list_blog_posts().await? {
        urls.push(format!("{site_url}/blog/{}", post.slug));
    }

    for note in list_note_entries().await? {
        urls.push(format!("{site_url}/notes/{}", note.slug));
    }

    for project in list_project_entries().await? {
        urls.push(format!("{site_url}/projects/{}", project.slug));
    }

    for tag in list_all_tags().await? {
        urls.push(format!("{site_url}/tags/{}", tag));
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
    for entry in fs::read_dir(blog_dir).with_context(|| format!("读取目录失败：{BLOG_DIR}"))?
    {
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
fn load_note_entries() -> Result<Vec<NoteEntry>> {
    let notes_dir = Path::new(NOTES_DIR);
    if !notes_dir.exists() {
        return Err(ContentError::MissingDirectory("笔记", NOTES_DIR.to_string()).into());
    }

    let mut notes = Vec::new();
    for entry in fs::read_dir(notes_dir).with_context(|| format!("读取目录失败：{NOTES_DIR}"))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取笔记失败：{}", path.display()))?;
        notes.push(parse_note_entry(&path, &raw)?);
    }

    Ok(notes)
}

#[cfg(feature = "ssr")]
fn load_project_entries() -> Result<Vec<ProjectEntry>> {
    let projects_dir = Path::new(PROJECTS_DIR);
    if !projects_dir.exists() {
        return Err(ContentError::MissingDirectory("项目", PROJECTS_DIR.to_string()).into());
    }

    let mut projects = Vec::new();
    for entry in
        fs::read_dir(projects_dir).with_context(|| format!("读取目录失败：{PROJECTS_DIR}"))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取项目内容失败：{}", path.display()))?;
        projects.push(parse_project_entry(&path, &raw)?);
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
fn parse_note_entry(path: &Path, raw: &str) -> Result<NoteEntry> {
    let (slug, front_matter_raw, markdown_raw) = split_markdown_file(path, raw, "笔记")?;
    let front_matter: NoteFrontMatter = serde_yaml::from_str(front_matter_raw)
        .with_context(|| format!("front matter 解析失败：{}", path.display()))?;

    Ok(NoteEntry {
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
fn parse_project_entry(path: &Path, raw: &str) -> Result<ProjectEntry> {
    let (slug, front_matter_raw, markdown_raw) = split_markdown_file(path, raw, "项目")?;
    let front_matter: ProjectFrontMatter = serde_yaml::from_str(front_matter_raw)
        .with_context(|| format!("front matter 解析失败：{}", path.display()))?;

    Ok(ProjectEntry {
        slug,
        title: front_matter.title,
        summary: front_matter.summary,
        status: front_matter.status,
        stack: front_matter.stack,
        repo_url: front_matter.repo_url,
        live_url: front_matter.live_url,
        html: render_markdown(markdown_raw.trim()),
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
        .ok_or_else(|| {
            anyhow!(ContentError::ParseFailure(
                kind,
                format!("无法解析文件名：{}", path.display())
            ))
        })?
        .to_string();

    let Some(stripped) = raw.strip_prefix("---\n") else {
        return Err(ContentError::ParseFailure(
            kind,
            format!("front matter 缺失：{}", path.display()),
        )
        .into());
    };
    let Some((front_matter_raw, markdown_raw)) = stripped.split_once("\n---\n") else {
        return Err(ContentError::ParseFailure(
            kind,
            format!("front matter 结尾缺失：{}", path.display()),
        )
        .into());
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
fn to_adjacent_note(note: &NoteEntry) -> AdjacentNote {
    AdjacentNote {
        slug: note.slug.clone(),
        title: note.title.clone(),
        date: note.date,
    }
}

#[cfg(feature = "ssr")]
async fn list_all_tags() -> Result<Vec<String>> {
    let posts = list_blog_posts().await?;
    let notes = list_note_entries().await?;
    let mut tags = posts
        .into_iter()
        .flat_map(|post| post.tags)
        .chain(notes.into_iter().flat_map(|note| note.tags))
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    Ok(tags)
}

#[cfg(feature = "ssr")]
fn resolve_tag_display_name(
    fallback: &str,
    posts: &[TagArchiveItem],
    notes: &[TagArchiveItem],
) -> String {
    posts
        .iter()
        .chain(notes.iter())
        .flat_map(|item| item.tags.iter())
        .find(|item| normalize_text(item) == normalize_text(fallback))
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

#[cfg(feature = "ssr")]
fn normalize_text(text: &str) -> String {
    text.trim().to_lowercase()
}

#[cfg(feature = "ssr")]
fn collect_match_fields(query: &str, fields: &[&str]) -> Vec<String> {
    let labels = ["标题", "摘要", "正文", "关键词", "状态"];
    fields
        .iter()
        .enumerate()
        .filter(|(_, field)| normalize_text(field).contains(query))
        .map(|(index, _)| labels.get(index).unwrap_or(&"内容").to_string())
        .collect()
}

#[cfg(feature = "ssr")]
fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
}

#[cfg(feature = "ssr")]
fn format_meta_line(date: NaiveDate, tags: &[String]) -> String {
    let date_text = date.format("%Y.%m.%d").to_string();
    let tag_text = if tags.is_empty() {
        "未分类".to_string()
    } else {
        tags.join(" · ")
    };

    format!("{date_text} · {tag_text}")
}

#[cfg(feature = "ssr")]
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&apos;")
}
