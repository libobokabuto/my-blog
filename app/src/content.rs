use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use {
    anyhow::{anyhow, Context, Result},
    pulldown_cmark::{html, Options, Parser},
    redis::{AsyncCommands, Client as RedisClient},
    sqlx::{mysql::MySqlPoolOptions, MySqlPool, Row},
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
    pub board: String,
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
    pub board: String,
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
pub struct ContributionCell {
    pub date: NaiveDate,
    pub count: usize,
    pub level: u8,
    pub week_index: usize,
    pub weekday_index: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContributionMonthLabel {
    pub label: String,
    pub week_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeOverview {
    pub latest_posts: Vec<BlogPostSummary>,
    pub latest_notes: Vec<NoteSummary>,
    pub featured_project: Option<ProjectSummary>,
    pub recent_activity: Vec<HomeActivityItem>,
    pub focus_tags: Vec<String>,
    pub stats: Vec<HomeStat>,
    pub contribution_cells: Vec<ContributionCell>,
    pub contribution_months: Vec<ContributionMonthLabel>,
    pub contribution_total: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteBoardSummary {
    pub key: String,
    pub label: String,
    pub description: String,
    pub total_count: usize,
    pub latest_date: Option<NaiveDate>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentLifecycleStatus {
    Draft,
    Review,
    Published,
    Archived,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentIssueSeverity {
    Info,
    Warning,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminSummaryStat {
    pub label: String,
    pub value: String,
    pub detail: String,
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminContentTypeSummary {
    pub content_type_key: String,
    pub content_type: String,
    pub total_count: usize,
    pub published_count: usize,
    pub issue_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminContentListItem {
    pub id: String,
    pub content_type_key: String,
    pub content_type: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub status: ContentLifecycleStatus,
    pub status_label: String,
    pub source_path: String,
    pub public_href: String,
    pub admin_href: String,
    pub date: Option<NaiveDate>,
    pub primary_context: String,
    pub tags: Vec<String>,
    pub related_count: usize,
    pub issue_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminContentFact {
    pub label: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminContentIssue {
    pub severity: ContentIssueSeverity,
    pub severity_label: String,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminContentDetail {
    pub item: AdminContentListItem,
    pub facts: Vec<AdminContentFact>,
    pub related: Vec<RelatedContentItem>,
    pub issues: Vec<AdminContentIssue>,
    pub bridge_notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminDashboardOverview {
    pub stats: Vec<AdminSummaryStat>,
    pub content_types: Vec<AdminContentTypeSummary>,
    pub recent_items: Vec<AdminContentListItem>,
    pub bridge_notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchIndexDocument {
    pub document_id: String,
    pub content_type_key: String,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub body_text: String,
    pub public_href: String,
    pub source_path: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub board: Option<String>,
    pub stage: Option<String>,
    pub series: Option<String>,
    pub project_status: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchRebuildRecord {
    pub run_id: i64,
    pub trigger: String,
    pub status: String,
    pub message: String,
    pub document_count: usize,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchQueryDiagnostic {
    pub query: String,
    pub mode: String,
    pub result_count: usize,
    pub top_results: Vec<SearchResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminSearchOverview {
    pub stats: Vec<AdminSummaryStat>,
    pub rebuild_records: Vec<SearchRebuildRecord>,
    pub diagnostics: Vec<SearchQueryDiagnostic>,
    pub coverage_notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricSnapshot {
    pub id: i64,
    pub metric_key: String,
    pub metric_value: String,
    pub detail: String,
    pub captured_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminStatsOverview {
    pub stats: Vec<AdminSummaryStat>,
    pub snapshots: Vec<MetricSnapshot>,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskRunRecord {
    pub id: i64,
    pub task_type: String,
    pub trigger: String,
    pub status: String,
    pub summary: String,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminTasksOverview {
    pub stats: Vec<AdminSummaryStat>,
    pub tasks: Vec<TaskRunRecord>,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyncSourceRecord {
    pub id: i64,
    pub source_key: String,
    pub label: String,
    pub direction: String,
    pub status: String,
    pub endpoint: String,
    pub last_run_at: Option<String>,
    pub notes: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyncRunRecord {
    pub id: i64,
    pub source_key: String,
    pub trigger: String,
    pub status: String,
    pub summary: String,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminSyncOverview {
    pub stats: Vec<AdminSummaryStat>,
    pub sources: Vec<SyncSourceRecord>,
    pub runs: Vec<SyncRunRecord>,
    pub notes: Vec<String>,
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
    #[error("后台内容 ID 无效：{0}")]
    InvalidAdminContentId(String),
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
    board: String,
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
const SEARCH_INDEX_TABLE: &str = "search_documents";
#[cfg(feature = "ssr")]
const SEARCH_REBUILD_TABLE: &str = "search_rebuild_runs";
#[cfg(feature = "ssr")]
const TASK_RUNS_TABLE: &str = "task_runs";
#[cfg(feature = "ssr")]
const STATS_SNAPSHOTS_TABLE: &str = "stats_snapshots";
#[cfg(feature = "ssr")]
const SYNC_SOURCES_TABLE: &str = "sync_sources";
#[cfg(feature = "ssr")]
const SYNC_RUNS_TABLE: &str = "sync_runs";
#[cfg(feature = "ssr")]
const SEARCH_STATUS_KEY_PREFIX: &str = "my-blog:search:index";

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
            board: note.board,
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
    if let Some(results) =
        search_content_from_persisted_index(query, type_filter, tag_filter).await?
    {
        return Ok(results);
    }

    search_content_live(query, type_filter, tag_filter).await
}

#[cfg(feature = "ssr")]
async fn search_content_live(
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
                SearchField::new("板块", &note.board, 5),
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
                "{} · {} · {} · {}",
                format_meta_line(note.date, &note.tags),
                note_board_label(&note.board),
                note.stage,
                note.experiment_state
            ),
            match_hint: build_match_hint(&matches, score, &normalized_tag_filter),
            keywords: enrich_keywords(
                note.tags.clone(),
                vec![
                    note_board_label(&note.board).to_string(),
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
pub async fn get_note_boards_overview() -> Result<Vec<NoteBoardSummary>> {
    let notes = list_note_entries().await?;
    let board_order = ["rust", "cpp", "bochs", "general"];
    let mut counts = BTreeMap::<String, NoteBoardSummary>::new();

    for board_key in board_order {
        counts.insert(
            board_key.to_string(),
            NoteBoardSummary {
                key: board_key.to_string(),
                label: note_board_label(board_key).to_string(),
                description: note_board_description(board_key).to_string(),
                total_count: 0,
                latest_date: None,
            },
        );
    }

    for note in notes {
        let key = normalize_note_board(&note.board);
        let entry = counts
            .entry(key.clone())
            .or_insert_with(|| NoteBoardSummary {
                key: key.clone(),
                label: note_board_label(&key).to_string(),
                description: note_board_description(&key).to_string(),
                total_count: 0,
                latest_date: None,
            });

        entry.total_count += 1;
        entry.latest_date = Some(
            entry
                .latest_date
                .map(|current| current.max(note.date))
                .unwrap_or(note.date),
        );
    }

    let mut boards = counts.into_values().collect::<Vec<_>>();
    boards.sort_by(|left, right| {
        board_sort_order(&left.key)
            .cmp(&board_sort_order(&right.key))
            .then_with(|| left.key.cmp(&right.key))
    });

    Ok(boards)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_note_boards_overview() -> Result<Vec<NoteBoardSummary>, String> {
    unreachable!("get_note_boards_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_admin_search_overview(sample_query: &str) -> Result<AdminSearchOverview> {
    let runtime = load_search_runtime().await;
    let index_docs = build_search_index_documents().await?;
    let persisted_count = if let Some(pool) = runtime.db_pool.as_ref() {
        ensure_search_storage(pool).await?;
        sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {SEARCH_INDEX_TABLE}"))
            .fetch_one(pool)
            .await
            .unwrap_or_default() as usize
    } else {
        0
    };

    let rebuild_records = if let Some(pool) = runtime.db_pool.as_ref() {
        load_search_rebuild_records(pool).await?
    } else {
        Vec::new()
    };

    let diagnostics = build_search_diagnostics(sample_query, runtime.db_pool.as_ref()).await?;
    let boards = collect_index_board_coverage(&index_docs);
    let board_text = if boards.is_empty() {
        "暂无板块".to_string()
    } else {
        boards.join(" / ")
    };
    let stats = vec![
        AdminSummaryStat {
            label: "MySQL".to_string(),
            value: runtime.db_status_label().to_string(),
            detail: runtime.db_status_detail().to_string(),
            href: "/admin/search".to_string(),
        },
        AdminSummaryStat {
            label: "Redis".to_string(),
            value: runtime.redis_status_label().to_string(),
            detail: runtime.redis_status_detail().to_string(),
            href: "/admin/search".to_string(),
        },
        AdminSummaryStat {
            label: "索引文档".to_string(),
            value: persisted_count.to_string(),
            detail: format!("当前从内容源可构建 {} 条文档。", index_docs.len()),
            href: "/admin/search".to_string(),
        },
    ];

    Ok(AdminSearchOverview {
        stats,
        rebuild_records,
        diagnostics,
        coverage_notes: vec![
            "当前搜索索引以 MySQL 存储搜索文档与重建记录。".to_string(),
            "Redis 当前用于记录重建状态与轻量运行态信息。".to_string(),
            format!("当前索引覆盖的笔记板块：{}。", board_text),
            "如果 MySQL 未启动，前台 `/search` 会回退到第三版内存聚合搜索。".to_string(),
        ],
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_search_overview(_sample_query: &str) -> Result<AdminSearchOverview, String> {
    unreachable!("get_admin_search_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_admin_stats_overview() -> Result<AdminStatsOverview> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        return Ok(AdminStatsOverview {
            stats: vec![AdminSummaryStat {
                label: "MySQL".to_string(),
                value: "disconnected".to_string(),
                detail: runtime.db_status_detail().to_string(),
                href: "/admin/stats".to_string(),
            }],
            snapshots: Vec::new(),
            notes: vec![
                "统计快照依赖 MySQL。".to_string(),
                "当前数据库未连通，所以这里只能显示空状态。".to_string(),
            ],
        });
    };

    ensure_search_storage(pool).await?;
    ensure_operations_storage(pool).await?;
    capture_stats_snapshot(
        pool,
        "content_total",
        count_content_items().await?.to_string(),
        "当前内容总量",
    )
    .await?;
    capture_stats_snapshot(
        pool,
        "search_documents",
        count_search_documents(pool).await?.to_string(),
        "当前搜索文档数",
    )
    .await?;
    capture_stats_snapshot(
        pool,
        "task_runs",
        count_table_rows(pool, TASK_RUNS_TABLE).await?.to_string(),
        "任务记录总数",
    )
    .await?;
    capture_stats_snapshot(
        pool,
        "sync_runs",
        count_table_rows(pool, SYNC_RUNS_TABLE).await?.to_string(),
        "同步运行记录总数",
    )
    .await?;

    let snapshots = load_metric_snapshots(pool).await?;
    let task_total = count_table_rows(pool, TASK_RUNS_TABLE).await?;
    let sync_total = count_table_rows(pool, SYNC_RUNS_TABLE).await?;
    let stats = vec![
        AdminSummaryStat {
            label: "内容总量".to_string(),
            value: count_content_items().await?.to_string(),
            detail: "按当前 Markdown 内容源聚合得出的统一内容数量。".to_string(),
            href: "/admin/content".to_string(),
        },
        AdminSummaryStat {
            label: "任务记录".to_string(),
            value: task_total.to_string(),
            detail: "当前统一任务表中已记录的运行次数。".to_string(),
            href: "/admin/tasks".to_string(),
        },
        AdminSummaryStat {
            label: "同步记录".to_string(),
            value: sync_total.to_string(),
            detail: "当前同步边界产生的最近运行记录。".to_string(),
            href: "/admin/sync".to_string(),
        },
    ];

    Ok(AdminStatsOverview {
        stats,
        snapshots,
        notes: vec![
            "第四版当前统计更偏后台治理指标，不是公开流量看板。".to_string(),
            "后续可以把访问量、热门内容、时间窗口分析继续接进这里。".to_string(),
        ],
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_stats_overview() -> Result<AdminStatsOverview, String> {
    unreachable!("get_admin_stats_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_admin_tasks_overview() -> Result<AdminTasksOverview> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        return Ok(AdminTasksOverview {
            stats: vec![AdminSummaryStat {
                label: "MySQL".to_string(),
                value: "disconnected".to_string(),
                detail: runtime.db_status_detail().to_string(),
                href: "/admin/tasks".to_string(),
            }],
            tasks: Vec::new(),
            notes: vec!["任务系统依赖 MySQL 持久化。".to_string()],
        });
    };

    ensure_operations_storage(pool).await?;
    let tasks = load_task_runs(pool).await?;
    let success_count = tasks.iter().filter(|task| task.status == "success").count();
    let failed_count = tasks.iter().filter(|task| task.status == "failed").count();

    Ok(AdminTasksOverview {
        stats: vec![
            AdminSummaryStat {
                label: "任务总数".to_string(),
                value: tasks.len().to_string(),
                detail: "统一任务表中的最近运行记录。".to_string(),
                href: "/admin/tasks".to_string(),
            },
            AdminSummaryStat {
                label: "成功".to_string(),
                value: success_count.to_string(),
                detail: "最近任务中标记为 success 的数量。".to_string(),
                href: "/admin/tasks".to_string(),
            },
            AdminSummaryStat {
                label: "失败".to_string(),
                value: failed_count.to_string(),
                detail: "最近任务中标记为 failed 的数量。".to_string(),
                href: "/admin/tasks".to_string(),
            },
        ],
        tasks,
        notes: vec![
            "搜索重建和同步运行都会写入统一任务表。".to_string(),
            "第四版当前还没有独立 worker，这里先记录服务端内触发的任务。".to_string(),
        ],
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_tasks_overview() -> Result<AdminTasksOverview, String> {
    unreachable!("get_admin_tasks_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_admin_sync_overview() -> Result<AdminSyncOverview> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        return Ok(AdminSyncOverview {
            stats: vec![AdminSummaryStat {
                label: "MySQL".to_string(),
                value: "disconnected".to_string(),
                detail: runtime.db_status_detail().to_string(),
                href: "/admin/sync".to_string(),
            }],
            sources: Vec::new(),
            runs: Vec::new(),
            notes: vec!["同步边界依赖 MySQL 持久化。".to_string()],
        });
    };

    ensure_operations_storage(pool).await?;
    seed_sync_sources(pool).await?;
    let sources = load_sync_sources(pool).await?;
    let runs = load_sync_runs(pool).await?;

    Ok(AdminSyncOverview {
        stats: vec![
            AdminSummaryStat {
                label: "同步源".to_string(),
                value: sources.len().to_string(),
                detail: "当前已登记的外部同步源数量。".to_string(),
                href: "/admin/sync".to_string(),
            },
            AdminSummaryStat {
                label: "同步记录".to_string(),
                value: runs.len().to_string(),
                detail: "最近同步运行记录。".to_string(),
                href: "/admin/sync".to_string(),
            },
            AdminSummaryStat {
                label: "Redis".to_string(),
                value: runtime.redis_status_label().to_string(),
                detail: runtime.redis_status_detail().to_string(),
                href: "/admin/sync".to_string(),
            },
        ],
        sources,
        runs,
        notes: vec![
            "第四版当前只建立外部同步边界，不接入真实第三方 OAuth。".to_string(),
            "后续可以在这些 source_key 上继续接 GitHub、RSS 或本地导入器。".to_string(),
        ],
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_sync_overview() -> Result<AdminSyncOverview, String> {
    unreachable!("get_admin_sync_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn run_sync_source(source_key: &str, trigger: &str) -> Result<SyncRunRecord> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        return Err(anyhow!("MySQL 当前不可用，无法记录同步运行。"));
    };

    ensure_operations_storage(pool).await?;
    seed_sync_sources(pool).await?;

    let task_id =
        create_task_run(pool, "sync", trigger, &format!("开始同步源 {source_key}")).await?;
    let run_result = sqlx::query(&format!(
        "INSERT INTO {SYNC_RUNS_TABLE} (source_key, `trigger`, status, summary, started_at) VALUES (?, ?, 'running', ?, NOW())"
    ))
    .bind(source_key)
    .bind(trigger)
    .bind("同步边界占位运行")
    .execute(pool)
    .await?;
    let run_id = run_result.last_insert_id() as i64;

    let summary =
        format!("已记录同步源 {source_key} 的一次占位运行。当前第四版还没有接入真实外部 API。");
    sqlx::query(&format!(
        "UPDATE {SYNC_RUNS_TABLE} SET status='success', summary=?, finished_at=NOW() WHERE id=?"
    ))
    .bind(&summary)
    .bind(run_id)
    .execute(pool)
    .await?;
    sqlx::query(&format!(
        "UPDATE {SYNC_SOURCES_TABLE} SET last_run_at=NOW(), status='healthy' WHERE source_key=?"
    ))
    .bind(source_key)
    .execute(pool)
    .await?;

    finish_task_run(pool, task_id, "success", &summary).await?;

    Ok(SyncRunRecord {
        id: run_id,
        source_key: source_key.to_string(),
        trigger: trigger.to_string(),
        status: "success".to_string(),
        summary,
        started_at: Utc::now().to_rfc3339(),
        finished_at: Some(Utc::now().to_rfc3339()),
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn run_sync_source(_source_key: &str, _trigger: &str) -> Result<SyncRunRecord, String> {
    unreachable!("run_sync_source 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn rebuild_search_index(trigger: &str) -> Result<SearchRebuildRecord> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        let record = SearchRebuildRecord {
            run_id: 0,
            trigger: trigger.to_string(),
            status: "failed".to_string(),
            message: runtime
                .db_error
                .unwrap_or_else(|| "MySQL 当前不可用，无法持久化搜索索引。".to_string()),
            document_count: 0,
            started_at: Utc::now().to_rfc3339(),
            finished_at: Some(Utc::now().to_rfc3339()),
        };
        write_search_runtime_state(
            runtime.redis_client.as_ref(),
            "failed",
            trigger,
            &record.message,
            0,
        )
        .await;
        return Ok(record);
    };

    ensure_search_storage(pool).await?;
    ensure_operations_storage(pool).await?;
    let started_at = Utc::now();
    let task_id = create_task_run(pool, "search_rebuild", trigger, "搜索索引开始重建").await?;
    let run_result = sqlx::query(&format!(
        "INSERT INTO {SEARCH_REBUILD_TABLE} (`trigger`, status, message, document_count, started_at) VALUES (?, 'running', ?, 0, NOW())"
    ))
    .bind(trigger)
    .bind("搜索索引开始重建")
    .execute(pool)
    .await?;
    let run_id = run_result.last_insert_id() as i64;

    let docs = build_search_index_documents().await?;
    let mut tx = pool.begin().await?;
    sqlx::query(&format!("DELETE FROM {SEARCH_INDEX_TABLE}"))
        .execute(&mut *tx)
        .await?;

    for doc in &docs {
        sqlx::query(&format!(
            "INSERT INTO {SEARCH_INDEX_TABLE} (document_id, content_type_key, slug, public_href, title, summary, body_text, source_path, tags, keywords, board, stage, series, project_status, updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
        ))
        .bind(&doc.document_id)
        .bind(&doc.content_type_key)
        .bind(&doc.slug)
        .bind(&doc.public_href)
        .bind(&doc.title)
        .bind(&doc.summary)
        .bind(&doc.body_text)
        .bind(&doc.source_path)
        .bind(encode_string_list(&doc.tags))
        .bind(encode_string_list(&doc.keywords))
        .bind(&doc.board)
        .bind(&doc.stage)
        .bind(&doc.series)
        .bind(&doc.project_status)
        .bind(doc.updated_at)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    let message = format!("已重建 {} 条搜索文档。", docs.len());
    sqlx::query(&format!(
        "UPDATE {SEARCH_REBUILD_TABLE} SET status = 'success', message = ?, document_count = ?, finished_at = NOW() WHERE id = ?"
    ))
    .bind(&message)
    .bind(docs.len() as i32)
    .bind(run_id)
    .execute(pool)
    .await?;

    write_search_runtime_state(
        runtime.redis_client.as_ref(),
        "success",
        trigger,
        &message,
        docs.len(),
    )
    .await;
    finish_task_run(pool, task_id, "success", &message).await?;

    Ok(SearchRebuildRecord {
        run_id,
        trigger: trigger.to_string(),
        status: "success".to_string(),
        message,
        document_count: docs.len(),
        started_at: started_at.to_rfc3339(),
        finished_at: Some(Utc::now().to_rfc3339()),
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn rebuild_search_index(_trigger: &str) -> Result<SearchRebuildRecord, String> {
    unreachable!("rebuild_search_index 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
async fn search_content_from_persisted_index(
    query: &str,
    type_filter: Option<&str>,
    tag_filter: Option<&str>,
) -> Result<Option<Vec<SearchResult>>> {
    let runtime = load_search_runtime().await;
    let Some(pool) = runtime.db_pool.as_ref() else {
        return Ok(None);
    };

    ensure_search_storage(pool).await?;
    let total_count =
        sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {SEARCH_INDEX_TABLE}"))
            .fetch_one(pool)
            .await
            .unwrap_or_default();
    if total_count == 0 {
        return Ok(None);
    }

    let normalized_query = normalize_text(query);
    let normalized_type_filter = type_filter.map(normalize_text).unwrap_or_default();
    let normalized_tag_filter = tag_filter.map(normalize_text).unwrap_or_default();
    if normalized_query.is_empty() && normalized_tag_filter.is_empty() {
        return Ok(Some(Vec::new()));
    }

    let rows = sqlx::query(&format!(
        "SELECT content_type_key, slug, public_href, title, summary, tags, keywords, board, stage, series, project_status FROM {SEARCH_INDEX_TABLE}"
    ))
    .fetch_all(pool)
    .await?;

    let mut results = Vec::new();

    for row in rows {
        let content_type_key: String = row.try_get("content_type_key")?;
        if !normalized_type_filter.is_empty()
            && normalize_text(&content_type_key) != normalized_type_filter
        {
            continue;
        }

        let tags = decode_string_list(&row.try_get::<String, _>("tags")?);
        if !matches_filter_tags(&tags, &normalized_tag_filter) {
            continue;
        }

        let title: String = row.try_get("title")?;
        let summary: String = row.try_get("summary")?;
        let keywords = decode_string_list(&row.try_get::<String, _>("keywords")?);
        let keyword_text = keywords.join(" ");
        let board: Option<String> = row.try_get("board")?;
        let stage: Option<String> = row.try_get("stage")?;
        let series: Option<String> = row.try_get("series")?;
        let project_status: Option<String> = row.try_get("project_status")?;
        let href: String = row.try_get("public_href")?;

        let mut match_fields = vec![
            SearchField::new("标题", &title, 8),
            SearchField::new("摘要", &summary, 5),
            SearchField::new("关键词", &keyword_text, 6),
        ];
        if let Some(board) = board.as_ref() {
            match_fields.push(SearchField::new("板块", board, 5));
        }
        if let Some(stage) = stage.as_ref() {
            match_fields.push(SearchField::new("阶段", stage, 4));
        }
        if let Some(series) = series.as_ref() {
            match_fields.push(SearchField::new("系列", series, 4));
        }
        if let Some(project_status) = project_status.as_ref() {
            match_fields.push(SearchField::new("状态", project_status, 4));
        }

        let (matches, score) = collect_match_fields(&normalized_query, &match_fields);
        if normalized_query.is_empty() && matches.is_empty() && normalized_tag_filter.is_empty() {
            continue;
        }

        let context = match content_type_key.as_str() {
            "blog" => {
                let series_text = series
                    .as_ref()
                    .map(|value| humanize_slug(value))
                    .unwrap_or_else(|| "未分类系列".to_string());
                format!("博客 · 系列 {series_text}")
            }
            "notes" => {
                let board_text = board
                    .as_ref()
                    .map(|value| note_board_label(value).to_string())
                    .unwrap_or_else(|| "未分类板块".to_string());
                let stage_text = stage.clone().unwrap_or_else(|| "未标注阶段".to_string());
                format!("笔记 · {board_text} · {stage_text}")
            }
            "projects" => format!(
                "项目 · {}",
                project_status.unwrap_or_else(|| "未标注状态".to_string())
            ),
            _ => content_type_key.clone(),
        };

        results.push(SearchResult {
            content_type_key: content_type_key.clone(),
            content_type: display_content_type(&content_type_key).to_string(),
            title,
            summary,
            href,
            context,
            match_hint: build_match_hint(&matches, score, &normalized_tag_filter),
            keywords: enrich_keywords(tags.clone(), keywords),
            score: score + 1,
        });
    }

    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.content_type_key.cmp(&right.content_type_key))
            .then_with(|| left.title.cmp(&right.title))
    });

    Ok(Some(results))
}

#[cfg(feature = "ssr")]
async fn build_search_diagnostics(
    sample_query: &str,
    pool: Option<&MySqlPool>,
) -> Result<Vec<SearchQueryDiagnostic>> {
    let queries = if sample_query.trim().is_empty() {
        vec!["Rust".to_string(), "PRD".to_string(), "Leptos".to_string()]
    } else {
        vec![sample_query.trim().to_string()]
    };

    let mut diagnostics = Vec::new();
    for query in queries {
        let (mode, results) = if pool.is_some() {
            match search_content_from_persisted_index(&query, None, None).await? {
                Some(results) => ("persisted-index".to_string(), results),
                None => (
                    "live-fallback".to_string(),
                    search_content_live(&query, None, None).await?,
                ),
            }
        } else {
            (
                "live-fallback".to_string(),
                search_content_live(&query, None, None).await?,
            )
        };

        diagnostics.push(SearchQueryDiagnostic {
            query,
            result_count: results.len(),
            top_results: results.into_iter().take(3).collect(),
            mode,
        });
    }

    Ok(diagnostics)
}

#[cfg(feature = "ssr")]
async fn build_search_index_documents() -> Result<Vec<SearchIndexDocument>> {
    let mut docs = Vec::new();
    let now = Utc::now();

    for post in load_blog_posts()? {
        docs.push(SearchIndexDocument {
            document_id: build_admin_content_id("blog", &post.slug),
            content_type_key: "blog".to_string(),
            slug: post.slug.clone(),
            title: post.title.clone(),
            summary: post.summary.clone(),
            body_text: strip_html(&post.html),
            public_href: format!("/blog/{}", post.slug),
            source_path: format!("{BLOG_DIR}/{}.md", post.slug),
            tags: post.tags.clone(),
            keywords: enrich_keywords(post.tags.clone(), vec![humanize_slug(&post.series)]),
            board: None,
            stage: None,
            series: Some(post.series.clone()),
            project_status: None,
            updated_at: now,
        });
    }

    for note in load_note_entries()? {
        docs.push(SearchIndexDocument {
            document_id: build_admin_content_id("notes", &note.slug),
            content_type_key: "notes".to_string(),
            slug: note.slug.clone(),
            title: note.title.clone(),
            summary: note.summary.clone(),
            body_text: strip_html(&note.html),
            public_href: format!("/notes/{}", note.slug),
            source_path: format!("{NOTES_DIR}/{}.md", note.slug),
            tags: note.tags.clone(),
            keywords: enrich_keywords(
                note.tags.clone(),
                vec![
                    note_board_label(&note.board).to_string(),
                    note.stage.clone(),
                    note.source.clone(),
                    note.experiment_state.clone(),
                ],
            ),
            board: Some(normalize_note_board(&note.board)),
            stage: Some(note.stage.clone()),
            series: None,
            project_status: None,
            updated_at: now,
        });
    }

    for project in load_project_entries()? {
        docs.push(SearchIndexDocument {
            document_id: build_admin_content_id("projects", &project.slug),
            content_type_key: "projects".to_string(),
            slug: project.slug.clone(),
            title: project.title.clone(),
            summary: project.summary.clone(),
            body_text: strip_html(&project.html),
            public_href: format!("/projects/{}", project.slug),
            source_path: format!("{PROJECTS_DIR}/{}.md", project.slug),
            tags: project.stack.clone(),
            keywords: enrich_keywords(
                project.stack.clone(),
                vec![project.status.clone(), project.role.clone()],
            ),
            board: None,
            stage: None,
            series: None,
            project_status: Some(project.status.clone()),
            updated_at: now,
        });
    }

    Ok(docs)
}

#[cfg(feature = "ssr")]
async fn ensure_search_storage(pool: &MySqlPool) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {SEARCH_INDEX_TABLE} (
            document_id VARCHAR(191) PRIMARY KEY,
            content_type_key VARCHAR(64) NOT NULL,
            slug VARCHAR(191) NOT NULL,
            public_href VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            summary TEXT NOT NULL,
            body_text LONGTEXT NOT NULL,
            source_path VARCHAR(255) NOT NULL,
            tags LONGTEXT NOT NULL,
            keywords LONGTEXT NOT NULL,
            board VARCHAR(64) NULL,
            stage VARCHAR(64) NULL,
            series VARCHAR(191) NULL,
            project_status VARCHAR(64) NULL,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {SEARCH_REBUILD_TABLE} (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            `trigger` VARCHAR(64) NOT NULL,
            status VARCHAR(32) NOT NULL,
            message TEXT NOT NULL,
            document_count INT NOT NULL DEFAULT 0,
            started_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            finished_at DATETIME NULL
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE INDEX idx_search_documents_type ON {SEARCH_INDEX_TABLE} (content_type_key)"
    ))
    .execute(pool)
    .await
    .ok();
    sqlx::query(&format!(
        "CREATE INDEX idx_search_documents_board ON {SEARCH_INDEX_TABLE} (board)"
    ))
    .execute(pool)
    .await
    .ok();
    sqlx::query(&format!(
        "CREATE INDEX idx_search_documents_stage ON {SEARCH_INDEX_TABLE} (stage)"
    ))
    .execute(pool)
    .await
    .ok();

    Ok(())
}

#[cfg(feature = "ssr")]
async fn ensure_operations_storage(pool: &MySqlPool) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {TASK_RUNS_TABLE} (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            task_type VARCHAR(64) NOT NULL,
            `trigger` VARCHAR(64) NOT NULL,
            status VARCHAR(32) NOT NULL,
            summary TEXT NOT NULL,
            started_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            finished_at DATETIME NULL
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {STATS_SNAPSHOTS_TABLE} (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            metric_key VARCHAR(64) NOT NULL UNIQUE,
            metric_value VARCHAR(255) NOT NULL,
            detail VARCHAR(255) NOT NULL,
            captured_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {SYNC_SOURCES_TABLE} (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            source_key VARCHAR(64) NOT NULL UNIQUE,
            label VARCHAR(128) NOT NULL,
            direction VARCHAR(32) NOT NULL,
            status VARCHAR(32) NOT NULL,
            endpoint VARCHAR(255) NOT NULL,
            last_run_at DATETIME NULL,
            notes TEXT NOT NULL
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {SYNC_RUNS_TABLE} (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            source_key VARCHAR(64) NOT NULL,
            `trigger` VARCHAR(64) NOT NULL,
            status VARCHAR(32) NOT NULL,
            summary TEXT NOT NULL,
            started_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            finished_at DATETIME NULL
        )"
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE INDEX idx_task_runs_type ON {TASK_RUNS_TABLE} (task_type, started_at)"
    ))
    .execute(pool)
    .await
    .ok();
    sqlx::query(&format!(
        "CREATE INDEX idx_sync_runs_source ON {SYNC_RUNS_TABLE} (source_key, started_at)"
    ))
    .execute(pool)
    .await
    .ok();

    Ok(())
}

#[cfg(feature = "ssr")]
async fn count_table_rows(pool: &MySqlPool, table_name: &str) -> Result<i64> {
    let sql = format!("SELECT COUNT(*) FROM {table_name}");
    Ok(sqlx::query_scalar::<_, i64>(&sql)
        .fetch_one(pool)
        .await
        .unwrap_or_default())
}

#[cfg(feature = "ssr")]
async fn count_search_documents(pool: &MySqlPool) -> Result<i64> {
    count_table_rows(pool, SEARCH_INDEX_TABLE).await
}

#[cfg(feature = "ssr")]
async fn count_content_items() -> Result<usize> {
    Ok(load_blog_posts()?.len() + load_note_entries()?.len() + load_project_entries()?.len())
}

#[cfg(feature = "ssr")]
async fn capture_stats_snapshot(
    pool: &MySqlPool,
    metric_key: &str,
    metric_value: String,
    detail: &str,
) -> Result<()> {
    sqlx::query(&format!(
        "INSERT INTO {STATS_SNAPSHOTS_TABLE} (metric_key, metric_value, detail, captured_at)
         VALUES (?, ?, ?, NOW())
         ON DUPLICATE KEY UPDATE metric_value = VALUES(metric_value), detail = VALUES(detail), captured_at = NOW()"
    ))
    .bind(metric_key)
    .bind(metric_value)
    .bind(detail)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn load_metric_snapshots(pool: &MySqlPool) -> Result<Vec<MetricSnapshot>> {
    let rows = sqlx::query(&format!(
        "SELECT id, metric_key, metric_value, detail,
         DATE_FORMAT(captured_at, '%Y-%m-%dT%H:%i:%sZ') AS captured_at_text
         FROM {STATS_SNAPSHOTS_TABLE}
         ORDER BY captured_at DESC, id DESC
         LIMIT 12"
    ))
    .fetch_all(pool)
    .await?;

    let mut snapshots = Vec::new();
    for row in rows {
        snapshots.push(MetricSnapshot {
            id: row.try_get("id")?,
            metric_key: row.try_get("metric_key")?,
            metric_value: row.try_get("metric_value")?,
            detail: row.try_get("detail")?,
            captured_at: row.try_get("captured_at_text")?,
        });
    }

    Ok(snapshots)
}

#[cfg(feature = "ssr")]
async fn create_task_run(
    pool: &MySqlPool,
    task_type: &str,
    trigger: &str,
    summary: &str,
) -> Result<i64> {
    let result = sqlx::query(&format!(
        "INSERT INTO {TASK_RUNS_TABLE} (task_type, `trigger`, status, summary, started_at)
         VALUES (?, ?, 'running', ?, NOW())"
    ))
    .bind(task_type)
    .bind(trigger)
    .bind(summary)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

#[cfg(feature = "ssr")]
async fn finish_task_run(
    pool: &MySqlPool,
    task_id: i64,
    status: &str,
    summary: &str,
) -> Result<()> {
    sqlx::query(&format!(
        "UPDATE {TASK_RUNS_TABLE} SET status = ?, summary = ?, finished_at = NOW() WHERE id = ?"
    ))
    .bind(status)
    .bind(summary)
    .bind(task_id)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn load_task_runs(pool: &MySqlPool) -> Result<Vec<TaskRunRecord>> {
    let rows = sqlx::query(&format!(
        "SELECT id, task_type, `trigger`, status, summary,
         DATE_FORMAT(started_at, '%Y-%m-%dT%H:%i:%sZ') AS started_at_text,
         CASE WHEN finished_at IS NULL THEN NULL ELSE DATE_FORMAT(finished_at, '%Y-%m-%dT%H:%i:%sZ') END AS finished_at_text
         FROM {TASK_RUNS_TABLE}
         ORDER BY started_at DESC, id DESC
         LIMIT 20"
    ))
    .fetch_all(pool)
    .await?;

    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(TaskRunRecord {
            id: row.try_get("id")?,
            task_type: row.try_get("task_type")?,
            trigger: row.try_get("trigger")?,
            status: row.try_get("status")?,
            summary: row.try_get("summary")?,
            started_at: row.try_get("started_at_text")?,
            finished_at: row.try_get("finished_at_text")?,
        });
    }

    Ok(tasks)
}

#[cfg(feature = "ssr")]
async fn seed_sync_sources(pool: &MySqlPool) -> Result<()> {
    let seeds = [
        (
            "notes-import",
            "笔记内容导入",
            "inbound",
            "planned",
            "filesystem://content/notes",
            "和第三版本地 Markdown 内容系统衔接，后续可扩展成批量导入入口。",
        ),
        (
            "project-catalog",
            "项目资料同步",
            "bidirectional",
            "planned",
            "internal://projects",
            "为项目卡片、项目状态和未来外部项目源预留统一边界。",
        ),
        (
            "search-runtime",
            "搜索运行态回写",
            "outbound",
            "healthy",
            "redis://127.0.0.1:6379/",
            "当前把搜索重建运行态写入 Redis，供后台观察索引状态。",
        ),
    ];

    for (source_key, label, direction, status, endpoint, notes) in seeds {
        sqlx::query(&format!(
            "INSERT INTO {SYNC_SOURCES_TABLE} (source_key, label, direction, status, endpoint, notes)
             VALUES (?, ?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE
               label = VALUES(label),
               direction = VALUES(direction),
               status = VALUES(status),
               endpoint = VALUES(endpoint),
               notes = VALUES(notes)"
        ))
        .bind(source_key)
        .bind(label)
        .bind(direction)
        .bind(status)
        .bind(endpoint)
        .bind(notes)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[cfg(feature = "ssr")]
async fn load_sync_sources(pool: &MySqlPool) -> Result<Vec<SyncSourceRecord>> {
    let rows = sqlx::query(&format!(
        "SELECT id, source_key, label, direction, status, endpoint,
         CASE WHEN last_run_at IS NULL THEN NULL ELSE DATE_FORMAT(last_run_at, '%Y-%m-%dT%H:%i:%sZ') END AS last_run_at_text,
         notes
         FROM {SYNC_SOURCES_TABLE}
         ORDER BY id ASC"
    ))
    .fetch_all(pool)
    .await?;

    let mut sources = Vec::new();
    for row in rows {
        sources.push(SyncSourceRecord {
            id: row.try_get("id")?,
            source_key: row.try_get("source_key")?,
            label: row.try_get("label")?,
            direction: row.try_get("direction")?,
            status: row.try_get("status")?,
            endpoint: row.try_get("endpoint")?,
            last_run_at: row.try_get("last_run_at_text")?,
            notes: row.try_get("notes")?,
        });
    }

    Ok(sources)
}

#[cfg(feature = "ssr")]
async fn load_sync_runs(pool: &MySqlPool) -> Result<Vec<SyncRunRecord>> {
    let rows = sqlx::query(&format!(
        "SELECT id, source_key, `trigger`, status, summary,
         DATE_FORMAT(started_at, '%Y-%m-%dT%H:%i:%sZ') AS started_at_text,
         CASE WHEN finished_at IS NULL THEN NULL ELSE DATE_FORMAT(finished_at, '%Y-%m-%dT%H:%i:%sZ') END AS finished_at_text
         FROM {SYNC_RUNS_TABLE}
         ORDER BY started_at DESC, id DESC
         LIMIT 20"
    ))
    .fetch_all(pool)
    .await?;

    let mut runs = Vec::new();
    for row in rows {
        runs.push(SyncRunRecord {
            id: row.try_get("id")?,
            source_key: row.try_get("source_key")?,
            trigger: row.try_get("trigger")?,
            status: row.try_get("status")?,
            summary: row.try_get("summary")?,
            started_at: row.try_get("started_at_text")?,
            finished_at: row.try_get("finished_at_text")?,
        });
    }

    Ok(runs)
}

#[cfg(feature = "ssr")]
async fn load_search_rebuild_records(pool: &MySqlPool) -> Result<Vec<SearchRebuildRecord>> {
    let rows = sqlx::query(&format!(
        "SELECT id, `trigger`, status, message, document_count,
         DATE_FORMAT(started_at, '%Y-%m-%dT%H:%i:%sZ') AS started_at_text,
         CASE WHEN finished_at IS NULL THEN NULL ELSE DATE_FORMAT(finished_at, '%Y-%m-%dT%H:%i:%sZ') END AS finished_at_text
         FROM {SEARCH_REBUILD_TABLE} ORDER BY started_at DESC LIMIT 8"
    ))
    .fetch_all(pool)
    .await?;

    let mut records = Vec::new();
    for row in rows {
        records.push(SearchRebuildRecord {
            run_id: row.try_get("id")?,
            trigger: row.try_get("trigger")?,
            status: row.try_get("status")?,
            message: row.try_get("message")?,
            document_count: row.try_get::<i32, _>("document_count")? as usize,
            started_at: row.try_get("started_at_text")?,
            finished_at: row.try_get("finished_at_text")?,
        });
    }

    Ok(records)
}

#[cfg(feature = "ssr")]
fn collect_index_board_coverage(docs: &[SearchIndexDocument]) -> Vec<String> {
    let mut boards = docs
        .iter()
        .filter_map(|doc| {
            doc.board
                .as_ref()
                .map(|board| note_board_label(board).to_string())
        })
        .collect::<Vec<_>>();
    boards.sort();
    boards.dedup();
    boards
}

#[cfg(feature = "ssr")]
async fn write_search_runtime_state(
    redis_client: Option<&RedisClient>,
    status: &str,
    trigger: &str,
    message: &str,
    document_count: usize,
) {
    let Some(redis_client) = redis_client else {
        return;
    };

    if let Ok(mut connection) = redis_client.get_multiplexed_async_connection().await {
        let _: redis::RedisResult<()> = connection
            .set(format!("{SEARCH_STATUS_KEY_PREFIX}:status"), status)
            .await;
        let _: redis::RedisResult<()> = connection
            .set(format!("{SEARCH_STATUS_KEY_PREFIX}:trigger"), trigger)
            .await;
        let _: redis::RedisResult<()> = connection
            .set(format!("{SEARCH_STATUS_KEY_PREFIX}:message"), message)
            .await;
        let _: redis::RedisResult<()> = connection
            .set(
                format!("{SEARCH_STATUS_KEY_PREFIX}:document-count"),
                document_count as i64,
            )
            .await;
        let _: redis::RedisResult<()> = connection
            .set(
                format!("{SEARCH_STATUS_KEY_PREFIX}:updated-at"),
                Utc::now().to_rfc3339(),
            )
            .await;
    }
}

#[cfg(feature = "ssr")]
struct SearchRuntime {
    db_pool: Option<MySqlPool>,
    redis_client: Option<RedisClient>,
    db_error: Option<String>,
    redis_error: Option<String>,
}

#[cfg(feature = "ssr")]
impl SearchRuntime {
    fn db_status_label(&self) -> &'static str {
        if self.db_pool.is_some() {
            "connected"
        } else {
            "disconnected"
        }
    }

    fn db_status_detail(&self) -> &str {
        self.db_error
            .as_deref()
            .unwrap_or("MySQL 已连接，可持久化搜索文档。")
    }

    fn redis_status_label(&self) -> &'static str {
        if self.redis_client.is_some() {
            "connected"
        } else {
            "disconnected"
        }
    }

    fn redis_status_detail(&self) -> &str {
        self.redis_error
            .as_deref()
            .unwrap_or("Redis 已连接，可记录索引运行态状态。")
    }
}

#[cfg(feature = "ssr")]
async fn load_search_runtime() -> SearchRuntime {
    let database_url = std::env::var("BLOG_DATABASE_URL").ok();
    let redis_url =
        std::env::var("BLOG_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());

    let (db_pool, db_error) = if let Some(database_url) = database_url {
        match MySqlPoolOptions::new()
            .max_connections(3)
            .connect(&database_url)
            .await
        {
            Ok(pool) => (Some(pool), None),
            Err(error) => (None, Some(format!("MySQL 连接失败：{error}"))),
        }
    } else {
        (
            None,
            Some("未设置环境变量 BLOG_DATABASE_URL，搜索索引当前不会落到 MySQL。".to_string()),
        )
    };

    let (redis_client, redis_error) = match RedisClient::open(redis_url.clone()) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(_) => (Some(client), None),
            Err(error) => (None, Some(format!("Redis 连接失败：{error}"))),
        },
        Err(error) => (None, Some(format!("Redis URL 无效：{error}"))),
    };

    SearchRuntime {
        db_pool,
        redis_client,
        db_error,
        redis_error,
    }
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

    let mut contribution_counts = BTreeMap::new();
    for date in posts
        .iter()
        .map(|post| post.date)
        .chain(notes.iter().map(|note| note.date))
    {
        *contribution_counts.entry(date).or_insert(0usize) += 1;
    }

    let contribution_total = contribution_counts.values().sum::<usize>();
    let end_date = Utc::now().date_naive();
    let raw_start = end_date - Duration::days(363);
    let start_date = raw_start - Duration::days(raw_start.weekday().num_days_from_monday() as i64);
    let total_days = (end_date - start_date).num_days() as usize + 1;
    let max_contribution_count = contribution_counts.values().copied().max().unwrap_or(1);
    let mut contribution_cells = Vec::with_capacity(total_days);
    let mut contribution_months = Vec::new();

    for day_offset in 0..total_days {
        let current_date = start_date + Duration::days(day_offset as i64);
        let count = contribution_counts.get(&current_date).copied().unwrap_or(0);
        let level = if count == 0 {
            0
        } else {
            (((count * 4) + max_contribution_count - 1) / max_contribution_count).clamp(1, 4) as u8
        };
        let week_index = day_offset / 7;

        if current_date.day() == 1 || day_offset == 0 {
            let label = format!("{}月", current_date.month());
            if contribution_months
                .last()
                .map(|item: &ContributionMonthLabel| item.week_index != week_index)
                .unwrap_or(true)
            {
                contribution_months.push(ContributionMonthLabel { label, week_index });
            }
        }

        contribution_cells.push(ContributionCell {
            date: current_date,
            count,
            level,
            week_index,
            weekday_index: current_date.weekday().num_days_from_monday(),
        });
    }

    Ok(HomeOverview {
        latest_posts,
        latest_notes,
        featured_project,
        recent_activity,
        focus_tags,
        stats,
        contribution_cells,
        contribution_months,
        contribution_total,
    })
}
#[cfg(feature = "ssr")]
pub async fn get_admin_dashboard_overview() -> Result<AdminDashboardOverview> {
    let items = build_admin_content_index()?;
    let total_items = items.len();
    let total_issues = items.iter().map(|item| item.issue_count).sum::<usize>();
    let published_count = items
        .iter()
        .filter(|item| item.status == ContentLifecycleStatus::Published)
        .count();

    let mut recent_items = items.clone();
    sort_admin_items(&mut recent_items);
    recent_items.truncate(6);

    let content_types = summarize_admin_content_types(&items);
    let stats = vec![
        AdminSummaryStat {
            label: "内容总量".to_string(),
            value: total_items.to_string(),
            detail: "当前后台统一接入 blog / notes / projects 三类内容。".to_string(),
            href: "/admin/content".to_string(),
        },
        AdminSummaryStat {
            label: "已发布".to_string(),
            value: published_count.to_string(),
            detail: "第一批后台读模型默认把现有公开内容视为已发布。".to_string(),
            href: "/admin/content?status=published".to_string(),
        },
        AdminSummaryStat {
            label: "问题项".to_string(),
            value: total_issues.to_string(),
            detail: "当前问题摘要主要用于暴露字段缺口和治理线索，不代表审核流。".to_string(),
            href: "/admin/content".to_string(),
        },
    ];

    Ok(AdminDashboardOverview {
        stats,
        content_types,
        recent_items,
        bridge_notes: vec![
            "当前后台读模型直接复用第三版内容解析与关联逻辑。".to_string(),
            "正式数据库 schema、写入动作和任务调度会在第四版后续阶段接入。".to_string(),
            "现阶段 `/admin/content/:id` 使用“内容类型 + slug”的派生 ID。".to_string(),
        ],
    })
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_dashboard_overview() -> Result<AdminDashboardOverview, String> {
    unreachable!("get_admin_dashboard_overview 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn list_admin_content(
    query: &str,
    type_filter: Option<&str>,
    status_filter: Option<&str>,
) -> Result<Vec<AdminContentListItem>> {
    let normalized_query = normalize_text(query);
    let normalized_type = type_filter.map(normalize_text).unwrap_or_default();
    let normalized_status = status_filter.map(normalize_text).unwrap_or_default();

    let mut items = build_admin_content_index()?
        .into_iter()
        .filter(|item| {
            (normalized_type.is_empty()
                || normalize_text(&item.content_type_key) == normalized_type)
                && (normalized_status.is_empty()
                    || normalize_text(&item.status_label) == normalized_status
                    || normalize_text(status_key(&item.status)) == normalized_status)
                && (normalized_query.is_empty()
                    || admin_item_matches_query(item, &normalized_query))
        })
        .collect::<Vec<_>>();

    sort_admin_items(&mut items);

    Ok(items)
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn list_admin_content(
    _query: &str,
    _type_filter: Option<&str>,
    _status_filter: Option<&str>,
) -> Result<Vec<AdminContentListItem>, String> {
    unreachable!("list_admin_content 只会在服务器侧执行")
}

#[cfg(feature = "ssr")]
pub async fn get_admin_content_detail(id: &str) -> Result<AdminContentDetail> {
    let (content_type, slug) = parse_admin_content_id(id)?;

    match content_type {
        "blog" => {
            let post = get_blog_post(slug).await?;
            let issues = build_blog_admin_issues(&post);

            Ok(AdminContentDetail {
                item: admin_list_item_from_blog(&post, issues.len()),
                facts: vec![
                    AdminContentFact {
                        label: "公开路由".to_string(),
                        value: format!("/blog/{}", post.slug),
                    },
                    AdminContentFact {
                        label: "内容来源".to_string(),
                        value: format!("{BLOG_DIR}/{}.md", post.slug),
                    },
                    AdminContentFact {
                        label: "系列".to_string(),
                        value: humanize_slug(&post.series),
                    },
                    AdminContentFact {
                        label: "阅读时长".to_string(),
                        value: format!("{} 分钟", post.reading_minutes),
                    },
                    AdminContentFact {
                        label: "标签".to_string(),
                        value: if post.tags.is_empty() {
                            "暂无".to_string()
                        } else {
                            post.tags.join(" / ")
                        },
                    },
                    AdminContentFact {
                        label: "手动关联".to_string(),
                        value: if post.manual_related.is_empty() {
                            "未配置".to_string()
                        } else {
                            post.manual_related.join(" / ")
                        },
                    },
                ],
                related: post.related.clone(),
                issues,
                bridge_notes: vec![
                    "详情数据直接复用第三版 `get_blog_post` 聚合结果。".to_string(),
                    "当前状态字段由第四版后台读模型补出，后续可切换到数据库字段。".to_string(),
                ],
            })
        }
        "notes" => {
            let note = get_note_entry(slug).await?;
            let issues = build_note_admin_issues(&note);

            Ok(AdminContentDetail {
                item: admin_list_item_from_note(&note, issues.len()),
                facts: vec![
                    AdminContentFact {
                        label: "公开路由".to_string(),
                        value: format!("/notes/{}", note.slug),
                    },
                    AdminContentFact {
                        label: "内容来源".to_string(),
                        value: format!("{NOTES_DIR}/{}.md", note.slug),
                    },
                    AdminContentFact {
                        label: "笔记板块".to_string(),
                        value: note_board_label(&note.board).to_string(),
                    },
                    AdminContentFact {
                        label: "阶段".to_string(),
                        value: note.stage.clone(),
                    },
                    AdminContentFact {
                        label: "来源说明".to_string(),
                        value: note.source.clone(),
                    },
                    AdminContentFact {
                        label: "实验状态".to_string(),
                        value: note.experiment_state.clone(),
                    },
                    AdminContentFact {
                        label: "标签".to_string(),
                        value: if note.tags.is_empty() {
                            "暂无".to_string()
                        } else {
                            note.tags.join(" / ")
                        },
                    },
                ],
                related: note.related.clone(),
                issues,
                bridge_notes: vec![
                    "详情数据直接复用第三版 `get_note_entry` 聚合结果。".to_string(),
                    "第四版当前只建立后台读取与治理视图，不接入在线编辑。".to_string(),
                ],
            })
        }
        "projects" => {
            let project = get_project_entry(slug).await?;
            let issues = build_project_admin_issues(&project);

            Ok(AdminContentDetail {
                item: admin_list_item_from_project(&project, issues.len()),
                facts: vec![
                    AdminContentFact {
                        label: "公开路由".to_string(),
                        value: format!("/projects/{}", project.slug),
                    },
                    AdminContentFact {
                        label: "内容来源".to_string(),
                        value: format!("{PROJECTS_DIR}/{}.md", project.slug),
                    },
                    AdminContentFact {
                        label: "项目状态".to_string(),
                        value: project.status.clone(),
                    },
                    AdminContentFact {
                        label: "技术栈".to_string(),
                        value: if project.stack.is_empty() {
                            "暂无".to_string()
                        } else {
                            project.stack.join(" / ")
                        },
                    },
                    AdminContentFact {
                        label: "仓库地址".to_string(),
                        value: project
                            .repo_url
                            .clone()
                            .unwrap_or_else(|| "未填写".to_string()),
                    },
                    AdminContentFact {
                        label: "在线地址".to_string(),
                        value: project
                            .live_url
                            .clone()
                            .unwrap_or_else(|| "未填写".to_string()),
                    },
                ],
                related: project.related.clone(),
                issues,
                bridge_notes: vec![
                    "详情数据直接复用第三版 `get_project_entry` 聚合结果。".to_string(),
                    "项目状态仍来自 Markdown front matter，后续可升级为正式状态字段。".to_string(),
                ],
            })
        }
        _ => Err(ContentError::InvalidAdminContentId(id.to_string()).into()),
    }
}

#[cfg(not(feature = "ssr"))]
#[allow(dead_code)]
pub async fn get_admin_content_detail(_id: &str) -> Result<AdminContentDetail, String> {
    unreachable!("get_admin_content_detail 只会在服务器侧执行")
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
fn build_admin_content_index() -> Result<Vec<AdminContentListItem>> {
    let blog_posts = load_blog_posts()?;
    let note_entries = load_note_entries()?;
    let project_entries = load_project_entries()?;

    let items = blog_posts
        .into_iter()
        .map(|post| {
            let issue_count = build_blog_admin_issues(&post).len();
            admin_list_item_from_blog(&post, issue_count)
        })
        .chain(note_entries.into_iter().map(|note| {
            let issue_count = build_note_admin_issues(&note).len();
            admin_list_item_from_note(&note, issue_count)
        }))
        .chain(project_entries.into_iter().map(|project| {
            let issue_count = build_project_admin_issues(&project).len();
            admin_list_item_from_project(&project, issue_count)
        }))
        .collect::<Vec<_>>();

    Ok(items)
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
        board: front_matter.board,
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
fn admin_list_item_from_blog(post: &BlogPost, issue_count: usize) -> AdminContentListItem {
    AdminContentListItem {
        id: build_admin_content_id("blog", &post.slug),
        content_type_key: "blog".to_string(),
        content_type: "博客".to_string(),
        slug: post.slug.clone(),
        title: post.title.clone(),
        summary: post.summary.clone(),
        status: ContentLifecycleStatus::Published,
        status_label: "published".to_string(),
        source_path: format!("{BLOG_DIR}/{}.md", post.slug),
        public_href: format!("/blog/{}", post.slug),
        admin_href: format!(
            "/admin/content/{}",
            build_admin_content_id("blog", &post.slug)
        ),
        date: Some(post.date),
        primary_context: format!("系列：{}", humanize_slug(&post.series)),
        tags: post.tags.clone(),
        related_count: post.related.len().max(post.manual_related.len()),
        issue_count,
    }
}

#[cfg(feature = "ssr")]
fn admin_list_item_from_note(note: &NoteEntry, issue_count: usize) -> AdminContentListItem {
    AdminContentListItem {
        id: build_admin_content_id("notes", &note.slug),
        content_type_key: "notes".to_string(),
        content_type: "笔记".to_string(),
        slug: note.slug.clone(),
        title: note.title.clone(),
        summary: note.summary.clone(),
        status: ContentLifecycleStatus::Published,
        status_label: "published".to_string(),
        source_path: format!("{NOTES_DIR}/{}.md", note.slug),
        public_href: format!("/notes/{}", note.slug),
        admin_href: format!(
            "/admin/content/{}",
            build_admin_content_id("notes", &note.slug)
        ),
        date: Some(note.date),
        primary_context: format!(
            "板块：{} · 阶段：{}",
            note_board_label(&note.board),
            note.stage
        ),
        tags: note.tags.clone(),
        related_count: note.related.len(),
        issue_count,
    }
}

#[cfg(feature = "ssr")]
fn admin_list_item_from_project(
    project: &ProjectEntry,
    issue_count: usize,
) -> AdminContentListItem {
    AdminContentListItem {
        id: build_admin_content_id("projects", &project.slug),
        content_type_key: "projects".to_string(),
        content_type: "项目".to_string(),
        slug: project.slug.clone(),
        title: project.title.clone(),
        summary: project.summary.clone(),
        status: ContentLifecycleStatus::Published,
        status_label: "published".to_string(),
        source_path: format!("{PROJECTS_DIR}/{}.md", project.slug),
        public_href: format!("/projects/{}", project.slug),
        admin_href: format!(
            "/admin/content/{}",
            build_admin_content_id("projects", &project.slug)
        ),
        date: None,
        primary_context: format!("状态：{}", project.status),
        tags: project.stack.clone(),
        related_count: project.related.len(),
        issue_count,
    }
}

#[cfg(feature = "ssr")]
fn build_blog_admin_issues(post: &BlogPost) -> Vec<AdminContentIssue> {
    let mut issues = Vec::new();

    if post.tags.is_empty() {
        issues.push(admin_issue(
            ContentIssueSeverity::Warning,
            "missing-tags",
            "这篇博客还没有标签，后续聚合和搜索提示会偏弱。",
        ));
    }

    if post.manual_related.is_empty() {
        issues.push(admin_issue(
            ContentIssueSeverity::Info,
            "related-not-curated",
            "这篇博客还没有显式配置 manual related，当前主要依赖自动关联。",
        ));
    }

    issues
}

#[cfg(feature = "ssr")]
fn build_note_admin_issues(note: &NoteEntry) -> Vec<AdminContentIssue> {
    let mut issues = Vec::new();

    if normalize_note_board(&note.board) == "general" {
        issues.push(admin_issue(
            ContentIssueSeverity::Info,
            "board-not-specialized",
            "这条笔记目前仍在通用板块，后续如果主题稳定，可以再归入 rust / cpp / bochs。",
        ));
    }

    if note.source.trim().is_empty() {
        issues.push(admin_issue(
            ContentIssueSeverity::Warning,
            "missing-source",
            "这条笔记没有来源说明，后台治理时不利于追踪上下文。",
        ));
    }

    if note.tags.is_empty() {
        issues.push(admin_issue(
            ContentIssueSeverity::Info,
            "missing-tags",
            "这条笔记还没有标签，主题聚合密度会下降。",
        ));
    }

    issues
}

#[cfg(feature = "ssr")]
fn build_project_admin_issues(project: &ProjectEntry) -> Vec<AdminContentIssue> {
    let mut issues = Vec::new();

    if project.repo_url.is_none() && project.live_url.is_none() {
        issues.push(admin_issue(
            ContentIssueSeverity::Warning,
            "missing-links",
            "这个项目还没有 repo_url 或 live_url，后台难以直接跳转验证。",
        ));
    }

    if project.retrospective.is_empty() {
        issues.push(admin_issue(
            ContentIssueSeverity::Info,
            "missing-retrospective",
            "这个项目还没有 retrospective，后续复盘信息较弱。",
        ));
    }

    issues
}

#[cfg(feature = "ssr")]
fn admin_issue(severity: ContentIssueSeverity, code: &str, message: &str) -> AdminContentIssue {
    AdminContentIssue {
        severity_label: match severity {
            ContentIssueSeverity::Info => "提示".to_string(),
            ContentIssueSeverity::Warning => "注意".to_string(),
        },
        severity,
        code: code.to_string(),
        message: message.to_string(),
    }
}

#[cfg(feature = "ssr")]
fn summarize_admin_content_types(items: &[AdminContentListItem]) -> Vec<AdminContentTypeSummary> {
    let mut by_type = BTreeMap::<String, AdminContentTypeSummary>::new();

    for item in items {
        let entry = by_type
            .entry(item.content_type_key.clone())
            .or_insert_with(|| AdminContentTypeSummary {
                content_type_key: item.content_type_key.clone(),
                content_type: item.content_type.clone(),
                total_count: 0,
                published_count: 0,
                issue_count: 0,
            });

        entry.total_count += 1;
        entry.issue_count += item.issue_count;
        if item.status == ContentLifecycleStatus::Published {
            entry.published_count += 1;
        }
    }

    by_type.into_values().collect()
}

#[cfg(feature = "ssr")]
fn admin_item_matches_query(item: &AdminContentListItem, normalized_query: &str) -> bool {
    [
        item.title.as_str(),
        item.summary.as_str(),
        item.slug.as_str(),
        item.primary_context.as_str(),
        item.source_path.as_str(),
    ]
    .into_iter()
    .any(|value| normalize_text(value).contains(normalized_query))
        || item
            .tags
            .iter()
            .any(|tag| normalize_text(tag).contains(normalized_query))
}

#[cfg(feature = "ssr")]
fn sort_admin_items(items: &mut [AdminContentListItem]) {
    items.sort_by(|left, right| {
        right
            .date
            .cmp(&left.date)
            .then_with(|| left.content_type_key.cmp(&right.content_type_key))
            .then_with(|| left.title.cmp(&right.title))
    });
}

#[cfg(feature = "ssr")]
fn build_admin_content_id(content_type: &str, slug: &str) -> String {
    format!("{content_type}--{slug}")
}

#[cfg(feature = "ssr")]
fn parse_admin_content_id(id: &str) -> Result<(&str, &str)> {
    let Some((content_type, slug)) = id.split_once("--") else {
        return Err(ContentError::InvalidAdminContentId(id.to_string()).into());
    };

    if content_type.is_empty() || slug.is_empty() {
        return Err(ContentError::InvalidAdminContentId(id.to_string()).into());
    }

    Ok((content_type, slug))
}

#[cfg(feature = "ssr")]
fn status_key(status: &ContentLifecycleStatus) -> &'static str {
    match status {
        ContentLifecycleStatus::Draft => "draft",
        ContentLifecycleStatus::Review => "review",
        ContentLifecycleStatus::Published => "published",
        ContentLifecycleStatus::Archived => "archived",
    }
}

#[cfg(feature = "ssr")]
fn display_content_type(content_type_key: &str) -> &'static str {
    match content_type_key {
        "blog" => "博客",
        "notes" => "笔记",
        "projects" => "项目",
        _ => "内容",
    }
}

#[cfg(feature = "ssr")]
fn normalize_note_board(value: &str) -> String {
    let normalized = normalize_text(value);
    match normalized.as_str() {
        "rust" => "rust".to_string(),
        "c++" | "cpp" => "cpp".to_string(),
        "bochs" => "bochs".to_string(),
        "general" | "" => "general".to_string(),
        _ => normalized,
    }
}

#[cfg(feature = "ssr")]
fn note_board_label(value: &str) -> &'static str {
    match normalize_note_board(value).as_str() {
        "rust" => "Rust",
        "cpp" => "C++",
        "bochs" => "Bochs",
        _ => "通用笔记",
    }
}

#[cfg(feature = "ssr")]
fn note_board_description(value: &str) -> &'static str {
    match normalize_note_board(value).as_str() {
        "rust" => "偏 Rust 学习、语义理解、项目实践记录。",
        "cpp" => "预留给 C++ 相关笔记、语法复盘与底层实验。",
        "bochs" => "预留给 Bochs、操作系统实验和调试记录。",
        _ => "暂时还没归入专门技术板块的过程型笔记。",
    }
}

#[cfg(feature = "ssr")]
fn board_sort_order(value: &str) -> usize {
    match normalize_note_board(value).as_str() {
        "rust" => 0,
        "cpp" => 1,
        "bochs" => 2,
        _ => 9,
    }
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
fn encode_string_list(items: &[String]) -> String {
    items
        .iter()
        .map(|item| item.replace('\n', " ").trim().to_string())
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(feature = "ssr")]
fn decode_string_list(raw: &str) -> Vec<String> {
    raw.split('\n')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
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
