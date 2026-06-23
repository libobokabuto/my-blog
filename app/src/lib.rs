pub mod content;

use chrono::NaiveDate;
use content::{
    ArchiveOverview, ArchiveYearGroup, BlogPost, BlogPostSummary, HomeActivityItem, HomeOverview,
    HomeStat, NoteEntry, NoteSummary, ProjectEntry, ProjectSummary, RelatedContentItem,
    SearchResult, SeriesPage, TagArchive, TagArchiveItem, TagOverviewItem, TagsOverview,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::{use_params_map, use_query_map},
    path,
};

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
                    <p class="eyebrow">"第三版正在把这个站点推进成更完整的内容系统。"</p>
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
                        <h2>"第三版 · 个人主页优先"</h2>
                        <p>"这一轮先把 `/me`、首页、about、blog、notes、projects 的关系理顺，再继续推进内容关联、搜索和治理增强。"</p>
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
                        <span class="meta-label">"第三版重点"</span>
                        <span>"把内容组织成系统"</span>
                    </div>
                    <p>"第三版的起点不是继续堆页面，而是先把个人主页做成内容站枢纽，让首页、about、博客、笔记和项目在结构上开始彼此支撑。"</p>
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
                        <span>"当前不接数据库、后台、用户系统和持久化统计。"</span>
                        <span>"最近动态与统计先走内容聚合，为第四版后端能力预留结构。"</span>
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
                        "我在用 Rust、Leptos SSR 和 Markdown 把这个站点做成可长期维护的内容系统。第三版先解决内容入口、最近动态、内容组织和治理前置问题，再决定后端能力什么时候值得接进来。"
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
                <p>"这里收集的是短判断、实验结论和阶段记录，不追求每篇都写成完整长文，但现在已经能通过详情页、标签和搜索被更自然地找到。"</p>
            </div>

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
fn NotesListContent(notes: Vec<NoteSummary>) -> impl IntoView {
    view! {
        <div class="notes-grid">
            {notes
                .into_iter()
                .map(|note| {
                    view! {
                        <A href=format!("/notes/{}", note.slug) attr:class="note-entry">
                            <p class="blog-meta">
                                {format!("{} · {}", format_meta_line(&note.date, &note.tags), note.stage)}
                            </p>
                            <h3>{note.title}</h3>
                            <p>{note.summary}</p>
                            <div class="tag-row compact-tags">
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
                    <span>"第三版阶段二"</span>
                </div>
                <div class="timeline-list">
                    <span>"当前时间归档只覆盖博客与笔记，因为它们已经具备稳定日期字段。"</span>
                    <span>"项目内容是否纳入时间归档，放到第三版内容模型增强时统一处理。"</span>
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
                <p>"搜索目前覆盖博客、笔记和项目，先把标题、摘要、正文和关键词检索做稳，不引入数据库。"</p>
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
fn AboutPage() -> impl IntoView {
    view! {
        <Title text="关于 | Wen's Field Notes" />
        <Meta
            name="description"
            content="了解这个个人内容站为什么存在、如何推进，以及第三版当前明确的范围边界。"
        />
        <PageHeadExtras
            title="关于 | Wen's Field Notes".to_string()
            description="了解这个个人内容站为什么存在、如何推进，以及第三版当前明确的范围边界。".to_string()
            canonical_path="/about".to_string()
        />
        <section class="preview-section about-section">
            <div class="section-kicker">"关于"</div>
            <div class="about-layout">
                <div class="about-copy">
                    <h2>"我想把这个网站做成一份持续更新的公开工作现场，而不是一张静态名片。"</h2>
                    <p>"这个仓库里会同时放产品文档、学习文档和正式代码。Rust 学习不是和项目分开的副线，而是直接在真实实现里推进。"</p>
                    <p>"第三版当前先做个人主页、内容组织、关联、搜索和治理增强。数据库、评论、后台和用户系统仍然明确不在当前范围里。"</p>
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
                        <p>"第三版重点是内容系统增强，不接数据库、持久化统计、后台、任务系统和外部数据同步。"</p>
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
