pub mod content;

use chrono::NaiveDate;
use content::{
    AdminContentDetail, AdminContentFact, AdminContentIssue, AdminContentListItem,
    AdminContentTypeSummary, AdminDashboardOverview, AdminSearchOverview, AdminStatsOverview,
    AdminSummaryStat, AdminSyncOverview, AdminTasksOverview, ArchiveOverview, ArchiveYearGroup,
    BlogPost, BlogPostSummary, HomeActivityItem, HomeOverview, HomeStat, MetricSnapshot,
    NoteBoardSummary, NoteEntry, NoteSummary, ProjectEntry, ProjectSummary, RelatedContentItem,
    SearchQueryDiagnostic, SearchRebuildRecord, SearchResult, SeriesPage, SyncRunRecord,
    SyncSourceRecord, TagArchive, TagArchiveItem, TagOverviewItem, TagsOverview, TaskRunRecord,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::{use_params_map, use_query_map},
    path,
};
use std::collections::BTreeMap;

#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;

const DEFAULT_SITE_URL: &str = "http://127.0.0.1:3000";

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=false />
                <leptos_meta::MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/my-blog.css" />
        <Title text="Wen's Field Notes" />
        <Meta
            name="description"
            content="一个使用 Leptos SSR 与 Markdown 构建的个人内容站，记录博客、笔记、项目和 Rust 学习过程。"
        />
        <Router>
            <div class="page-shell">
                <SiteHeader />
                <main class="site-main">
                    <Routes fallback=|| view! { <NotFoundPage /> }>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/me") view=MePage />
                        <Route path=path!("/tags") view=TagsOverviewPage />
                        <Route path=path!("/blog") view=BlogListPage />
                        <Route path=path!("/blog/:slug") view=BlogDetailPage />
                        <Route path=path!("/notes") view=NotesPage />
                        <Route path=path!("/notes/:slug") view=NoteDetailPage />
                        <Route path=path!("/projects") view=ProjectsPage />
                        <Route path=path!("/projects/:slug") view=ProjectDetailPage />
                        <Route path=path!("/series/:slug") view=SeriesPageView />
                        <Route path=path!("/tags/:tag") view=TagArchivePage />
                        <Route path=path!("/archive") view=ArchiveOverviewPage />
                        <Route path=path!("/search") view=SearchPage />
                        <Route path=path!("/admin") view=AdminDashboardPage />
                        <Route path=path!("/admin/content") view=AdminContentPage />
                        <Route path=path!("/admin/content/:id") view=AdminContentDetailPage />
                        <Route path=path!("/admin/search") view=AdminSearchPage />
                        <Route path=path!("/admin/stats") view=AdminStatsPage />
                        <Route path=path!("/admin/tasks") view=AdminTasksPage />
                        <Route path=path!("/admin/sync") view=AdminSyncPage />
                        <Route path=path!("/about") view=AboutPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

fn configured_site_url() -> String {
    #[cfg(feature = "ssr")]
    {
        std::env::var("SITE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_SITE_URL.to_string())
            .trim_end_matches('/')
            .to_string()
    }

    #[cfg(not(feature = "ssr"))]
    {
        option_env!("SITE_URL")
            .unwrap_or(DEFAULT_SITE_URL)
            .trim_end_matches('/')
            .to_string()
    }
}

fn absolute_url(path: &str) -> String {
    let site_url = configured_site_url();

    match path {
        "" => site_url,
        "/" => format!("{site_url}/"),
        _ if path.starts_with('/') => format!("{site_url}{path}"),
        _ => format!("{site_url}/{path}"),
    }
}

#[component]
fn PageHeadExtras(
    title: String,
    description: String,
    canonical_path: String,
    #[prop(optional, into)] page_type: Option<String>,
    #[prop(optional, into)] robots: Option<String>,
) -> impl IntoView {
    let canonical_url = absolute_url(&canonical_path);
    let page_type = page_type.unwrap_or_else(|| "website".to_string());

    match robots {
        Some(robots) => view! {
            <Link rel="canonical" href=canonical_url.clone() />
            <Meta name="robots" content=robots />
            <Meta property="og:title" content=title.clone() />
            <Meta property="og:description" content=description.clone() />
            <Meta property="og:type" content=page_type />
            <Meta property="og:url" content=canonical_url />
            <Meta property="og:site_name" content="Wen's Field Notes" />
            <Meta property="og:locale" content="zh_CN" />
            <Meta name="twitter:card" content="summary" />
            <Meta name="twitter:title" content=title />
            <Meta name="twitter:description" content=description />
        }
        .into_any(),
        None => view! {
            <Link rel="canonical" href=canonical_url.clone() />
            <Meta property="og:title" content=title.clone() />
            <Meta property="og:description" content=description.clone() />
            <Meta property="og:type" content=page_type />
            <Meta property="og:url" content=canonical_url />
            <Meta property="og:site_name" content="Wen's Field Notes" />
            <Meta property="og:locale" content="zh_CN" />
            <Meta name="twitter:card" content="summary" />
            <Meta name="twitter:title" content=title />
            <Meta name="twitter:description" content=description />
        }
        .into_any(),
    }
}

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header class="topbar">
            <a href="/" class="brand">
                <span class="brand-mark">"W"</span>
                <span class="brand-copy">
                    <strong>"Wen's Field Notes"</strong>
                    <small>"Leptos SSR + Markdown"</small>
                </span>
            </a>

            <nav class="topnav" aria-label="主导航">
                <NavLink href="/" label="首页" />
                <NavLink href="/me" label="个人主页" />
                <NavLink href="/tags" label="标签" />
                <NavLink href="/archive" label="归档" />
                <NavLink href="/blog" label="博客" />
                <NavLink href="/notes" label="笔记" />
                <NavLink href="/projects" label="项目" />
                <NavLink href="/search" label="搜索" />
                <NavLink href="/about" label="关于" />
            </nav>
        </header>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! { <a href=href>{label}</a> }
}

#[component]
fn HomePage() -> impl IntoView {
    let home_overview = Resource::new_blocking(|| (), |_| async move { get_home_overview().await });

    view! {
        <Title text="首页 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看这个内容驱动的 Rust 个人站首页，快速进入个人主页、博客、笔记、项目与搜索入口。"
        />
        <PageHeadExtras
            title="首页 | Wen's Field Notes".to_string()
            description="查看这个内容驱动的 Rust 个人站首页，快速进入个人主页、博客、笔记、项目与搜索入口。".to_string()
            canonical_path="/".to_string()
        />
        <section class="preview-section hero">
            <div class="section-kicker">"首页"</div>
            <div class="hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"`v1.0` 已把这个站点收束成可运行、可部署的个人内容系统。"</p>
                    <h1>"首页负责欢迎你进来，`/me` 负责把我最近在做什么摊开给你看。"</h1>
                    <p class="lede">
                        "现在的重点不再只是把页面补齐，而是把内容入口、最近动态、主题组织和轻量统计接成一张真正可继续生长的公开工作台。"
                    </p>
                    <div class="hero-actions">
                        <A href="/me" attr:class="button primary">"进入个人主页"</A>
                        <A href="/blog" attr:class="button ghost">"阅读博客"</A>
                        <A href="/search" attr:class="button ghost">"站内搜索"</A>
                    </div>
                </div>

                <aside class="hero-aside">
                    <div class="note-card warm">
                        <span class="meta-label">"当前阶段"</span>
                        <h2>"第五版 · `v1.0` 收尾完成"</h2>
                        <p>"这一轮已经把 `/me`、首页、about、blog、notes、projects 的关系整理成统一站点结构，重点转到真实运行、部署与长期维护。"</p>
                    </div>

                    <div class="note-card">
                        <span class="meta-label">"本轮入口"</span>
                        <ul>
                            <li><A href="/me">"先去 `/me` 看工作台"</A></li>
                            <li><A href="/about">"再看这个站为什么这样做"</A></li>
                            <li><A href="/projects">"最后回到项目与内容现场"</A></li>
                        </ul>
                    </div>
                </aside>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在整理首页内容..." /> }>
                {move || {
                    home_overview.get().map(|result| match result {
                        Ok(overview) => view! { <HomePreview overview=overview /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn HomePreview(overview: HomeOverview) -> impl IntoView {
    let HomeOverview {
        latest_posts,
        latest_notes,
        featured_project,
        recent_activity,
        focus_tags,
        stats,
    } = overview;

    view! {
        <>
            <div class="home-entry-grid">
                <article class="panel entry-panel">
                    <div class="panel-head">
                        <span class="meta-label">"站点入口"</span>
                        <A href="/me">"打开工作台"</A>
                    </div>
                    <div class="entry-card-list">
                        {[
                            ("/me", "个人主页", "集中看当前状态、最近动态和轻量统计。"),
                            ("/tags", "标签总览", "按主题密度进入博客和笔记内容。"),
                            ("/archive", "时间归档", "沿着年份与更新节奏继续浏览。"),
                            ("/blog", "博客", "阅读完整文章与阶段性输出。"),
                            ("/notes", "笔记", "查看学习过程、思路草稿与阶段记录。"),
                            ("/projects", "项目", "跟进长期项目的现场与结果。"),
                            ("/about", "关于", "了解站点背景、方法和当前边界。"),
                        ]
                            .into_iter()
                            .map(|(href, title, summary)| {
                                view! {
                                    <A href=href attr:class="entry-card">
                                        <strong>{title}</strong>
                                        <span>{summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel feature-panel">
                    <div class="panel-head">
                        <span class="meta-label">"最近动态预览"</span>
                        <A href="/me">"查看更多"</A>
                    </div>
                    <div class="activity-list compact">
                        {recent_activity
                            .into_iter()
                            .take(3)
                            .map(|item| {
                                view! {
                                    <A href=item.href attr:class="activity-card">
                                        <div class="activity-topline">
                                            <span class="meta-label">{item.content_type}</span>
                                            <span class="meta-label">{item.date.format("%Y.%m.%d").to_string()}</span>
                                        </div>
                                        <strong>{item.title}</strong>
                                        <span>{item.summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel compact">
                    <div class="panel-head">
                        <span class="meta-label">"轻量统计"</span>
                        <A href="/me">"去主页"</A>
                    </div>
                    <div class="stats-grid compact">
                        {stats
                            .into_iter()
                            .take(3)
                            .map(|stat| {
                                view! {
                                    <A href=stat.href attr:class="stat-card">
                                        <span class="meta-label">{stat.label}</span>
                                        <strong>{stat.value}</strong>
                                        <small>{stat.detail}</small>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>
            </div>

            <div class="home-panels v3">
                <article class="panel split-panel">
                    <div class="panel-head">
                        <span class="meta-label">"最近博客"</span>
                        <A href="/blog">"更多文章"</A>
                    </div>
                    <div class="mini-list">
                        {latest_posts
                            .into_iter()
                            .map(|post| {
                                view! {
                                    <A href=format!("/blog/{}", post.slug) attr:class="mini-list-link">
                                        <strong>{post.title}</strong>
                                        <span>{post.summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel feature-panel compact">
                    <div class="panel-head">
                        <span class="meta-label">"最近笔记"</span>
                        <A href="/notes">"全部笔记"</A>
                    </div>
                    <div class="mini-list">
                        {latest_notes
                            .into_iter()
                            .map(|note| {
                                view! {
                                    <A href=format!("/notes/{}", note.slug) attr:class="mini-list-link">
                                        <strong>{note.title}</strong>
                                        <span>{note.summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel feature-panel compact">
                    <div class="panel-head">
                        <span class="meta-label">"重点项目"</span>
                        <A href="/projects">"项目页"</A>
                    </div>
                    {featured_project
                        .map(|project| {
                            view! {
                                <A href=format!("/projects/{}", project.slug) attr:class="project-feature">
                                    <h3>{project.title}</h3>
                                    <p class="blog-meta">{project.status.clone()}</p>
                                    <p>{project.stack.join(" / ")}</p>
                                    <small>{project.summary}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <p>"项目内容正在整理中。"</p> }.into_any())}
                </article>
            </div>

            <div class="home-reference-grid">
                <article class="panel manifesto-panel">
                    <div class="panel-head">
                        <span class="meta-label">"`v1.0` 重点"</span>
                        <span>"把内容组织成系统"</span>
                    </div>
                    <p>"这个版本的重点不是继续扩张页面，而是把个人主页、内容入口、后台与运行链路整理成真正可部署的个人内容站。"</p>
                    <div class="manifesto-list">
                        <span>"`/me` 负责公开工作台，首页负责迎接访问与总览。"</span>
                        <span>"最近动态和轻量统计先基于本地 Markdown 聚合。"</span>
                        <span>"后续关联、搜索和治理都会复用这层服务端装配结构。"</span>
                    </div>
                </article>

                <article class="panel timeline-panel">
                    <div class="panel-head">
                        <span class="meta-label">"当前主题"</span>
                        <span>"本轮聚焦"</span>
                    </div>
                    <div class="tag-row compact-tags">
                        {focus_tags
                            .into_iter()
                            .map(|tag| {
                                view! { <A href=format!("/tags/{}", tag) attr:class="chip soft">{tag}</A> }
                            })
                            .collect_view()}
                    </div>
                    <div class="timeline-list">
                        <span>"当前已经接入数据库、后台与持久化统计，但仍然不扩展到用户系统和平台化能力。"</span>
                        <span>"最近动态与统计已经接入真实后端链路，便于继续维护和核对运行状态。"</span>
                        <span>"这一步把 `/tags` 和 `/archive` 接起来，下一轮更适合继续推进内容关联。"</span>
                    </div>
                </article>
            </div>
        </>
    }
}

#[component]
fn MePage() -> impl IntoView {
    let home_overview = Resource::new_blocking(|| (), |_| async move { get_home_overview().await });

    view! {
        <Title text="个人主页 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看我的公开工作台：当前状态、内容入口、最近动态、重点项目与轻量统计。"
        />
        <PageHeadExtras
            title="个人主页 | Wen's Field Notes".to_string()
            description="查看我的公开工作台：当前状态、内容入口、最近动态、重点项目与轻量统计。".to_string()
            canonical_path="/me".to_string()
        />
        <section class="preview-section me-section">
            <div class="section-kicker">"个人主页"</div>
            <Suspense fallback=move || view! { <PageLoading label="正在整理个人主页..." /> }>
                {move || {
                    home_overview.get().map(|result| match result {
                        Ok(overview) => view! { <MeWorkbench overview=overview /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn MeWorkbench(overview: HomeOverview) -> impl IntoView {
    let HomeOverview {
        latest_posts,
        latest_notes,
        featured_project,
        recent_activity,
        focus_tags,
        stats,
    } = overview;

    view! {
        <>
            <div class="me-hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"公开工作台 / public workbench"</p>
                    <h1>"这里比 about 更接近现场，比首页更接近我现在真正花时间的地方。"</h1>
                    <p class="lede">
                        "我在用 Rust、Leptos SSR、MySQL、Redis 和 Markdown 把这个站点做成一个可长期维护的个人内容系统。现在的重点不是继续堆功能，而是保证内容发布、搜索、统计、任务和同步这些基础链路真实可运行。"
                    </p>
                    <div class="hero-actions">
                        <A href="/blog" attr:class="button primary">"看最新文章"</A>
                        <A href="/notes" attr:class="button ghost">"看学习笔记"</A>
                        <A href="/projects" attr:class="button ghost">"看项目现场"</A>
                    </div>
                </div>

                <aside class="hero-aside">
                    <div class="note-card warm">
                        <span class="meta-label">"Now"</span>
                        <h2>"个人主页、内容组织、搜索与治理前置"</h2>
                        <p>"这一轮优先把内容站的枢纽层搭起来：先组织、再关联、再搜索、最后再判断后端什么时候真正值得引入。"</p>
                    </div>

                    <div class="note-card">
                        <span class="meta-label">"页面分工"</span>
                        <ul>
                            <li>"首页：欢迎访问与全站总览"</li>
                            <li>"个人主页：当前状态、内容入口、最近动态"</li>
                            <li>"About：背景、方法论与版本边界"</li>
                        </ul>
                    </div>
                </aside>
            </div>

            <div class="me-layout">
                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"内容入口"</span>
                        <A href="/">"回首页"</A>
                    </div>
                    <div class="entry-card-list">
                        {[
                            ("/tags", "标签总览", "按主题查看博客与笔记的聚合入口。"),
                            ("/archive", "时间归档", "按年份浏览最近更新与长期积累。"),
                            ("/blog", "博客", "正式输出、文章化表达与阶段性沉淀。"),
                            ("/notes", "笔记", "学习过程、实验记录与还在形成中的想法。"),
                            ("/projects", "项目", "长期项目的上下文、状态和结果。"),
                            ("/search", "搜索", "直接按关键词穿透 blog / notes / projects。"),
                            ("/about", "About", "为什么做、怎么做、当前明确不做什么。"),
                        ]
                            .into_iter()
                            .map(|(href, title, summary)| {
                                view! {
                                    <A href=href attr:class="entry-card">
                                        <strong>{title}</strong>
                                        <span>{summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"最近动态"</span>
                        <A href="/search">"继续找内容"</A>
                    </div>
                    <div class="activity-list">
                        {recent_activity
                            .into_iter()
                            .map(|item| view! { <ActivityCard item=item /> })
                            .collect_view()}
                    </div>
                </article>
            </div>

            <div class="me-layout secondary">
                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"轻量统计"</span>
                        <span>"内容驱动，不做持久化"</span>
                    </div>
                    <div class="stats-grid">
                        {stats
                            .into_iter()
                            .map(|stat| view! { <StatCard stat=stat /> })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"当前重点项目"</span>
                        <A href="/projects">"全部项目"</A>
                    </div>
                    {featured_project
                        .map(|project| {
                            view! {
                                <A href=format!("/projects/{}", project.slug) attr:class="project-feature expanded">
                                    <span class="meta-label">{project.status.clone()}</span>
                                    <h3>{project.title}</h3>
                                    <p>{project.summary}</p>
                                    <small>{project.stack.join(" / ")}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <p>"项目内容正在整理中。"</p> }.into_any())}
                </article>
            </div>

            <div class="me-layout tertiary">
                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"关注主题"</span>
                        <A href="/search">"去搜索页"</A>
                    </div>
                    <div class="tag-row compact-tags">
                        {focus_tags
                            .into_iter()
                            .map(|tag| {
                                view! { <A href=format!("/tags/{}", tag) attr:class="chip soft">{tag}</A> }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel">
                    <div class="panel-head">
                        <span class="meta-label">"最近入口"</span>
                        <span>"直接继续阅读"</span>
                    </div>
                    <div class="mini-columns">
                        <div class="mini-list">
                            <span class="meta-label">"博客"</span>
                            {latest_posts
                                .into_iter()
                                .take(2)
                                .map(|post| {
                                    view! {
                                        <A href=format!("/blog/{}", post.slug) attr:class="mini-list-link">
                                            <strong>{post.title}</strong>
                                            <span>{post.summary}</span>
                                        </A>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <div class="mini-list">
                            <span class="meta-label">"笔记"</span>
                            {latest_notes
                                .into_iter()
                                .take(2)
                                .map(|note| {
                                    view! {
                                        <A href=format!("/notes/{}", note.slug) attr:class="mini-list-link">
                                            <strong>{note.title}</strong>
                                            <span>{note.summary}</span>
                                        </A>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </article>
            </div>
        </>
    }
}

#[component]
fn ActivityCard(item: HomeActivityItem) -> impl IntoView {
    view! {
        <A href=item.href attr:class="activity-card">
            <div class="activity-topline">
                <span class="meta-label">{item.content_type}</span>
                <span class="meta-label">{format_meta_line(&item.date, &item.tags)}</span>
            </div>
            <strong>{item.title}</strong>
            <span>{item.summary}</span>
        </A>
    }
}

#[component]
fn StatCard(stat: HomeStat) -> impl IntoView {
    view! {
        <A href=stat.href attr:class="stat-card">
            <span class="meta-label">{stat.label}</span>
            <strong>{stat.value}</strong>
            <small>{stat.detail}</small>
        </A>
    }
}

#[component]
fn BlogListPage() -> impl IntoView {
    let blog_posts = Resource::new_blocking(|| (), |_| async move { list_blog_posts().await });

    view! {
        <Title text="博客 | Wen's Field Notes" />
        <Meta
            name="description"
            content="按时间与标签浏览博客文章，查看这个个人内容站里已经正式发布的文章。"
        />
        <PageHeadExtras
            title="博客 | Wen's Field Notes".to_string()
            description="按时间与标签浏览博客文章，查看这个个人内容站里已经正式发布的内容。".to_string()
            canonical_path="/blog".to_string()
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"博客列表"</div>
                    <h2>"正式文章应该像作品被陈列，而不是像日志被堆叠。"</h2>
                </div>
                <p>"这里按时间组织文章，同时把标签入口直接接入归档页，让继续按主题阅读更自然。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在载入博客列表..." /> }>
                {move || {
                    blog_posts.get().map(|posts| match posts {
                        Ok(posts) => view! { <BlogListContent posts=posts /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn BlogListContent(posts: Vec<BlogPostSummary>) -> impl IntoView {
    let mut tags = posts
        .iter()
        .flat_map(|post| post.tags.iter().cloned())
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    let featured_post = posts.first().cloned();
    let secondary_posts = posts.iter().skip(1).take(2).cloned().collect::<Vec<_>>();

    view! {
        <div class="blog-layout">
            <aside class="blog-filter">
                <span class="meta-label">"标签预览"</span>
                <span class="chip active">"全部"</span>
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <A href=format!("/tags/{}", tag) attr:class="chip">
                                {tag}
                            </A>
                        }
                    })
                    .collect_view()}
            </aside>

            <div class="blog-feed enriched">
                {featured_post
                    .map(|post| {
                        view! {
                            <article class="blog-card featured editorial-card">
                                <div class="editorial-copy">
                                    <p class="blog-meta">{format_meta_line(&post.date, &post.tags)}</p>
                                    <h3>
                                        <A href=format!("/blog/{}", post.slug)>{post.title}</A>
                                    </h3>
                                    <p>{post.summary}</p>
                                </div>
                                <div class="editorial-side">
                                    <span class="meta-label">"阅读提示"</span>
                                    <p>"先抓标题和摘要，再决定是否深入阅读，这是列表页最重要的节奏控制。"</p>
                                </div>
                            </article>
                        }
                    })}

                <div class="blog-rail">
                    {secondary_posts
                        .into_iter()
                        .map(|post| {
                            view! {
                                <article class="blog-card compact-card">
                                    <p class="blog-meta">{format_meta_line(&post.date, &post.tags)}</p>
                                    <h3>
                                        <A href=format!("/blog/{}", post.slug)>{post.title}</A>
                                    </h3>
                                    <p>{post.summary}</p>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>

                {posts
                    .into_iter()
                    .enumerate()
                    .map(|(index, post)| {
                        let class = if index == 0 {
                            "blog-card featured is-hidden"
                        } else {
                            "blog-card"
                        };

                        view! {
                            <article class=class>
                                <p class="blog-meta">{format_meta_line(&post.date, &post.tags)}</p>
                                <h3>
                                    <A href=format!("/blog/{}", post.slug)>{post.title}</A>
                                </h3>
                                <p>{post.summary}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn BlogDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|map| map.get("slug").unwrap_or_default()));
    let post = Resource::new_blocking(
        move || slug.get(),
        |slug| async move { get_blog_post(slug).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在载入文章..." /> }>
            {move || {
                post.get().map(|post| match post {
                    Ok(post) => view! { <BlogDetailContent post=post /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn BlogDetailContent(post: BlogPost) -> impl IntoView {
    let html = post.html.clone();
    let title_text = format!("{} | Wen's Field Notes", post.title);
    let description_text = post.summary.clone();
    let tags = post.tags.clone();
    let related = post.related.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text.clone()
            description=description_text.clone()
            canonical_path=format!("/blog/{}", post.slug)
            page_type="article".to_string()
        />
        <section class="preview-section article-preview">
            <div class="section-heading article-head">
                <div>
                    <div class="section-kicker">"博客详情"</div>
                    <h2>"文章页的重点不是装饰，而是让内容被舒服地读完。"</h2>
                </div>

                <div class="article-nav-inline">
                    <A href="/blog">"返回列表"</A>
                    <A href="/search">"去搜索页"</A>
                </div>
            </div>

            <article class="article-card">
                <header class="article-header">
                    <p class="blog-meta">{format_meta_line(&post.date, &post.tags)}</p>
                    <h3>{post.title.clone()}</h3>
                    <div class="tag-row compact-tags">
                        <A href=format!("/series/{}", post.series.clone()) attr:class="chip soft">
                            {format!("系列：{}", humanize_slug(&post.series))}
                        </A>
                        <span class="chip soft">{format!("{} 分钟", post.reading_minutes)}</span>
                    </div>
                    <div class="tag-row">
                        {tags
                            .into_iter()
                            .map(|tag| {
                                view! {
                                    <A href=format!("/tags/{}", tag) attr:class="chip soft">
                                        {tag}
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </header>

                <div class="article-body" inner_html=html></div>

                <RelatedContentSection
                    title="继续阅读"
                    description="这些内容与当前文章共享系列、标签或技术主题。"
                    items=related
                />

                <div class="article-footer article-pagination">
                    {post
                        .previous
                        .clone()
                        .map(|previous| {
                            view! {
                                <A href=format!("/blog/{}", previous.slug) attr:class="pager-card">
                                    <span class="meta-label">"上一篇"</span>
                                    <strong>{previous.title}</strong>
                                    <small>{previous.date.format("%Y.%m.%d").to_string()}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <div class="pager-card empty"></div> }.into_any())}

                    {post
                        .next
                        .clone()
                        .map(|next| {
                            view! {
                                <A href=format!("/blog/{}", next.slug) attr:class="pager-card align-right">
                                    <span class="meta-label">"下一篇"</span>
                                    <strong>{next.title}</strong>
                                    <small>{next.date.format("%Y.%m.%d").to_string()}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <div class="pager-card empty"></div> }.into_any())}
                </div>
            </article>
        </section>
    }
}

#[component]
fn NotesPage() -> impl IntoView {
    let notes = Resource::new_blocking(|| (), |_| async move { list_note_entries().await });
    let boards = Resource::new_blocking(|| (), |_| async move { get_note_boards_overview().await });

    view! {
        <Title text="笔记 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看这个站点中的学习记录、实验结论与过程型笔记。"
        />
        <PageHeadExtras
            title="笔记 | Wen's Field Notes".to_string()
            description="查看这个站点中的学习记录、实验结论与过程型笔记。".to_string()
            canonical_path="/notes".to_string()
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"笔记"</div>
                    <h2>"笔记比博客更轻、更快，也更接近学习过程本身。"</h2>
                </div>
                <p>"这里收集的是短判断、实验结论和阶段记录，不追求每篇都写成完整长文。现在笔记开始按 Rust、C++、Bochs 三个技术板块组织，并保留通用板块承接过程型记录。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在整理笔记板块..." /> }>
                {move || {
                    boards.get().map(|boards| match boards {
                        Ok(boards) => view! { <NotesBoardOverview boards=boards /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>

            <Suspense fallback=move || view! { <PageLoading label="正在载入笔记..." /> }>
                {move || {
                    notes.get().map(|notes| match notes {
                        Ok(notes) => view! { <NotesListContent notes=notes /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn NotesBoardOverview(boards: Vec<NoteBoardSummary>) -> impl IntoView {
    view! {
        <div class="entry-card-list notes-board-grid">
            {boards
                .into_iter()
                .map(|board| {
                    view! {
                        <div class="note-card admin-type-card">
                            <span class="meta-label">{board.label.clone()}</span>
                            <h3>{format!("{} 篇", board.total_count)}</h3>
                            <p>{board.description.clone()}</p>
                            <small class="board-footnote">
                                {board
                                    .latest_date
                                    .map(|date| format!("最近更新：{}", date.format("%Y.%m.%d")))
                                    .unwrap_or_else(|| "当前还没有内容，后续可以直接往这个板块补笔记。".to_string())}
                            </small>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn NotesListContent(notes: Vec<NoteSummary>) -> impl IntoView {
    let mut grouped = BTreeMap::<String, Vec<NoteSummary>>::new();
    for note in notes {
        grouped.entry(note.board.clone()).or_default().push(note);
    }

    let mut ordered_groups = ["rust", "cpp", "bochs", "general"]
        .into_iter()
        .map(|key| {
            let items = grouped.remove(key).unwrap_or_default();
            (key.to_string(), items)
        })
        .collect::<Vec<_>>();
    ordered_groups.extend(grouped.into_iter());

    view! {
        <div class="entry-card-list notes-board-sections">
            {ordered_groups
                .into_iter()
                .map(|(board, items)| {
                    view! {
                        <section class="editorial-card admin-block">
                            <div class="section-heading compact">
                                <div>
                                    <div class="section-kicker">"笔记板块"</div>
                                    <h2>{note_board_label(&board)}</h2>
                                </div>
                                <p>{note_board_description(&board)}</p>
                            </div>
                            <div class="notes-grid">
                                {if items.is_empty() {
                                    view! {
                                        <div class="note-card">
                                            <span class="meta-label">{note_board_label(&board)}</span>
                                            <p>"这个板块暂时还没有内容，后续新增对应笔记后会直接显示在这里。"</p>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        {items
                                            .into_iter()
                                            .map(|note| {
                                                view! {
                                                    <A href=format!("/notes/{}", note.slug) attr:class="note-entry">
                                                        <p class="blog-meta">
                                                            {format!(
                                                                "{} · {} · {}",
                                                                format_meta_line(&note.date, &note.tags),
                                                                note_board_label(&note.board),
                                                                note.stage
                                                            )}
                                                        </p>
                                                        <h3>{note.title}</h3>
                                                        <p>{note.summary}</p>
                                                        <div class="tag-row compact-tags">
                                                            <span class="chip soft">{note_board_label(&note.board)}</span>
                                                            {note
                                                                .tags
                                                                .iter()
                                                                .map(|tag| view! { <span class="chip soft">{tag.clone()}</span> })
                                                                .collect_view()}
                                                        </div>
                                                    </A>
                                                }
                                            })
                                            .collect_view()}
                                    }
                                        .into_any()
                                }}
                            </div>
                        </section>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn NoteDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|map| map.get("slug").unwrap_or_default()));
    let note = Resource::new_blocking(
        move || slug.get(),
        |slug| async move { get_note_entry(slug).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在载入笔记..." /> }>
            {move || {
                note.get().map(|note| match note {
                    Ok(note) => view! { <NoteDetailContent note=note /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn NoteDetailContent(note: NoteEntry) -> impl IntoView {
    let html = note.html.clone();
    let title_text = format!("{} | Wen's Field Notes", note.title);
    let description_text = note.summary.clone();
    let tags = note.tags.clone();
    let related = note.related.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text.clone()
            description=description_text.clone()
            canonical_path=format!("/notes/{}", note.slug)
            page_type="article".to_string()
        />
        <section class="preview-section note-detail-shell">
            <div class="section-heading note-detail-head">
                <div>
                    <div class="section-kicker">"笔记详情"</div>
                    <h2>"这里保留过程感，不把每条记录都写成正式文章。"</h2>
                </div>

                <div class="article-nav-inline">
                    <A href="/notes">"返回笔记列表"</A>
                    <A href="/blog">"查看博客文章"</A>
                </div>
            </div>

            <article class="article-card note-article-card">
                <header class="article-header note-article-header">
                    <p class="blog-meta">{format_meta_line(&note.date, &note.tags)}</p>
                    <h3>{note.title.clone()}</h3>
                    <p class="note-summary">{note.summary.clone()}</p>
                    <div class="tag-row compact-tags">
                        <span class="chip soft">{format!("板块：{}", note_board_label(&note.board))}</span>
                        <span class="chip soft">{format!("阶段：{}", note.stage.clone())}</span>
                        <span class="chip soft">{format!("来源：{}", note.source.clone())}</span>
                        <span class="chip soft">{format!("实验状态：{}", note.experiment_state.clone())}</span>
                    </div>
                    <div class="tag-row">
                        {tags
                            .into_iter()
                            .map(|tag| {
                                view! {
                                    <A href=format!("/tags/{}", tag) attr:class="chip soft">
                                        {tag}
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </header>

                <div class="article-body note-article-body" inner_html=html></div>

                <RelatedContentSection
                    title="继续延展"
                    description="这些内容与当前笔记共享标签、阶段或技术主题。"
                    items=related
                />

                <div class="article-footer note-pagination">
                    {note
                        .previous
                        .clone()
                        .map(|previous| {
                            view! {
                                <A href=format!("/notes/{}", previous.slug) attr:class="pager-card note-pager-card">
                                    <span class="meta-label">"更新较新"</span>
                                    <strong>{previous.title}</strong>
                                    <small>{previous.date.format("%Y.%m.%d").to_string()}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <div class="pager-card empty"></div> }.into_any())}

                    {note
                        .next
                        .clone()
                        .map(|next| {
                            view! {
                                <A href=format!("/notes/{}", next.slug) attr:class="pager-card note-pager-card align-right">
                                    <span class="meta-label">"更新较早"</span>
                                    <strong>{next.title}</strong>
                                    <small>{next.date.format("%Y.%m.%d").to_string()}</small>
                                </A>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| view! { <div class="pager-card empty"></div> }.into_any())}
                </div>
            </article>
        </section>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    let projects = Resource::new_blocking(|| (), |_| async move { list_project_entries().await });

    view! {
        <Title text="项目 | Wen's Field Notes" />
        <Meta
            name="description"
            content="浏览这个站点中的项目展示，了解当前在做什么、用什么做、进行到哪一步。"
        />
        <PageHeadExtras
            title="项目 | Wen's Field Notes".to_string()
            description="浏览这个站点中的项目展示，了解当前在做什么、用什么做、进行到哪一步。".to_string()
            canonical_path="/projects".to_string()
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"项目"</div>
                    <h2>"项目页不是文章索引，而是把正在做的东西清楚地陈列出来。"</h2>
                </div>
                <p>"这一版把项目从列表扩展到详情，并按状态组织，让它更像持续维护中的作品档案。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在载入项目..." /> }>
                {move || {
                    projects.get().map(|projects| match projects {
                        Ok(projects) => view! { <ProjectsListContent projects=projects /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn ProjectsListContent(projects: Vec<ProjectSummary>) -> impl IntoView {
    let mut active = projects
        .iter()
        .filter(|project| project.status != "已归档")
        .cloned()
        .collect::<Vec<_>>();
    let mut archived = projects
        .iter()
        .filter(|project| project.status == "已归档")
        .cloned()
        .collect::<Vec<_>>();
    active.sort_by_key(|project| project.status.clone());
    archived.sort_by_key(|project| project.title.clone());

    view! {
        <div class="project-section-list">
            <section class="project-section-block">
                <div class="panel-head">
                    <span class="meta-label">"进行中与持续整理"</span>
                    <span>{format!("{} 个项目", active.len())}</span>
                </div>
                <div class="projects-grid">
                    {active
                        .into_iter()
                        .map(|project| view! { <ProjectCard project=project /> })
                        .collect_view()}
                </div>
            </section>

            <section class="project-section-block">
                <div class="panel-head">
                    <span class="meta-label">"已归档"</span>
                    <span>{format!("{} 个项目", archived.len())}</span>
                </div>
                <div class="projects-grid">
                    {archived
                        .into_iter()
                        .map(|project| view! { <ProjectCard project=project /> })
                        .collect_view()}
                </div>
            </section>
        </div>
    }
}

#[component]
fn ProjectCard(project: ProjectSummary) -> impl IntoView {
    view! {
        <article class="project-card">
            <div class="panel-head">
                <span class="meta-label">{project.status.clone()}</span>
                <span class="project-stack-inline">{project.stack.join(" / ")}</span>
            </div>
            <h3>{project.title.clone()}</h3>
            <p>{project.summary.clone()}</p>
            <div class="tag-row">
                {project
                    .stack
                    .iter()
                    .map(|item| view! { <span class="chip soft">{item.clone()}</span> })
                    .collect_view()}
            </div>
            <div class="project-links">
                <A href=format!("/projects/{}", project.slug) attr:class="button ghost">
                    "查看详情"
                </A>
                {project
                    .repo_url
                    .clone()
                    .map(|url| {
                        view! {
                            <a href=url target="_blank" rel="noreferrer" class="button ghost">
                                "查看仓库"
                            </a>
                        }
                            .into_any()
                    })
                    .unwrap_or_else(|| ().into_any())}
                {project
                    .live_url
                    .clone()
                    .map(|url| {
                        view! {
                            <a href=url target="_blank" rel="noreferrer" class="button primary">
                                "查看演示"
                            </a>
                        }
                            .into_any()
                    })
                    .unwrap_or_else(|| ().into_any())}
            </div>
        </article>
    }
}

#[component]
fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|map| map.get("slug").unwrap_or_default()));
    let project = Resource::new_blocking(
        move || slug.get(),
        |slug| async move { get_project_entry(slug).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在载入项目..." /> }>
            {move || {
                project.get().map(|project| match project {
                    Ok(project) => view! { <ProjectDetailContent project=project /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn ProjectDetailContent(project: ProjectEntry) -> impl IntoView {
    let html = project.html.clone();
    let title_text = format!("{} | Wen's Field Notes", project.title);
    let description_text = project.summary.clone();
    let related = project.related.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text.clone()
            description=description_text.clone()
            canonical_path=format!("/projects/{}", project.slug)
            page_type="article".to_string()
        />
        <section class="preview-section project-detail-shell">
            <div class="section-heading project-detail-head">
                <div>
                    <div class="section-kicker">"项目详情"</div>
                    <h2>"项目页应该先让人知道它在解决什么，再决定要不要继续深入。"</h2>
                </div>
                <div class="article-nav-inline">
                    <A href="/projects">"返回项目列表"</A>
                    <A href="/search?q=Rust">"继续浏览内容"</A>
                </div>
            </div>

            <article class="article-card project-article-card">
                <header class="article-header project-article-header">
                    <p class="blog-meta">{project.status.clone()}</p>
                    <h3>{project.title.clone()}</h3>
                    <p class="note-summary">{project.summary.clone()}</p>
                    <div class="tag-row compact-tags">
                        <span class="chip soft">{format!("角色：{}", project.role.clone())}</span>
                    </div>
                    <div class="tag-row">
                        {project
                            .stack
                            .iter()
                            .map(|item| view! { <span class="chip soft">{item.clone()}</span> })
                            .collect_view()}
                    </div>
                    <div class="project-links detail-links">
                        <A href="/projects" attr:class="button ghost">"更多项目"</A>
                        {project
                            .repo_url
                            .clone()
                            .map(|url| {
                                view! {
                                    <a href=url target="_blank" rel="noreferrer" class="button ghost">
                                        "查看仓库"
                                    </a>
                                }
                                    .into_any()
                            })
                            .unwrap_or_else(|| ().into_any())}
                        {project
                            .live_url
                            .clone()
                            .map(|url| {
                                view! {
                                    <a href=url target="_blank" rel="noreferrer" class="button primary">
                                        "查看演示"
                                    </a>
                                }
                                    .into_any()
                            })
                            .unwrap_or_else(|| ().into_any())}
                    </div>
                </header>

                <div class="project-facts-grid">
                    <div class="identity-card">
                        <span class="meta-label">"背景"</span>
                        <p>{project.background.clone()}</p>
                    </div>
                    <div class="identity-card">
                        <span class="meta-label">"时间线"</span>
                        <ul>
                            {project
                                .timeline
                                .iter()
                                .map(|item| view! { <li>{item.clone()}</li> })
                                .collect_view()}
                        </ul>
                    </div>
                    <div class="identity-card">
                        <span class="meta-label">"结果"</span>
                        <ul>
                            {project
                                .outcomes
                                .iter()
                                .map(|item| view! { <li>{item.clone()}</li> })
                                .collect_view()}
                        </ul>
                    </div>
                    <div class="identity-card">
                        <span class="meta-label">"复盘"</span>
                        <ul>
                            {project
                                .retrospective
                                .iter()
                                .map(|item| view! { <li>{item.clone()}</li> })
                                .collect_view()}
                        </ul>
                    </div>
                </div>

                <div class="article-body project-article-body" inner_html=html></div>

                <RelatedContentSection
                    title="相关内容"
                    description="这些内容与当前项目共享技术栈、主题或相近推进状态。"
                    items=related
                />
            </article>
        </section>
    }
}

#[component]
fn RelatedContentSection(
    title: &'static str,
    description: &'static str,
    items: Vec<RelatedContentItem>,
) -> impl IntoView {
    if items.is_empty() {
        return ().into_any();
    }

    view! {
        <section class="panel related-section">
            <div class="panel-head">
                <span class="meta-label">{title}</span>
                <span>{description}</span>
            </div>
            <div class="archive-card-list">
                {items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <A href=item.href attr:class="archive-card related-card">
                                <p class="blog-meta">{format!("{} · {}", item.content_type, item.context)}</p>
                                <h3>{item.title}</h3>
                                <p>{item.summary}</p>
                                <small class="blog-meta">{item.reason}</small>
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
    .into_any()
}

#[component]
fn SeriesPageView() -> impl IntoView {
    let params = use_params_map();
    let slug = Memo::new(move |_| params.with(|map| map.get("slug").unwrap_or_default()));
    let series = Resource::new_blocking(
        move || slug.get(),
        |slug| async move { get_series_page(slug).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在整理系列内容..." /> }>
            {move || {
                series.get().map(|result| match result {
                    Ok(series) => view! { <SeriesPageContent series=series /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn SeriesPageContent(series: SeriesPage) -> impl IntoView {
    let title_text = format!("系列：{} | Wen's Field Notes", series.title);
    let description_text = format!(
        "查看系列 {} 下的全部文章，按顺序了解这一主题的持续展开。",
        series.title
    );

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text
            description=description_text
            canonical_path=format!("/series/{}", series.slug)
        />
        <section class="preview-section series-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"系列页"</div>
                    <h2>{format!("{} · {} 篇文章", series.title, series.total_posts)}</h2>
                </div>
                <p>"系列页负责把原本分散的文章重新排回同一条叙事线上。"</p>
            </div>

            <div class="tag-hero-row">
                <span class="chip active">{series.title.clone()}</span>
                <A href="/blog" attr:class="button ghost">"回博客列表"</A>
                <A href="/archive" attr:class="button ghost">"去归档"</A>
            </div>

            <div class="archive-card-list">
                {series
                    .posts
                    .into_iter()
                    .map(|post| {
                        view! {
                            <A href=format!("/blog/{}", post.slug) attr:class="archive-card">
                                <p class="blog-meta">
                                    {format!(
                                        "{} · {} 分钟 · {}",
                                        post.date.format("%Y.%m.%d"),
                                        post.reading_minutes,
                                        post.tags.join(" · ")
                                    )}
                                </p>
                                <h3>{post.title}</h3>
                                <p>{post.summary}</p>
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn TagArchivePage() -> impl IntoView {
    let params = use_params_map();
    let tag = Memo::new(move |_| params.with(|map| map.get("tag").unwrap_or_default()));
    let archive = Resource::new_blocking(
        move || tag.get(),
        |tag| async move { get_tag_archive(tag).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在整理标签归档..." /> }>
            {move || {
                archive.get().map(|archive| match archive {
                    Ok(archive) => view! { <TagArchiveContent archive=archive /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn TagsOverviewPage() -> impl IntoView {
    let overview = Resource::new_blocking(|| (), |_| async move { get_tags_overview().await });

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在整理标签总览..." /> }>
            {move || {
                overview.get().map(|result| match result {
                    Ok(overview) => view! { <TagsOverviewContent overview=overview /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn TagsOverviewContent(overview: TagsOverview) -> impl IntoView {
    let title_text = "标签总览 | Wen's Field Notes".to_string();
    let description_text =
        "查看这个内容站当前已经形成的主题标签，按主题密度继续进入博客与笔记。".to_string();
    let total_tags = overview.total_tags;
    let total_items = overview.total_items;
    let tags = overview.tags.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text
            description=description_text
            canonical_path="/tags".to_string()
        />
        <section class="preview-section tag-overview-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"标签总览"</div>
                    <h2>"先按主题，再深入内容。"</h2>
                </div>
                <p>{format!("当前已形成 {} 个标签入口，共覆盖 {} 次内容命中。", total_tags, total_items)}</p>
            </div>

            <div class="tag-hero-row">
                <span class="chip active">{format!("{} 个标签", total_tags)}</span>
                <span class="meta-label">{format!("{} 次主题出现", total_items)}</span>
                <A href="/archive" attr:class="button ghost">"去时间归档"</A>
            </div>

            <div class="tag-overview-grid">
                {tags
                    .into_iter()
                    .map(|item| view! { <TagOverviewCard item=item /> })
                    .collect_view()}
            </div>

            <div class="panel">
                <div class="panel-head">
                    <span class="meta-label">"继续浏览"</span>
                    <A href="/search">"去搜索页"</A>
                </div>
                <div class="tag-row">
                    <A href="/blog" attr:class="chip soft">"博客"</A>
                    <A href="/notes" attr:class="chip soft">"笔记"</A>
                    <A href="/me" attr:class="chip soft">"个人主页"</A>
                    <A href="/archive" attr:class="chip soft">"归档"</A>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TagOverviewCard(item: TagOverviewItem) -> impl IntoView {
    let latest = item.latest_date.format("%Y.%m.%d").to_string();

    view! {
        <A href=format!("/tags/{}", item.tag) attr:class="archive-card tag-overview-card">
            <div class="panel-head">
                <span class="chip active">{item.tag.clone()}</span>
                <span class="meta-label">{format!("{} 条", item.total_count)}</span>
            </div>
            <p>{format!("博客 {} 篇 · 笔记 {} 条", item.post_count, item.note_count)}</p>
            <small class="blog-meta">{format!("最近更新：{}", latest)}</small>
        </A>
    }
}

#[component]
fn ArchiveOverviewPage() -> impl IntoView {
    let overview = Resource::new_blocking(|| (), |_| async move { get_archive_overview().await });

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在整理时间归档..." /> }>
            {move || {
                overview.get().map(|result| match result {
                    Ok(overview) => view! { <ArchiveOverviewContent overview=overview /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn ArchiveOverviewContent(overview: ArchiveOverview) -> impl IntoView {
    let title_text = "归档 | Wen's Field Notes".to_string();
    let description_text =
        "按年份浏览这个内容站中已经公开的博客与笔记更新，查看长期积累的时间结构。".to_string();
    let total_entries = overview.total_entries;
    let total_years = overview.total_years;
    let years = overview.years.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text
            description=description_text
            canonical_path="/archive".to_string()
        />
        <section class="preview-section archive-overview-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"时间归档"</div>
                    <h2>"让内容按年份露出自己的演进节奏。"</h2>
                </div>
                <p>{format!("当前共收录 {} 条带日期内容，分布在 {} 个年份中。", total_entries, total_years)}</p>
            </div>

            <div class="tag-hero-row">
                <span class="chip active">{format!("{} 条内容", total_entries)}</span>
                <span class="meta-label">{format!("{} 个年份", total_years)}</span>
                <A href="/tags" attr:class="button ghost">"去标签总览"</A>
            </div>

            <div class="archive-year-list">
                {years
                    .into_iter()
                    .map(|group| view! { <ArchiveYearSection group=group /> })
                    .collect_view()}
            </div>

            <div class="panel">
                <div class="panel-head">
                    <span class="meta-label">"当前边界"</span>
                    <span>"当前归档边界"</span>
                </div>
                <div class="timeline-list">
                    <span>"当前时间归档只覆盖博客与笔记，因为它们已经具备稳定日期字段。"</span>
                    <span>"项目内容暂时仍独立展示，是否并入时间归档会放到后续版本统一判断。"</span>
                    <span>"如果想跨类型直接找内容，优先使用搜索页或标签总览。"</span>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ArchiveYearSection(group: ArchiveYearGroup) -> impl IntoView {
    let year = group.year;
    let total = group.entries.len();

    view! {
        <section class="panel archive-year-section">
            <div class="panel-head">
                <span class="meta-label">{year.to_string()}</span>
                <span>{format!("{} 条", total)}</span>
            </div>
            <div class="archive-card-list">
                {group
                    .entries
                    .into_iter()
                    .map(|item| view! { <ArchiveCard item=item /> })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn TagArchiveContent(archive: TagArchive) -> impl IntoView {
    let title_text = format!("标签：{} | Wen's Field Notes", archive.tag);
    let description_text = format!(
        "查看标签 {} 关联的博客与笔记内容，沿着主题继续浏览这个站点中的公开记录。",
        archive.tag
    );
    let total = archive.posts.len() + archive.notes.len();
    let tag_name = archive.tag.clone();
    let related_tags = archive.related_tags.clone();
    let posts = archive.posts.clone();
    let notes = archive.notes.clone();

    view! {
        <Title text=title_text.clone() />
        <Meta name="description" content=description_text.clone() />
        <PageHeadExtras
            title=title_text.clone()
            description=description_text.clone()
            canonical_path=format!("/tags/{}", archive.tag)
        />
        <section class="preview-section tag-archive-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"标签归档"</div>
                    <h2>{format!("围绕 {} 的内容入口", tag_name)}</h2>
                </div>
                <p>{format!("当前共命中 {} 条内容，先按博客和笔记分开展示，保持阅读路径清楚。", total)}</p>
            </div>

            <div class="tag-hero-row">
                <span class="chip active">{archive.tag.clone()}</span>
                <span class="meta-label">{format!("{} 条内容", total)}</span>
                <A href="/tags" attr:class="button ghost">"回标签总览"</A>
                <A href="/archive" attr:class="button ghost">"去时间归档"</A>
                <A href="/search" attr:class="button ghost">"去搜索页继续找"</A>
            </div>

            <div class="tag-archive-layout">
                <section class="archive-group">
                    <div class="panel-head">
                        <span class="meta-label">"博客"</span>
                        <span>{format!("{} 篇", posts.len())}</span>
                    </div>
                    <div class="archive-card-list">
                        {posts
                            .into_iter()
                            .map(|item| view! { <ArchiveCard item=item /> })
                            .collect_view()}
                    </div>
                </section>

                <section class="archive-group">
                    <div class="panel-head">
                        <span class="meta-label">"笔记"</span>
                        <span>{format!("{} 条", notes.len())}</span>
                    </div>
                    <div class="archive-card-list">
                        {notes
                            .into_iter()
                            .map(|item| view! { <ArchiveCard item=item /> })
                            .collect_view()}
                    </div>
                </section>
            </div>

            <div class="tag-row">
                {related_tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <A href=format!("/tags/{}", tag) attr:class="chip">
                                {tag}
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

#[component]
fn ArchiveCard(item: TagArchiveItem) -> impl IntoView {
    view! {
        <A href=item.href.clone() attr:class="archive-card">
            <p class="blog-meta">{format!("{} · {}", item.content_type, format_meta_line(&item.date, &item.tags))}</p>
            <h3>{item.title}</h3>
            <p>{item.summary}</p>
        </A>
    }
}

#[component]
fn SearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let query = Memo::new(move |_| query_map.with(|map| map.get("q").unwrap_or_default()));
    let type_filter = Memo::new(move |_| query_map.with(|map| map.get("type").unwrap_or_default()));
    let tag_filter = Memo::new(move |_| query_map.with(|map| map.get("tag").unwrap_or_default()));
    let search_results = Resource::new(
        move || (query.get(), type_filter.get(), tag_filter.get()),
        |(current_query, current_type, current_tag)| async move {
            search_content(
                current_query.clone(),
                current_type.clone(),
                current_tag.clone(),
            )
            .await
            .map(|results| (current_query, current_type, current_tag, results))
        },
    );

    view! {
        <Title text="搜索 | Wen's Field Notes" />
        <Meta
            name="description"
            content="在博客、笔记和项目范围内搜索这个站点中的公开内容。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="搜索 | Wen's Field Notes".to_string()
            description="在博客、笔记和项目范围内搜索这个站点中的公开内容。".to_string()
            canonical_path="/search".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section search-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"站内搜索"</div>
                    <h2>"不想只靠时间线找内容时，搜索页应该给你一条更直接的路。"</h2>
                </div>
                <p>"搜索目前覆盖博客、笔记和项目，标题、摘要、正文和关键词检索都已经进入真实索引链路。"</p>
            </div>

            <SearchForm
                query=query.get_untracked()
                type_filter=type_filter.get_untracked()
                tag_filter=tag_filter.get_untracked()
            />
            <div class="tag-row">
                <A href="/tags" attr:class="chip soft">"标签总览"</A>
                <A href="/archive" attr:class="chip soft">"时间归档"</A>
                <A href="/me" attr:class="chip soft">"个人主页"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在搜索内容..." /> }>
                {move || {
                    search_results.get().map(|result| match result {
                        Ok((current_query, current_type, current_tag, results)) => {
                            view! {
                                <SearchResultsContent
                                    query=current_query
                                    type_filter=current_type
                                    tag_filter=current_tag
                                    results=results
                                />
                            }
                                .into_any()
                        }
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn SearchForm(query: String, type_filter: String, tag_filter: String) -> impl IntoView {
    view! {
        <form action="/search" method="get" class="search-form">
            <label class="search-label" for="search-q">
                "搜索 blog / notes / projects"
            </label>
            <div class="search-form-row">
                <input
                    id="search-q"
                    class="search-input"
                    type="search"
                    name="q"
                    value=query
                    placeholder="例如：Rust、Leptos、Ownership、PRD"
                />
                <button type="submit" class="button primary">"开始搜索"</button>
            </div>
            <div class="search-filter-row">
                <label class="search-label">
                    "类型"
                    <select name="type" class="search-input search-select">
                        <option value="" selected={type_filter.is_empty()}>"全部"</option>
                        <option value="blog" selected={type_filter == "blog"}>"博客"</option>
                        <option value="notes" selected={type_filter == "notes"}>"笔记"</option>
                        <option value="projects" selected={type_filter == "projects"}>"项目"</option>
                    </select>
                </label>
                <label class="search-label">
                    "标签"
                    <input
                        class="search-input"
                        type="text"
                        name="tag"
                        value=tag_filter
                        placeholder="例如：Rust"
                    />
                </label>
            </div>
        </form>
    }
}

#[component]
fn SearchResultsContent(
    query: String,
    type_filter: String,
    tag_filter: String,
    results: Vec<SearchResult>,
) -> impl IntoView {
    let normalized_query = query.trim().to_string();
    let has_query = !normalized_query.is_empty();
    let has_tag_filter = !tag_filter.trim().is_empty();

    if !has_query && !has_tag_filter {
        return view! {
            <div class="loading-card search-empty-card">
                <span class="meta-label">"等待输入"</span>
                <p>"输入关键词或标签后，这里会展示博客、笔记和项目中的匹配结果。"</p>
            </div>
        }
        .into_any();
    }

    if results.is_empty() {
        return view! {
            <div class="loading-card search-empty-card">
                <span class="meta-label">"没有命中"</span>
                <p>{format!("没有找到和 “{}” 相关的内容。可以换一个词，或者从标签页继续浏览。", normalized_query)}</p>
            </div>
        }
        .into_any();
    }

    view! {
        <div class="search-results-shell">
            <div class="panel-head">
                <span class="meta-label">
                    {format!(
                        "搜索词：{}{}{}",
                        if has_query { normalized_query.clone() } else { "（空）".to_string() },
                        if type_filter.trim().is_empty() {
                            "".to_string()
                        } else {
                            format!(" · 类型：{}", display_search_type(type_filter.trim()))
                        },
                        if tag_filter.trim().is_empty() {
                            "".to_string()
                        } else {
                            format!(" · 标签：{}", tag_filter)
                        }
                    )}
                </span>
                <span>{format!("共 {} 条结果", results.len())}</span>
            </div>
            <div class="search-results-list">
                {results
                    .into_iter()
                    .map(|result| {
                        view! { <SearchResultCard result=result query=normalized_query.clone() /> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn SearchResultCard(result: SearchResult, query: String) -> impl IntoView {
    let title_html = highlight_text_html(&result.title, &query);
    let summary_html = highlight_text_html(&result.summary, &query);

    view! {
        <A href=result.href.clone() attr:class="archive-card search-result-card">
            <p class="blog-meta">{format!("{} · {}", result.content_type, result.context)}</p>
            <h3 inner_html=title_html></h3>
            <p inner_html=summary_html></p>
            <div class="panel-head search-result-foot">
                <span class="meta-label">{result.match_hint}</span>
                <div class="tag-row compact-tags">
                    {result
                        .keywords
                        .into_iter()
                        .map(|item| view! { <span class="chip soft">{item}</span> })
                        .collect_view()}
                </div>
            </div>
        </A>
    }
}

#[component]
fn AdminDashboardPage() -> impl IntoView {
    let dashboard = Resource::new_blocking(
        || (),
        |_| async move { get_admin_dashboard_overview().await },
    );

    view! {
        <Title text="后台概览 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看当前内容后台的运行概览、内容总量、问题摘要与管理入口。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="后台概览 | Wen's Field Notes".to_string()
            description="查看当前内容后台的运行概览、内容总量、问题摘要与管理入口。".to_string()
            canonical_path="/admin".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"`v1.0` 后台"</div>
                    <h2>"这里汇总内容后台、搜索、统计、任务和同步的真实运行状态。"</h2>
                </div>
                <p>"当前后台已经接入真实的 MySQL 与 Redis，用来验证内容治理、索引重建、统计快照和同步记录是否正常落库。"</p>
            </div>

            <div class="tag-row">
                <A href="/admin/content" attr:class="chip active">"内容后台"</A>
                <A href="/admin/search" attr:class="chip active">"搜索索引"</A>
                <A href="/admin/stats" attr:class="chip soft">"统计快照"</A>
                <A href="/admin/tasks" attr:class="chip soft">"任务记录"</A>
                <A href="/admin/sync" attr:class="chip soft">"同步边界"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在加载后台概览..." /> }>
                {move || {
                    dashboard.get().map(|result| match result {
                        Ok(overview) => view! { <AdminDashboardContent overview=overview /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminDashboardContent(overview: AdminDashboardOverview) -> impl IntoView {
    view! {
        <div class="admin-grid">
            <div class="entry-card-list stats-grid">
                {overview
                    .stats
                    .into_iter()
                    .map(|stat| view! { <AdminStatCard stat=stat /> })
                    .collect_view()}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"内容类型"</span>
                    <h3>"当前后台统一视图覆盖三类内容。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .content_types
                        .into_iter()
                        .map(|item| view! { <AdminTypeSummaryCard item=item /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"最近内容"</span>
                    <h3>"后台优先关心最近变动和问题密度。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .recent_items
                        .into_iter()
                        .map(|item| view! { <AdminContentRow item=item /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"衔接说明"</span>
                    <h3>"后台现在承担运行验证与治理入口，不扩张为完整内容平台。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {overview
                        .bridge_notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn AdminStatCard(stat: AdminSummaryStat) -> impl IntoView {
    view! {
        <A href=stat.href attr:class="note-card warm admin-stat-card">
            <span class="meta-label">{stat.label}</span>
            <h3>{stat.value}</h3>
            <p>{stat.detail}</p>
        </A>
    }
}

#[component]
fn AdminTypeSummaryCard(item: AdminContentTypeSummary) -> impl IntoView {
    view! {
        <div class="note-card admin-type-card">
            <span class="meta-label">{item.content_type.clone()}</span>
            <h3>{format!("{} 条", item.total_count)}</h3>
            <p>{format!("已发布 {} · 问题项 {}", item.published_count, item.issue_count)}</p>
        </div>
    }
}

#[component]
fn AdminContentPage() -> impl IntoView {
    let query_map = use_query_map();
    let query = Memo::new(move |_| query_map.with(|map| map.get("q").unwrap_or_default()));
    let type_filter = Memo::new(move |_| query_map.with(|map| map.get("type").unwrap_or_default()));
    let status_filter =
        Memo::new(move |_| query_map.with(|map| map.get("status").unwrap_or_default()));
    let content_items = Resource::new(
        move || (query.get(), type_filter.get(), status_filter.get()),
        |(current_query, current_type, current_status)| async move {
            list_admin_content(
                current_query.clone(),
                current_type.clone(),
                current_status.clone(),
            )
            .await
            .map(|items| (current_query, current_type, current_status, items))
        },
    );

    view! {
        <Title text="内容后台 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看统一内容后台列表、筛选条件、来源路径和问题摘要。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="内容后台 | Wen's Field Notes".to_string()
            description="查看统一内容后台列表、筛选条件、来源路径和问题摘要。".to_string()
            canonical_path="/admin/content".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"内容后台"</div>
                    <h2>"这里先把 blog / notes / projects 拉到同一张后台视图里。"</h2>
                </div>
                <p>"当前列表已经是服务端聚合结果，不再依赖前端自己拼接三类内容。"</p>
            </div>

            <AdminContentFilterForm
                query=query.get_untracked()
                type_filter=type_filter.get_untracked()
                status_filter=status_filter.get_untracked()
            />

            <Suspense fallback=move || view! { <PageLoading label="正在加载内容后台..." /> }>
                {move || {
                    content_items.get().map(|result| match result {
                        Ok((current_query, current_type, current_status, items)) => {
                            view! {
                                <AdminContentListContent
                                    query=current_query
                                    type_filter=current_type
                                    status_filter=current_status
                                    items=items
                                />
                            }
                                .into_any()
                        }
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminContentFilterForm(
    query: String,
    type_filter: String,
    status_filter: String,
) -> impl IntoView {
    view! {
        <form action="/admin/content" method="get" class="search-form admin-filter-form">
            <label class="search-label" for="admin-q">
                "按标题、摘要、slug、标签或来源路径搜索"
            </label>
            <div class="search-form-row">
                <input
                    id="admin-q"
                    class="search-input"
                    type="search"
                    name="q"
                    value=query
                    placeholder="例如：Rust、Leptos、building-content-site、content/blog"
                />
                <button type="submit" class="button primary">"筛选内容"</button>
            </div>
            <div class="search-filter-row">
                <label class="search-label">
                    "类型"
                    <select name="type" class="search-input search-select">
                        <option value="" selected={type_filter.is_empty()}>"全部"</option>
                        <option value="blog" selected={type_filter == "blog"}>"博客"</option>
                        <option value="notes" selected={type_filter == "notes"}>"笔记"</option>
                        <option value="projects" selected={type_filter == "projects"}>"项目"</option>
                    </select>
                </label>
                <label class="search-label">
                    "状态"
                    <select name="status" class="search-input search-select">
                        <option value="" selected={status_filter.is_empty()}>"全部"</option>
                        <option value="published" selected={status_filter == "published"}>"published"</option>
                    </select>
                </label>
            </div>
        </form>
    }
}

#[component]
fn AdminContentListContent(
    query: String,
    type_filter: String,
    status_filter: String,
    items: Vec<AdminContentListItem>,
) -> impl IntoView {
    view! {
        <div class="entry-card-list admin-list-shell">
            <div class="panel-head">
                <span class="meta-label">
                    {format!(
                        "query={} · type={} · status={}",
                        if query.trim().is_empty() { "全部".to_string() } else { query.clone() },
                        if type_filter.trim().is_empty() { "全部".to_string() } else { type_filter.clone() },
                        if status_filter.trim().is_empty() { "全部".to_string() } else { status_filter.clone() }
                    )}
                </span>
                <span>{format!("共 {} 条内容", items.len())}</span>
            </div>

            {if items.is_empty() {
                view! {
                    <div class="loading-card search-empty-card">
                        <span class="meta-label">"没有结果"</span>
                        <p>"当前筛选条件下没有命中内容。可以放宽关键词，或者切回全部类型。"</p>
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div class="entry-card-list">
                        {items
                            .into_iter()
                            .map(|item| view! { <AdminContentRow item=item /> })
                            .collect_view()}
                    </div>
                }
                    .into_any()
            }}
        </div>
    }
}

#[component]
fn AdminContentRow(item: AdminContentListItem) -> impl IntoView {
    view! {
        <A href=item.admin_href.clone() attr:class="archive-card admin-content-card">
            <div class="panel-head">
                <p class="blog-meta">
                    {format!(
                        "{} · {} · {}",
                        item.content_type,
                        item.status_label,
                        item.date
                            .map(|date| date.format("%Y-%m-%d").to_string())
                            .unwrap_or_else(|| "无日期".to_string())
                    )}
                </p>
                <span class="meta-label">{format!("问题项 {}", item.issue_count)}</span>
            </div>
            <h3>{item.title.clone()}</h3>
            <p>{item.summary.clone()}</p>
            <div class="entry-card-list admin-content-facts">
                <span class="chip soft">{format!("slug: {}", item.slug)}</span>
                <span class="chip soft">{item.primary_context.clone()}</span>
                <span class="chip soft">{item.source_path.clone()}</span>
            </div>
            <div class="panel-head search-result-foot">
                <div class="tag-row compact-tags">
                    {item
                        .tags
                        .into_iter()
                        .map(|tag| view! { <span class="chip soft">{tag}</span> })
                        .collect_view()}
                </div>
                <span class="meta-label">{format!("关联 {} 项", item.related_count)}</span>
            </div>
        </A>
    }
}

#[component]
fn AdminContentDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = Memo::new(move |_| params.with(|map| map.get("id").unwrap_or_default()));
    let detail = Resource::new_blocking(
        move || id.get(),
        |current_id| async move { get_admin_content_detail(current_id).await },
    );

    view! {
        <Title text="内容详情 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看后台中的单条内容详情、来源信息、关联项与问题摘要。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="内容详情 | Wen's Field Notes".to_string()
            description="查看后台中的单条内容详情、来源信息、关联项与问题摘要。".to_string()
            canonical_path="/admin/content".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"内容详情"</div>
                    <h2>"单条内容先提供治理视图，再决定后续是否进入正式写入流。"</h2>
                </div>
                <p>"这个详情页现在重点是数据边界、来源、关联和问题摘要，不是在线编辑器。"</p>
            </div>

            <div class="tag-row">
                <A href="/admin" attr:class="chip soft">"返回后台概览"</A>
                <A href="/admin/content" attr:class="chip soft">"返回内容列表"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在加载内容详情..." /> }>
                {move || {
                    detail.get().map(|result| match result {
                        Ok(detail) => view! { <AdminContentDetailContent detail=detail /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminContentDetailContent(detail: AdminContentDetail) -> impl IntoView {
    let item = detail.item.clone();

    view! {
        <div class="admin-grid">
            <div class="editorial-card admin-block">
                <div class="panel-head">
                    <div>
                        <span class="meta-label">{format!("{} · {}", item.content_type, item.status_label)}</span>
                        <h3>{item.title.clone()}</h3>
                    </div>
                    <A href=item.public_href.clone() attr:class="button ghost">"查看公开页"</A>
                </div>
                <p>{item.summary.clone()}</p>
                <div class="entry-card-list admin-facts-grid">
                    {detail
                        .facts
                        .into_iter()
                        .map(|fact| view! { <AdminFactCard fact=fact /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"问题摘要"</span>
                    <h3>"这一层先告诉我们哪里还需要治理，而不是直接触发审核流。"</h3>
                </div>
                {if detail.issues.is_empty() {
                    view! {
                        <div class="note-card">
                            <span class="meta-label">"当前无问题"</span>
                            <p>"按照当前后台规则，这条内容没有额外提示项。"</p>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="entry-card-list">
                            {detail
                                .issues
                                .into_iter()
                                .map(|issue| view! { <AdminIssueCard issue=issue /> })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"关联内容"</span>
                    <h3>"这里沿用现有的关联逻辑，把公开站点里的内容关系同步展示到后台视图。"</h3>
                </div>
                {if detail.related.is_empty() {
                    view! {
                        <div class="note-card">
                            <span class="meta-label">"暂无关联"</span>
                            <p>"当前没有可展示的关联项。"</p>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="archive-card-list">
                            {detail
                                .related
                                .into_iter()
                                .map(|related| {
                                    view! {
                                        <A href=related.href.clone() attr:class="archive-card related-card">
                                            <p class="blog-meta">{format!("{} · {}", related.content_type, related.context)}</p>
                                            <h3>{related.title}</h3>
                                            <p>{related.summary}</p>
                                            <span class="meta-label">{related.reason}</span>
                                        </A>
                                    }
                                })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"衔接说明"</span>
                    <h3>"这几条说明用来明确当前后台的职责边界。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {detail
                        .bridge_notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn AdminFactCard(fact: AdminContentFact) -> impl IntoView {
    view! {
        <div class="note-card admin-fact-card">
            <span class="meta-label">{fact.label}</span>
            <p>{fact.value}</p>
        </div>
    }
}

#[component]
fn AdminIssueCard(issue: AdminContentIssue) -> impl IntoView {
    view! {
        <div class="note-card admin-issue-card">
            <span class="meta-label">{format!("{} · {}", issue.severity_label, issue.code)}</span>
            <p>{issue.message}</p>
        </div>
    }
}

#[component]
fn AdminSearchPage() -> impl IntoView {
    let query_map = use_query_map();
    let sample_query =
        Memo::new(move |_| query_map.with(|map| map.get("sample").unwrap_or_default()));
    let rebuild_token =
        Memo::new(move |_| query_map.with(|map| map.get("rebuild").unwrap_or_default()));
    let rebuild_result = Resource::new(
        move || rebuild_token.get(),
        |token| async move {
            if token.trim().is_empty() {
                Ok(None)
            } else {
                rebuild_search_index("manual-admin".to_string())
                    .await
                    .map(Some)
            }
        },
    );
    let overview = Resource::new(
        move || (sample_query.get(), rebuild_token.get()),
        |(sample, _token)| async move { get_admin_search_overview(sample).await },
    );

    view! {
        <Title text="搜索后台 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看搜索索引状态、重建记录、运行态依赖与查询诊断。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="搜索后台 | Wen's Field Notes".to_string()
            description="查看搜索索引状态、重建记录、运行态依赖与查询诊断。".to_string()
            canonical_path="/admin/search".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"搜索后台"</div>
                    <h2>"这里负责确认搜索索引已经真实落库，并且可以随时重建。"</h2>
                </div>
                <p>"这里会直接告诉我们 MySQL 和 Redis 是否可用、索引有没有落库、最近一次重建发生了什么。"</p>
            </div>

            <form action="/admin/search" method="get" class="search-form admin-filter-form">
                <label class="search-label" for="search-sample">
                    "示例查询"
                </label>
                <div class="search-form-row">
                    <input
                        id="search-sample"
                        class="search-input"
                        type="search"
                        name="sample"
                        value=sample_query.get_untracked()
                        placeholder="例如：Rust、PRD、Leptos"
                    />
                    <button type="submit" class="button primary">"刷新诊断"</button>
                </div>
                <div class="tag-row">
                    <A href="/admin" attr:class="chip soft">"返回后台概览"</A>
                    <A href="/admin/content" attr:class="chip soft">"返回内容后台"</A>
                    <a href=format!("/admin/search?sample={}&rebuild={}", sample_query.get_untracked(), chrono::Utc::now().timestamp()) class="chip active">"执行一次重建"</a>
                </div>
            </form>

            <Suspense fallback=move || view! { <PageLoading label="正在检查搜索基础设施..." /> }>
                {move || {
                    overview.get().map(|result| match result {
                        Ok(overview) => {
                            let rebuild_feedback = rebuild_result
                                .get()
                                .and_then(|value| value.ok().flatten());
                            view! {
                                <AdminSearchContent overview=overview rebuild_feedback=rebuild_feedback />
                            }
                                .into_any()
                        }
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminSearchContent(
    overview: AdminSearchOverview,
    rebuild_feedback: Option<SearchRebuildRecord>,
) -> impl IntoView {
    view! {
        <div class="admin-grid">
            {rebuild_feedback
                .map(|record| {
                    view! {
                        <div class="note-card warm admin-block">
                            <span class="meta-label">"最近一次手动重建"</span>
                            <h3>{format!("{} · {} 条", record.status, record.document_count)}</h3>
                            <p>{record.message}</p>
                        </div>
                    }
                })}

            <div class="entry-card-list stats-grid">
                {overview
                    .stats
                    .into_iter()
                    .map(|stat| view! { <AdminStatCard stat=stat /> })
                    .collect_view()}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"重建记录"</span>
                    <h3>"先让索引重建可见，再进入任务系统。"</h3>
                </div>
                {if overview.rebuild_records.is_empty() {
                    view! {
                        <div class="note-card">
                            <span class="meta-label">"暂无记录"</span>
                            <p>"当前还没有持久化重建记录。通常是 MySQL 尚未连通，或者还没有执行过重建。"</p>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="entry-card-list">
                            {overview
                                .rebuild_records
                                .into_iter()
                                .map(|record| view! { <SearchRebuildRecordCard record=record /> })
                                .collect_view()}
                        </div>
                    }
                        .into_any()
                }}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"查询诊断"</span>
                    <h3>"同一个查询，当前到底是落到持久化索引还是实时回退。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .diagnostics
                        .into_iter()
                        .map(|diagnostic| view! { <SearchDiagnosticCard diagnostic=diagnostic /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"覆盖说明"</span>
                    <h3>"这一层明确告诉我们当前索引实际覆盖到了哪些内容。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {overview
                        .coverage_notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn SearchRebuildRecordCard(record: SearchRebuildRecord) -> impl IntoView {
    view! {
        <div class="note-card admin-fact-card">
            <span class="meta-label">{format!("{} · {}", record.status, record.trigger)}</span>
            <p>{record.message.clone()}</p>
            <small class="board-footnote">
                {format!("开始：{} · 完成：{} · 文档数：{}", record.started_at, record.finished_at.unwrap_or_else(|| "未完成".to_string()), record.document_count)}
            </small>
        </div>
    }
}

#[component]
fn SearchDiagnosticCard(diagnostic: SearchQueryDiagnostic) -> impl IntoView {
    view! {
        <div class="note-card admin-block">
            <span class="meta-label">{format!("{} · {} 条结果", diagnostic.mode, diagnostic.result_count)}</span>
            <h3>{format!("示例查询：{}", diagnostic.query)}</h3>
            <div class="entry-card-list">
                {diagnostic
                    .top_results
                    .into_iter()
                    .map(|result| {
                        view! {
                            <A href=result.href.clone() attr:class="archive-card search-result-card">
                                <p class="blog-meta">{format!("{} · {}", result.content_type, result.context)}</p>
                                <h3>{result.title}</h3>
                                <p>{result.summary}</p>
                                <span class="meta-label">{result.match_hint}</span>
                            </A>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn AdminStatsPage() -> impl IntoView {
    let overview =
        Resource::new_blocking(|| (), |_| async move { get_admin_stats_overview().await });

    view! {
        <Title text="统计后台 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看统计快照、治理指标与持久化统计结果。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="统计后台 | Wen's Field Notes".to_string()
            description="查看统计快照、治理指标与持久化统计结果。".to_string()
            canonical_path="/admin/stats".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"统计后台"</div>
                    <h2>"这里负责把治理指标写入 MySQL，方便核对站点当前状态。"</h2>
                </div>
                <p>"这里的统计当前偏内部运营视角，先服务内容后台、任务系统和同步边界。"</p>
            </div>

            <div class="tag-row">
                <A href="/admin" attr:class="chip soft">"后台概览"</A>
                <A href="/admin/tasks" attr:class="chip soft">"任务记录"</A>
                <A href="/admin/sync" attr:class="chip soft">"同步边界"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在加载统计快照..." /> }>
                {move || {
                    overview.get().map(|result| match result {
                        Ok(overview) => view! { <AdminStatsContent overview=overview /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminStatsContent(overview: AdminStatsOverview) -> impl IntoView {
    view! {
        <div class="admin-grid">
            <div class="entry-card-list stats-grid">
                {overview
                    .stats
                    .into_iter()
                    .map(|stat| view! { <AdminStatCard stat=stat /> })
                    .collect_view()}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"统计快照"</span>
                    <h3>"这里先持久化当前状态，而不是做复杂时间序列分析。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .snapshots
                        .into_iter()
                        .map(|snapshot| view! { <MetricSnapshotCard snapshot=snapshot /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"范围说明"</span>
                    <h3>"统计页只承担站点治理与运行核对，不扩展成完整 BI 系统。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {overview
                        .notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn MetricSnapshotCard(snapshot: MetricSnapshot) -> impl IntoView {
    view! {
        <div class="note-card admin-fact-card">
            <span class="meta-label">{snapshot.metric_key}</span>
            <h3>{snapshot.metric_value}</h3>
            <p>{snapshot.detail}</p>
            <small class="board-footnote">{format!("更新时间：{}", snapshot.captured_at)}</small>
        </div>
    }
}

#[component]
fn AdminTasksPage() -> impl IntoView {
    let overview =
        Resource::new_blocking(|| (), |_| async move { get_admin_tasks_overview().await });

    view! {
        <Title text="任务后台 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看任务记录、重建动作和服务端执行历史。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="任务后台 | Wen's Field Notes".to_string()
            description="查看任务记录、重建动作和服务端执行历史。".to_string()
            canonical_path="/admin/tasks".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"任务后台"</div>
                    <h2>"先把搜索重建、同步运行这些动作变成正式记录。"</h2>
                </div>
                <p>"当前仍然采用应用内触发任务的方式，不引入独立 worker 和复杂编排。"</p>
            </div>

            <div class="tag-row">
                <A href="/admin/search" attr:class="chip soft">"搜索索引"</A>
                <A href="/admin/stats" attr:class="chip soft">"统计快照"</A>
                <A href="/admin/sync" attr:class="chip soft">"同步边界"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在加载任务记录..." /> }>
                {move || {
                    overview.get().map(|result| match result {
                        Ok(overview) => view! { <AdminTasksContent overview=overview /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminTasksContent(overview: AdminTasksOverview) -> impl IntoView {
    view! {
        <div class="admin-grid">
            <div class="entry-card-list stats-grid">
                {overview
                    .stats
                    .into_iter()
                    .map(|stat| view! { <AdminStatCard stat=stat /> })
                    .collect_view()}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"任务记录"</span>
                    <h3>"统一任务表先收拢后台动作，再决定是否拆 worker。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .tasks
                        .into_iter()
                        .map(|task| view! { <TaskRunCard task=task /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"范围说明"</span>
                    <h3>"任务页当前聚焦执行记录与结果追踪，不扩展为复杂调度系统。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {overview
                        .notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn TaskRunCard(task: TaskRunRecord) -> impl IntoView {
    view! {
        <div class="note-card admin-fact-card">
            <span class="meta-label">{format!("{} · {}", task.task_type, task.status)}</span>
            <h3>{task.summary}</h3>
            <small class="board-footnote">
                {format!(
                    "触发：{} · 开始：{} · 完成：{}",
                    task.trigger,
                    task.started_at,
                    task.finished_at.unwrap_or_else(|| "未完成".to_string())
                )}
            </small>
        </div>
    }
}

#[component]
fn AdminSyncPage() -> impl IntoView {
    let query_map = use_query_map();
    let source_key =
        Memo::new(move |_| query_map.with(|map| map.get("source").unwrap_or_default()));
    let run_token = Memo::new(move |_| query_map.with(|map| map.get("run").unwrap_or_default()));
    let run_result = Resource::new(
        move || (source_key.get(), run_token.get()),
        |(source, token)| async move {
            if source.trim().is_empty() || token.trim().is_empty() {
                Ok(None)
            } else {
                run_sync_source(source, "manual-admin".to_string())
                    .await
                    .map(Some)
            }
        },
    );
    let overview = Resource::new(
        move || (source_key.get(), run_token.get()),
        |_| async move { get_admin_sync_overview().await },
    );

    view! {
        <Title text="同步后台 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看同步边界、同步源与运行记录。"
        />
        <Meta name="robots" content="noindex,follow" />
        <PageHeadExtras
            title="同步后台 | Wen's Field Notes".to_string()
            description="查看同步边界、同步源与运行记录。".to_string()
            canonical_path="/admin/sync".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section admin-shell">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"同步后台"</div>
                    <h2>"这里负责确认同步源、同步记录和任务链路已经可以最小可用地跑通。"</h2>
                </div>
                <p>"当前同步页重点是记录来源、执行结果和失败信息，不扩展到完整第三方开放平台。"</p>
            </div>

            <div class="tag-row">
                <A href="/admin/search" attr:class="chip soft">"搜索索引"</A>
                <A href="/admin/tasks" attr:class="chip soft">"任务记录"</A>
                <A href="/admin/stats" attr:class="chip soft">"统计快照"</A>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在加载同步边界..." /> }>
                {move || {
                    overview.get().map(|result| match result {
                        Ok(overview) => {
                            let run_feedback = run_result.get().and_then(|value| value.ok().flatten());
                            view! { <AdminSyncContent overview=overview run_feedback=run_feedback /> }.into_any()
                        }
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AdminSyncContent(
    overview: AdminSyncOverview,
    run_feedback: Option<SyncRunRecord>,
) -> impl IntoView {
    view! {
        <div class="admin-grid">
            {run_feedback
                .map(|run| {
                    view! {
                        <div class="note-card warm admin-block">
                            <span class="meta-label">"最近一次手动同步"</span>
                            <h3>{format!("{} · {}", run.source_key, run.status)}</h3>
                            <p>{run.summary}</p>
                        </div>
                    }
                })}

            <div class="entry-card-list stats-grid">
                {overview
                    .stats
                    .into_iter()
                    .map(|stat| view! { <AdminStatCard stat=stat /> })
                    .collect_view()}
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"同步源"</span>
                    <h3>"每个同步源都先成为可解释、可记录的正式边界。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .sources
                        .into_iter()
                        .map(|source| view! { <SyncSourceCard source=source /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"同步记录"</span>
                    <h3>"先看到运行结果，再决定未来是否需要真正异步化。"</h3>
                </div>
                <div class="entry-card-list">
                    {overview
                        .runs
                        .into_iter()
                        .map(|run| view! { <SyncRunCard run=run /> })
                        .collect_view()}
                </div>
            </div>

            <div class="editorial-card admin-block">
                <div>
                    <span class="meta-label">"范围说明"</span>
                    <h3>"同步页当前只覆盖 `v1.0` 所需的最小闭环。"</h3>
                </div>
                <ul class="admin-notes-list">
                    {overview
                        .notes
                        .into_iter()
                        .map(|note| view! { <li>{note}</li> })
                        .collect_view()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn SyncSourceCard(source: SyncSourceRecord) -> impl IntoView {
    let run_href = format!(
        "/admin/sync?source={}&run={}",
        source.source_key,
        chrono::Utc::now().timestamp()
    );

    view! {
        <div class="note-card admin-block">
            <div class="panel-head">
                <span class="meta-label">{format!("{} · {}", source.direction, source.status)}</span>
                <a href=run_href class="chip active">"执行一次同步"</a>
            </div>
            <h3>{source.label}</h3>
            <p>{source.notes}</p>
            <small class="board-footnote">
                {format!(
                    "{} · 最近运行：{}",
                    source.endpoint,
                    source.last_run_at.unwrap_or_else(|| "尚未运行".to_string())
                )}
            </small>
        </div>
    }
}

#[component]
fn SyncRunCard(run: SyncRunRecord) -> impl IntoView {
    view! {
        <div class="note-card admin-fact-card">
            <span class="meta-label">{format!("{} · {}", run.source_key, run.status)}</span>
            <h3>{run.summary}</h3>
            <small class="board-footnote">
                {format!(
                    "触发：{} · 开始：{} · 完成：{}",
                    run.trigger,
                    run.started_at,
                    run.finished_at.unwrap_or_else(|| "未完成".to_string())
                )}
            </small>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <Title text="关于 | Wen's Field Notes" />
        <Meta
            name="description"
            content="了解这个个人内容站为什么存在、如何推进，以及 `v1.0` 当前明确的范围边界。"
        />
        <PageHeadExtras
            title="关于 | Wen's Field Notes".to_string()
            description="了解这个个人内容站为什么存在、如何推进，以及 `v1.0` 当前明确的范围边界。".to_string()
            canonical_path="/about".to_string()
        />
        <section class="preview-section about-section">
            <div class="section-kicker">"关于"</div>
            <div class="about-layout">
                <div class="about-copy">
                    <h2>"我想把这个网站做成一份持续更新的公开工作现场，而不是一张静态名片。"</h2>
                    <p>"这个仓库里会同时放产品文档、学习文档和正式代码。Rust 学习不是和项目分开的副线，而是直接在真实实现里推进。"</p>
                    <p>"这个版本以可部署上线为目标，已经把数据库、搜索、后台、统计、任务和同步链路接了起来，但仍然刻意不扩展到评论、用户系统和平台化能力。"</p>
                    <p>"如果你想直接看我最近在做什么，请去 `/me`；如果你想理解这个站为什么这样组织、为什么暂时不引入更重的系统能力，这一页更适合。"</p>
                </div>

                <div class="about-sidebar">
                    <div class="identity-card">
                        <span class="meta-label">"关键词"</span>
                        <ul>
                            <li>"Rust 初学者"</li>
                            <li>"长期项目主义"</li>
                            <li>"内容驱动网站"</li>
                            <li>"边做边学"</li>
                        </ul>
                    </div>

                    <div class="identity-card contact">
                        <span class="meta-label">"当前边界"</span>
                        <p>"`v1.0` 重点是把现有内容站稳定落地，不新增用户体系、评论社区、开放平台和复杂工作流。"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="404 | Wen's Field Notes" />
        <PageHeadExtras
            title="404 | Wen's Field Notes".to_string()
            description="这个页面还没有被接进当前版本。".to_string()
            canonical_path="/404".to_string()
            robots="noindex,follow".to_string()
        />
        <section class="preview-section">
            <div class="section-kicker">"404"</div>
            <h2>"这个页面还没有被接进当前版本。"</h2>
            <p class="lede">"现在正式开放的有首页、个人主页、标签总览、归档、系列页、博客、笔记、项目、搜索、标签归档和关于页。"</p>
            <A href="/" attr:class="button primary">"回到首页"</A>
        </section>
    }
}

#[component]
fn PageLoading(label: &'static str) -> impl IntoView {
    view! {
        <div class="loading-card">
            <span class="meta-label">"加载中"</span>
            <p>{label}</p>
        </div>
    }
}

#[component]
fn PageError(message: String) -> impl IntoView {
    view! {
        <div class="loading-card error-card">
            <span class="meta-label">"内容加载失败"</span>
            <p>{message}</p>
        </div>
    }
}

fn format_meta_line(date: &NaiveDate, tags: &[String]) -> String {
    let date_text = date.format("%Y.%m.%d").to_string();
    let tag_text = if tags.is_empty() {
        "未分类".to_string()
    } else {
        tags.join(" · ")
    };

    format!("{date_text} · {tag_text}")
}

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

fn display_search_type(value: &str) -> &str {
    match value {
        "blog" => "博客",
        "notes" => "笔记",
        "projects" => "项目",
        _ => value,
    }
}

fn normalize_note_board(value: &str) -> String {
    match value.trim().to_lowercase().as_str() {
        "rust" => "rust".to_string(),
        "c++" | "cpp" => "cpp".to_string(),
        "bochs" => "bochs".to_string(),
        "general" | "" => "general".to_string(),
        other => other.to_string(),
    }
}

fn note_board_label(value: &str) -> &'static str {
    match normalize_note_board(value).as_str() {
        "rust" => "Rust",
        "cpp" => "C++",
        "bochs" => "Bochs",
        _ => "通用笔记",
    }
}

fn note_board_description(value: &str) -> &'static str {
    match normalize_note_board(value).as_str() {
        "rust" => "偏 Rust 学习、语义理解和项目实践记录。",
        "cpp" => "收纳 C++ 相关笔记、语法复盘与底层实验记录。",
        "bochs" => "收纳 Bochs、操作系统实验和调试记录。",
        _ => "暂时还没归入专门技术板块的过程型笔记。",
    }
}

fn highlight_text_html(text: &str, query: &str) -> String {
    if query.trim().is_empty() {
        return escape_html(text);
    }

    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();
    let mut cursor = 0;
    let mut html = String::new();

    while let Some(found) = lower_text[cursor..].find(&lower_query) {
        let start = cursor + found;
        let end = start + lower_query.len();
        html.push_str(&escape_html(&text[cursor..start]));
        html.push_str("<mark>");
        html.push_str(&escape_html(&text[start..end]));
        html.push_str("</mark>");
        cursor = end;
    }

    html.push_str(&escape_html(&text[cursor..]));
    html
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

#[server(GetHomeOverview, "/api")]
pub async fn get_home_overview() -> Result<HomeOverview, ServerFnError> {
    content::get_home_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetTagsOverview, "/api")]
pub async fn get_tags_overview() -> Result<TagsOverview, ServerFnError> {
    content::get_tags_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetArchiveOverview, "/api")]
pub async fn get_archive_overview() -> Result<ArchiveOverview, ServerFnError> {
    content::get_archive_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(ListBlogPosts, "/api")]
pub async fn list_blog_posts() -> Result<Vec<BlogPostSummary>, ServerFnError> {
    content::list_blog_posts()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetBlogPost, "/api")]
pub async fn get_blog_post(slug: String) -> Result<BlogPost, ServerFnError> {
    content::get_blog_post(&slug)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(ListNoteEntries, "/api")]
pub async fn list_note_entries() -> Result<Vec<NoteSummary>, ServerFnError> {
    content::list_note_entries()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetNoteEntry, "/api")]
pub async fn get_note_entry(slug: String) -> Result<NoteEntry, ServerFnError> {
    content::get_note_entry(&slug)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(ListProjectEntries, "/api")]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>, ServerFnError> {
    content::list_project_entries()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetProjectEntry, "/api")]
pub async fn get_project_entry(slug: String) -> Result<ProjectEntry, ServerFnError> {
    content::get_project_entry(&slug)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetTagArchive, "/api")]
pub async fn get_tag_archive(tag: String) -> Result<TagArchive, ServerFnError> {
    content::get_tag_archive(&tag)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetSeriesPage, "/api")]
pub async fn get_series_page(slug: String) -> Result<SeriesPage, ServerFnError> {
    content::get_series_page(&slug)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(SearchContent, "/api")]
pub async fn search_content(
    query: String,
    type_filter: String,
    tag_filter: String,
) -> Result<Vec<SearchResult>, ServerFnError> {
    let type_filter = if type_filter.trim().is_empty() {
        None
    } else {
        Some(type_filter.as_str())
    };
    let tag_filter = if tag_filter.trim().is_empty() {
        None
    } else {
        Some(tag_filter.as_str())
    };

    content::search_content(&query, type_filter, tag_filter)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminDashboardOverview, "/api")]
pub async fn get_admin_dashboard_overview() -> Result<AdminDashboardOverview, ServerFnError> {
    content::get_admin_dashboard_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(ListAdminContent, "/api")]
pub async fn list_admin_content(
    query: String,
    type_filter: String,
    status_filter: String,
) -> Result<Vec<AdminContentListItem>, ServerFnError> {
    let type_filter = if type_filter.trim().is_empty() {
        None
    } else {
        Some(type_filter.as_str())
    };
    let status_filter = if status_filter.trim().is_empty() {
        None
    } else {
        Some(status_filter.as_str())
    };

    content::list_admin_content(&query, type_filter, status_filter)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminContentDetail, "/api")]
pub async fn get_admin_content_detail(id: String) -> Result<AdminContentDetail, ServerFnError> {
    content::get_admin_content_detail(&id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetNoteBoardsOverview, "/api")]
pub async fn get_note_boards_overview() -> Result<Vec<NoteBoardSummary>, ServerFnError> {
    content::get_note_boards_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminSearchOverview, "/api")]
pub async fn get_admin_search_overview(
    sample_query: String,
) -> Result<AdminSearchOverview, ServerFnError> {
    content::get_admin_search_overview(&sample_query)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminStatsOverview, "/api")]
pub async fn get_admin_stats_overview() -> Result<AdminStatsOverview, ServerFnError> {
    content::get_admin_stats_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminTasksOverview, "/api")]
pub async fn get_admin_tasks_overview() -> Result<AdminTasksOverview, ServerFnError> {
    content::get_admin_tasks_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(GetAdminSyncOverview, "/api")]
pub async fn get_admin_sync_overview() -> Result<AdminSyncOverview, ServerFnError> {
    content::get_admin_sync_overview()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(RebuildSearchIndex, "/api")]
pub async fn rebuild_search_index(trigger: String) -> Result<SearchRebuildRecord, ServerFnError> {
    content::rebuild_search_index(&trigger)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[server(RunSyncSource, "/api")]
pub async fn run_sync_source(
    source_key: String,
    trigger: String,
) -> Result<SyncRunRecord, ServerFnError> {
    content::run_sync_source(&source_key, &trigger)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}
