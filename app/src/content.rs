use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    anyhow::{anyhow, Context, Result},
    pulldown_cmark::{html, Options, Parser},
    std::{cmp::Reverse, collections::BTreeMap, fs, path::Path},
    thiserror::Error,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlogPostSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub series: String,
    pub reading_minutes: u16,
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
    pub series: String,
    pub reading_minutes: u16,
    pub manual_related: Vec<String>,
    pub tags: Vec<String>,
    pub html: String,
    pub previous: Option<AdjacentPost>,
    pub next: Option<AdjacentPost>,
    pub related: Vec<RelatedContentItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub stage: String,
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
    pub stage: String,
    pub source: String,
    pub experiment_state: String,
    pub tags: Vec<String>,
    pub html: String,
    pub previous: Option<AdjacentNote>,
    pub next: Option<AdjacentNote>,
    pub related: Vec<RelatedContentItem>,
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
    pub background: String,
    pub role: String,
    pub timeline: Vec<String>,
    pub outcomes: Vec<String>,
    pub retrospective: Vec<String>,
    pub stack: Vec<String>,
    pub repo_url: Option<String>,
    pub live_url: Option<String>,
    pub html: String,
    pub related: Vec<RelatedContentItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelatedContentItem {
    pub content_type: String,
    pub title: String,
    pub summary: String,
    pub href: String,
    pub context: String,
    pub reason: String,
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
pub struct TagOverviewItem {
    pub tag: String,
    pub total_count: usize,
    pub post_count: usize,
    pub note_count: usize,
    pub latest_date: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagsOverview {
    pub total_tags: usize,
    pub total_items: usize,
    pub tags: Vec<TagOverviewItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveYearGroup {
    pub year: i32,
    pub entries: Vec<TagArchiveItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveOverview {
    pub total_entries: usize,
    pub total_years: usize,
    pub years: Vec<ArchiveYearGroup>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    pub content_type_key: String,
    pub content_type: String,
    pub title: String,
    pub summary: String,
    pub href: String,
    pub context: String,
    pub match_hint: String,
    pub keywords: Vec<String>,
    pub score: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeriesPostItem {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: NaiveDate,
    pub reading_minutes: u16,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SeriesPage {
    pub slug: String,
    pub title: String,
    pub total_posts: usize,
    pub posts: Vec<SeriesPostItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeActivityItem {
    pub content_type: String,
    pub title: String,
    pub summary: String,
    pub href: String,
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeStat {
    pub label: String,
    pub value: String,
    pub detail: String,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeOverview {
    pub latest_posts: Vec<BlogPostSummary>,
    pub latest_notes: Vec<NoteSummary>,
    pub featured_project: Option<ProjectSummary>,
    pub recent_activity: Vec<HomeActivityItem>,
    pub focus_tags: Vec<String>,
    pub stats: Vec<HomeStat>,
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
    #[error("没有找到系列 {0} 对应的内容")]
    SeriesNotFound(String),
    #[error("解析 {0} 内容失败：{1}")]
    ParseFailure(&'static str, String),
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct BlogFrontMatter {
    title: String,
    summary: String,
    date: NaiveDate,
    series: String,
    reading_minutes: u16,
    tags: Vec<String>,
    related: Vec<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct NoteFrontMatter {
    title: String,
    summary: String,
    date: NaiveDate,
    stage: String,
    source: String,
    experiment_state: String,
    tags: Vec<String>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct ProjectFrontMatter {
    title: String,
    summary: String,
    status: String,
    background: String,
    role: String,
    timeline: Vec<String>,
    outcomes: Vec<String>,
    retrospective: Vec<String>,
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
            series: post.series,
            reading_minutes: post.reading_minutes,
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
    post.related = build_related_for_blog(
        &post,
        &posts,
        &load_note_entries()?,
        &load_project_entries()?,
    );

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
            stage: note.stage,
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
    let blog_posts = load_blog_posts()?;
    let notes = load_note_entries()?;
    let all_projects = projects.clone();
    projects
        .into_iter()
        .find(|project| project.slug == slug)
        .map(|mut project| {
            project.related =
                build_related_for_project(&project, &blog_posts, &notes, &all_projects);
            project
        })
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
    note.related =
        build_related_for_note(&note, &load_blog_posts()?, &notes, &load_project_entries()?);

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
pub async fn search_content(
    query: &str,
    type_filter: Option<&str>,
    tag_filter: Option<&str>,
) -> Result<Vec<SearchResult>> {
    let normalized_query = normalize_text(query);
    let normalized_tag_filter = tag_filter.map(normalize_text).unwrap_or_default();
    let normalized_type_filter = type_filter.map(normalize_text).unwrap_or_default();

    if normalized_query.is_empty() && normalized_tag_filter.is_empty() {
        return Ok(Vec::new());
    }

    let posts = load_blog_posts()?;
    let notes = load_note_entries()?;
    let projects = load_project_entries()?;

    let mut results = Vec::new();

    for post in posts {
        if !normalized_type_filter.is_empty() && normalized_type_filter != "blog" {
            continue;
        }

        if !matches_filter_tags(&post.tags, &normalized_tag_filter) {
            continue;
        }

        let body_text = strip_html(&post.html);
        let series_text = post.series.replace('-', " ");
        let (matches, score) = collect_match_fields(
            &normalized_query,
            &[
                SearchField::new("标题", &post.title, 8),
                SearchField::new("摘要", &post.summary, 5),
                SearchField::new("关键词", &post.tags.join(" "), 6),
                SearchField::new("系列", &series_text, 5),
                SearchField::new("正文", &body_text, 2),
            ],
        );
        if normalized_query.is_empty() && matches.is_empty() && normalized_tag_filter.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type_key: "blog".to_string(),
            content_type: "博客".to_string(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            href: format!("/blog/{}", post.slug),
            context: format!(
                "{} · 系列 {} · {} 分钟",
                format_meta_line(post.date, &post.tags),
                humanize_slug(&post.series),
                post.reading_minutes
            ),
            match_hint: build_match_hint(&matches, score, &normalized_tag_filter),
            keywords: enrich_keywords(post.tags.clone(), vec![humanize_slug(&post.series)]),
            score,
        });
    }

    for note in notes {
        if !normalized_type_filter.is_empty() && normalized_type_filter != "notes" {
            continue;
        }

        if !matches_filter_tags(&note.tags, &normalized_tag_filter) {
            continue;
        }

        let body_text = strip_html(&note.html);
        let (matches, score) = collect_match_fields(
            &normalized_query,
            &[
                SearchField::new("标题", &note.title, 8),
                SearchField::new("摘要", &note.summary, 5),
                SearchField::new("关键词", &note.tags.join(" "), 6),
                SearchField::new("阶段", &note.stage, 4),
                SearchField::new("来源", &note.source, 3),
                SearchField::new("实验状态", &note.experiment_state, 3),
                SearchField::new("正文", &body_text, 2),
            ],
        );
        if normalized_query.is_empty() && matches.is_empty() && normalized_tag_filter.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type_key: "notes".to_string(),
            content_type: "笔记".to_string(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            href: format!("/notes/{}", note.slug),
            context: format!(
                "{} · {} · {}",
                format_meta_line(note.date, &note.tags),
                note.stage,
                note.experiment_state
            ),
            match_hint: build_match_hint(&matches, score, &normalized_tag_filter),
            keywords: enrich_keywords(
                note.tags.clone(),
                vec![
                    note.stage.clone(),
                    note.source.clone(),
                    note.experiment_state.clone(),
                ],
            ),
            score,
        });
    }

    for project in projects {
        if !normalized_type_filter.is_empty() && normalized_type_filter != "projects" {
            continue;
        }

        if !matches_filter_tags(&project.stack, &normalized_tag_filter) {
            continue;
        }

        let body_text = strip_html(&project.html);
        let timeline_text = project.timeline.join(" ");
        let outcomes_text = project.outcomes.join(" ");
        let retrospective_text = project.retrospective.join(" ");
        let (matches, score) = collect_match_fields(
            &normalized_query,
            &[
                SearchField::new("标题", &project.title, 8),
                SearchField::new("摘要", &project.summary, 5),
                SearchField::new("关键词", &project.stack.join(" "), 6),
                SearchField::new("状态", &project.status, 4),
                SearchField::new("背景", &project.background, 3),
                SearchField::new("角色", &project.role, 3),
                SearchField::new("时间线", &timeline_text, 2),
                SearchField::new("结果", &outcomes_text, 2),
                SearchField::new("复盘", &retrospective_text, 2),
                SearchField::new("正文", &body_text, 2),
            ],
        );
        if normalized_query.is_empty() && matches.is_empty() && normalized_tag_filter.is_empty() {
            continue;
        }

        results.push(SearchResult {
            content_type_key: "projects".to_string(),
            content_type: "项目".to_string(),
            title: project.title.clone(),
            summary: project.summary.clone(),
            href: format!("/projects/{}", project.slug),
            context: format!("{} · {}", project.status, project.stack.join(" / ")),
            match_hint: build_match_hint(&matches, score, &normalized_tag_filter),
            keywords: enrich_keywords(
                project.stack.clone(),
                vec![project.status.clone(), project.role.clone()],
            ),
            score,
        });
    }

    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.content_type_key.cmp(&right.content_type_key))
            .then_with(|| left.title.cmp(&right.title))
    });

    Ok(results)
}

#[cfg(feature = "ssr")]
pub async fn get_tags_overview() -> Result<TagsOverview> {
    let posts = list_blog_posts().await?;
    let notes = list_note_entries().await?;
    let mut by_tag: BTreeMap<String, TagOverviewItem> = BTreeMap::new();

    for post in posts {
        for tag in post.tags {
            let entry = by_tag
                .entry(tag.clone())
                .or_insert_with(|| TagOverviewItem {
                    tag: tag.clone(),
                    total_count: 0,
                    post_count: 0,
                    note_count: 0,
                    latest_date: post.date,
                });
            entry.total_count += 1;
            entry.post_count += 1;
            if post.date > entry.latest_date {
                entry.latest_date = post.date;
            }
        }
    }

    for note in notes {
        for tag in note.tags {
            let entry = by_tag
                .entry(tag.clone())
                .or_insert_with(|| TagOverviewItem {
                    tag: tag.clone(),
                    total_count: 0,
                    post_count: 0,
                    note_count: 0,
                    latest_date: note.date,
                });
            entry.total_count += 1;
            entry.note_count += 1;
            if note.date > entry.latest_date {
                entry.latest_date = note.date;
            }
        }
    }

    let total_items = by_tag.values().map(|item| item.total_count).sum();
    let mut tags = by_tag.into_values().collect::<Vec<_>>();
    tags.sort_by(|left, right| {
        right
            .total_count
            .cmp(&left.total_count)
            .then_with(|| right.latest_date.cmp(&left.latest_date))
            .then_with(|| left.tag.cmp(&right.tag))
    });

    Ok(TagsOverview {
        total_tags: tags.len(),
        total_items,
        tags,
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_tags_overview() -> Result<TagsOverview, String> {
    unreachable!("get_tags_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_archive_overview() -> Result<ArchiveOverview> {
    let posts = list_blog_posts().await?;
    let notes = list_note_entries().await?;
    let mut entries = posts
        .into_iter()
        .map(|post| TagArchiveItem {
            slug: post.slug.clone(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            date: post.date,
            tags: post.tags.clone(),
            href: format!("/blog/{}", post.slug),
            content_type: "博客".to_string(),
        })
        .chain(notes.into_iter().map(|note| TagArchiveItem {
            slug: note.slug.clone(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            date: note.date,
            tags: note.tags.clone(),
            href: format!("/notes/{}", note.slug),
            content_type: "笔记".to_string(),
        }))
        .collect::<Vec<_>>();

    entries.sort_by_key(|item| Reverse(item.date));

    let mut by_year: BTreeMap<i32, Vec<TagArchiveItem>> = BTreeMap::new();
    for entry in entries {
        by_year.entry(entry.date.year()).or_default().push(entry);
    }

    let total_entries = by_year.values().map(Vec::len).sum();
    let mut years = by_year
        .into_iter()
        .map(|(year, mut entries)| {
            entries.sort_by_key(|item| Reverse(item.date));
            ArchiveYearGroup { year, entries }
        })
        .collect::<Vec<_>>();
    years.sort_by(|left, right| right.year.cmp(&left.year));

    Ok(ArchiveOverview {
        total_entries,
        total_years: years.len(),
        years,
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_archive_overview() -> Result<ArchiveOverview, String> {
    unreachable!("get_archive_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_series_page(slug: &str) -> Result<SeriesPage> {
    let normalized = normalize_text(slug);
    let mut posts = load_blog_posts()?
        .into_iter()
        .filter(|post| normalize_text(&post.series) == normalized)
        .collect::<Vec<_>>();

    if posts.is_empty() {
        return Err(ContentError::SeriesNotFound(slug.to_string()).into());
    }

    posts.sort_by_key(|post| post.date);

    let posts = posts
        .into_iter()
        .map(|post| SeriesPostItem {
            slug: post.slug,
            title: post.title,
            summary: post.summary,
            date: post.date,
            reading_minutes: post.reading_minutes,
            tags: post.tags,
        })
        .collect::<Vec<_>>();

    Ok(SeriesPage {
        slug: slug.to_string(),
        title: humanize_slug(slug),
        total_posts: posts.len(),
        posts,
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_series_page(_slug: &str) -> Result<SeriesPage, String> {
    unreachable!("get_series_page 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub fn validate_content_tree() -> Result<Vec<String>> {
    let blog_posts = load_blog_posts()?;
    let notes = load_note_entries()?;
    let projects = load_project_entries()?;
    let mut errors = Vec::new();

    validate_duplicate_slugs(
        "博客",
        blog_posts.iter().map(|item| item.slug.as_str()),
        &mut errors,
    );
    validate_duplicate_slugs(
        "笔记",
        notes.iter().map(|item| item.slug.as_str()),
        &mut errors,
    );
    validate_duplicate_slugs(
        "项目",
        projects.iter().map(|item| item.slug.as_str()),
        &mut errors,
    );

    let mut known_routes = vec![
        "/".to_string(),
        "/me".to_string(),
        "/about".to_string(),
        "/blog".to_string(),
        "/notes".to_string(),
        "/projects".to_string(),
        "/tags".to_string(),
        "/archive".to_string(),
        "/search".to_string(),
    ];

    for post in &blog_posts {
        known_routes.push(format!("/blog/{}", post.slug));
        known_routes.push(format!("/series/{}", post.series));
    }
    for note in &notes {
        known_routes.push(format!("/notes/{}", note.slug));
    }
    for project in &projects {
        known_routes.push(format!("/projects/{}", project.slug));
    }

    for tag in collect_all_tags_from_loaded(&blog_posts, &notes) {
        known_routes.push(format!("/tags/{}", tag));
    }

    validate_related_slugs(&blog_posts, &mut errors);
    validate_markdown_links(BLOG_DIR, &known_routes, &mut errors)?;
    validate_markdown_links(NOTES_DIR, &known_routes, &mut errors)?;
    validate_markdown_links(PROJECTS_DIR, &known_routes, &mut errors)?;

    Ok(errors)
}

#[cfg(feature = "ssr")]
pub async fn get_home_overview() -> Result<HomeOverview> {
    let posts = list_blog_posts().await?;
    let notes = list_note_entries().await?;
    let projects = list_project_entries().await?;

    let latest_posts = posts.iter().take(3).cloned().collect::<Vec<_>>();
    let latest_notes = notes.iter().take(3).cloned().collect::<Vec<_>>();

    let mut recent_activity = posts
        .iter()
        .take(3)
        .map(|post| HomeActivityItem {
            content_type: "博客".to_string(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            href: format!("/blog/{}", post.slug),
            date: post.date,
            tags: post.tags.clone(),
        })
        .chain(notes.iter().take(3).map(|note| HomeActivityItem {
            content_type: "笔记".to_string(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            href: format!("/notes/{}", note.slug),
            date: note.date,
            tags: note.tags.clone(),
        }))
        .collect::<Vec<_>>();
    recent_activity.sort_by_key(|item| Reverse(item.date));
    recent_activity.truncate(5);

    let mut tag_counts = BTreeMap::new();
    for tag in posts
        .iter()
        .flat_map(|post| post.tags.iter())
        .chain(notes.iter().flat_map(|note| note.tags.iter()))
    {
        *tag_counts.entry(tag.clone()).or_insert(0usize) += 1;
    }

    let total_tag_count = tag_counts.len();
    let mut focus_tags = tag_counts.into_iter().collect::<Vec<_>>();
    focus_tags.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    let focus_tags = focus_tags
        .into_iter()
        .take(5)
        .map(|(tag, _)| tag)
        .collect::<Vec<_>>();

    let latest_update = recent_activity
        .first()
        .map(|item| item.date.format("%Y.%m.%d").to_string())
        .unwrap_or_else(|| "暂无".to_string());

    let featured_project = projects
        .iter()
        .find(|project| project.status.contains("进行中"))
        .cloned()
        .or_else(|| projects.first().cloned());

    let stats = vec![
        HomeStat {
            label: "博客".to_string(),
            value: posts.len().to_string(),
            detail: "已发布文章".to_string(),
            href: "/blog".to_string(),
        },
        HomeStat {
            label: "笔记".to_string(),
            value: notes.len().to_string(),
            detail: "公开笔记条目".to_string(),
            href: "/notes".to_string(),
        },
        HomeStat {
            label: "项目".to_string(),
            value: projects.len().to_string(),
            detail: "项目页面条目".to_string(),
            href: "/projects".to_string(),
        },
        HomeStat {
            label: "主题".to_string(),
            value: total_tag_count.to_string(),
            detail: "当前已使用标签".to_string(),
            href: "/search".to_string(),
        },
        HomeStat {
            label: "最近更新".to_string(),
            value: latest_update,
            detail: "来自博客与笔记".to_string(),
            href: "/me".to_string(),
        },
    ];

    Ok(HomeOverview {
        latest_posts,
        latest_notes,
        featured_project,
        recent_activity,
        focus_tags,
        stats,
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_home_overview() -> Result<HomeOverview, String> {
    unreachable!("get_home_overview 只会在服务器侧执行")
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn search_content(
    _query: &str,
    _type_filter: Option<&str>,
    _tag_filter: Option<&str>,
) -> Result<Vec<SearchResult>, String> {
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
        format!("{site_url}/me"),
        format!("{site_url}/tags"),
        format!("{site_url}/archive"),
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

    for series in list_all_series().await? {
        urls.push(format!("{site_url}/series/{}", series));
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
        series: front_matter.series,
        reading_minutes: front_matter.reading_minutes,
        manual_related: front_matter.related,
        tags: front_matter.tags,
        html: render_markdown(markdown_raw.trim()),
        previous: None,
        next: None,
        related: Vec::new(),
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
        stage: front_matter.stage,
        source: front_matter.source,
        experiment_state: front_matter.experiment_state,
        tags: front_matter.tags,
        html: render_markdown(markdown_raw.trim()),
        previous: None,
        next: None,
        related: Vec::new(),
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
        background: front_matter.background,
        role: front_matter.role,
        timeline: front_matter.timeline,
        outcomes: front_matter.outcomes,
        retrospective: front_matter.retrospective,
        stack: front_matter.stack,
        repo_url: front_matter.repo_url,
        live_url: front_matter.live_url,
        html: render_markdown(markdown_raw.trim()),
        related: Vec::new(),
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
async fn list_all_series() -> Result<Vec<String>> {
    let posts = load_blog_posts()?;
    let mut series = posts
        .into_iter()
        .map(|post| post.series)
        .collect::<Vec<_>>();
    series.sort();
    series.dedup();
    Ok(series)
}

#[cfg(feature = "ssr")]
struct SearchField<'a> {
    label: &'static str,
    value: &'a str,
    weight: i32,
}

#[cfg(feature = "ssr")]
impl<'a> SearchField<'a> {
    fn new(label: &'static str, value: &'a str, weight: i32) -> Self {
        Self {
            label,
            value,
            weight,
        }
    }
}

#[cfg(feature = "ssr")]
fn collect_match_fields(query: &str, fields: &[SearchField<'_>]) -> (Vec<String>, i32) {
    if query.is_empty() {
        return (Vec::new(), 0);
    }

    let mut labels = Vec::new();
    let mut score = 0;
    for field in fields {
        let normalized = normalize_text(field.value);
        if normalized.contains(query) {
            labels.push(field.label.to_string());
            score += field.weight;
            if normalized.starts_with(query) {
                score += 2;
            }
        }
    }

    (labels, score)
}

#[cfg(feature = "ssr")]
fn build_match_hint(matches: &[String], score: i32, tag_filter: &str) -> String {
    let mut parts = Vec::new();
    if !matches.is_empty() {
        parts.push(format!("命中：{}", matches.join("、")));
    }
    if !tag_filter.is_empty() {
        parts.push(format!("标签筛选：{}", tag_filter));
    }
    parts.push(format!("相关度：{}", score));
    parts.join(" · ")
}

#[cfg(feature = "ssr")]
fn matches_filter_tags(tags: &[String], normalized_tag_filter: &str) -> bool {
    normalized_tag_filter.is_empty()
        || tags
            .iter()
            .any(|tag| normalize_text(tag) == normalized_tag_filter)
}

#[cfg(feature = "ssr")]
fn enrich_keywords(mut base: Vec<String>, extras: Vec<String>) -> Vec<String> {
    for item in extras {
        if !item.trim().is_empty() && !base.iter().any(|existing| existing == &item) {
            base.push(item);
        }
    }
    base
}

#[cfg(feature = "ssr")]
fn build_related_for_blog(
    post: &BlogPost,
    posts: &[BlogPost],
    notes: &[NoteEntry],
    projects: &[ProjectEntry],
) -> Vec<RelatedContentItem> {
    let mut items = Vec::new();

    for candidate in posts.iter().filter(|candidate| candidate.slug != post.slug) {
        let mut score = shared_count(&post.tags, &candidate.tags) * 2;
        let mut reasons = Vec::new();
        if candidate.series == post.series {
            score += 4;
            reasons.push(format!("同系列：{}", humanize_slug(&post.series)));
        }
        if post
            .manual_related
            .iter()
            .any(|slug| normalize_text(slug) == normalize_text(&candidate.slug))
        {
            score += 5;
            reasons.push("手动关联".to_string());
        }
        if score > 0 {
            if shared_count(&post.tags, &candidate.tags) > 0 {
                reasons.push("共享标签".to_string());
            }
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "博客".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/blog/{}", candidate.slug),
                    context: format!(
                        "{} · 系列 {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        humanize_slug(&candidate.series)
                    ),
                    reason: reasons.join(" · "),
                },
            ));
        }
    }

    for candidate in notes {
        let score = shared_count(&post.tags, &candidate.tags) * 2;
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "笔记".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/notes/{}", candidate.slug),
                    context: format!(
                        "{} · {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        candidate.stage
                    ),
                    reason: "共享标签".to_string(),
                },
            ));
        }
    }

    for candidate in projects {
        let project_tags = candidate.stack.clone();
        let score = shared_count(&post.tags, &project_tags) * 2;
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "项目".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/projects/{}", candidate.slug),
                    context: format!("{} · {}", candidate.status, candidate.stack.join(" / ")),
                    reason: "共享技术主题".to_string(),
                },
            ));
        }
    }

    finalize_related(items)
}

#[cfg(feature = "ssr")]
fn build_related_for_note(
    note: &NoteEntry,
    posts: &[BlogPost],
    notes: &[NoteEntry],
    projects: &[ProjectEntry],
) -> Vec<RelatedContentItem> {
    let mut items = Vec::new();

    for candidate in posts {
        let mut score = shared_count(&note.tags, &candidate.tags) * 2;
        if normalize_text(&note.source).contains(&normalize_text(&candidate.title)) {
            score += 2;
        }
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "博客".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/blog/{}", candidate.slug),
                    context: format!(
                        "{} · 系列 {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        humanize_slug(&candidate.series)
                    ),
                    reason: "共享标签 / 来源主题".to_string(),
                },
            ));
        }
    }

    for candidate in notes.iter().filter(|candidate| candidate.slug != note.slug) {
        let mut score = shared_count(&note.tags, &candidate.tags) * 2;
        if candidate.stage == note.stage {
            score += 2;
        }
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "笔记".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/notes/{}", candidate.slug),
                    context: format!(
                        "{} · {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        candidate.stage
                    ),
                    reason: "共享标签 / 相近阶段".to_string(),
                },
            ));
        }
    }

    for candidate in projects {
        let score = shared_count(&note.tags, &candidate.stack) * 2;
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "项目".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/projects/{}", candidate.slug),
                    context: format!("{} · {}", candidate.status, candidate.stack.join(" / ")),
                    reason: "共享技术主题".to_string(),
                },
            ));
        }
    }

    finalize_related(items)
}

#[cfg(feature = "ssr")]
fn build_related_for_project(
    project: &ProjectEntry,
    posts: &[BlogPost],
    notes: &[NoteEntry],
    projects: &[ProjectEntry],
) -> Vec<RelatedContentItem> {
    let mut items = Vec::new();

    for candidate in posts {
        let score = shared_count(&project.stack, &candidate.tags) * 2;
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "博客".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/blog/{}", candidate.slug),
                    context: format!(
                        "{} · 系列 {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        humanize_slug(&candidate.series)
                    ),
                    reason: "共享技术主题".to_string(),
                },
            ));
        }
    }

    for candidate in notes {
        let score = shared_count(&project.stack, &candidate.tags) * 2;
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "笔记".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/notes/{}", candidate.slug),
                    context: format!(
                        "{} · {}",
                        format_meta_line(candidate.date, &candidate.tags),
                        candidate.stage
                    ),
                    reason: "共享技术主题".to_string(),
                },
            ));
        }
    }

    for candidate in projects
        .iter()
        .filter(|candidate| candidate.slug != project.slug)
    {
        let mut score = shared_count(&project.stack, &candidate.stack) * 2;
        if candidate.status == project.status {
            score += 1;
        }
        if score > 0 {
            items.push(scored_related_item(
                score,
                RelatedContentItem {
                    content_type: "项目".to_string(),
                    title: candidate.title.clone(),
                    summary: candidate.summary.clone(),
                    href: format!("/projects/{}", candidate.slug),
                    context: format!("{} · {}", candidate.status, candidate.stack.join(" / ")),
                    reason: "共享技术栈 / 相近状态".to_string(),
                },
            ));
        }
    }

    finalize_related(items)
}

#[cfg(feature = "ssr")]
fn scored_related_item(score: usize, item: RelatedContentItem) -> (usize, RelatedContentItem) {
    (score, item)
}

#[cfg(feature = "ssr")]
fn finalize_related(mut items: Vec<(usize, RelatedContentItem)>) -> Vec<RelatedContentItem> {
    items.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.title.cmp(&right.1.title))
    });
    items.into_iter().take(6).map(|(_, item)| item).collect()
}

#[cfg(feature = "ssr")]
fn shared_count(left: &[String], right: &[String]) -> usize {
    left.iter()
        .filter(|item| {
            right
                .iter()
                .any(|candidate| normalize_text(candidate) == normalize_text(item))
        })
        .count()
}

#[cfg(feature = "ssr")]
fn humanize_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(feature = "ssr")]
fn collect_all_tags_from_loaded(posts: &[BlogPost], notes: &[NoteEntry]) -> Vec<String> {
    let mut tags = posts
        .iter()
        .flat_map(|post| post.tags.iter().cloned())
        .chain(notes.iter().flat_map(|note| note.tags.iter().cloned()))
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    tags
}

#[cfg(feature = "ssr")]
fn validate_duplicate_slugs<'a>(
    label: &'static str,
    slugs: impl Iterator<Item = &'a str>,
    errors: &mut Vec<String>,
) {
    let mut seen = BTreeMap::new();
    for slug in slugs {
        *seen.entry(slug.to_string()).or_insert(0usize) += 1;
    }
    for (slug, count) in seen {
        if count > 1 {
            errors.push(format!("{label} slug 重复：{slug}（{count} 次）"));
        }
    }
}

#[cfg(feature = "ssr")]
fn validate_related_slugs(posts: &[BlogPost], errors: &mut Vec<String>) {
    let known = posts
        .iter()
        .map(|post| post.slug.clone())
        .collect::<Vec<_>>();
    for post in posts {
        for slug in &post.manual_related {
            if !known
                .iter()
                .any(|item| normalize_text(item) == normalize_text(slug))
            {
                errors.push(format!(
                    "博客 {} 的 related 指向不存在的 slug：{}",
                    post.slug, slug
                ));
            }
        }
    }
}

#[cfg(feature = "ssr")]
fn validate_markdown_links(
    dir: &str,
    known_routes: &[String],
    errors: &mut Vec<String>,
) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("读取目录失败：{dir}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("读取内容失败：{}", path.display()))?;
        for link in extract_markdown_links(&raw) {
            if link.starts_with("http://") || link.starts_with("https://") || link.starts_with('#')
            {
                continue;
            }
            if link.starts_with('/') && !known_routes.iter().any(|route| route == &link) {
                errors.push(format!("{} 包含未知站内链接：{}", path.display(), link));
            }
        }
    }
    Ok(())
}

#[cfg(feature = "ssr")]
fn extract_markdown_links(raw: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut cursor = 0;
    while let Some(start) = raw[cursor..].find("](") {
        let absolute_start = cursor + start + 2;
        if let Some(end) = raw[absolute_start..].find(')') {
            let link = raw[absolute_start..absolute_start + end].trim();
            if !link.is_empty() {
                links.push(link.to_string());
            }
            cursor = absolute_start + end + 1;
        } else {
            break;
        }
    }
    links
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
