pub mod content;

use chrono::NaiveDate;
use content::{
    BlogPost, BlogPostSummary, NoteSummary, ProjectSummary,
};
use leptos::prelude::*;
use leptos_meta::{Meta, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::use_params_map,
    path,
};

#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;

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
                        <Route path=path!("/blog") view=BlogListPage />
                        <Route path=path!("/blog/:slug") view=BlogDetailPage />
                        <Route path=path!("/notes") view=NotesPage />
                        <Route path=path!("/projects") view=ProjectsPage />
                        <Route path=path!("/about") view=AboutPage />
                    </Routes>
                </main>
            </div>
        </Router>
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
                <NavLink href="/blog" label="博客" />
                <NavLink href="/notes" label="笔记" />
                <NavLink href="/projects" label="项目" />
                <NavLink href="/about" label="关于" />
            </nav>
        </header>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a href=href>
            {label}
        </a>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="首页 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看这个全栈 Rust 个人内容站的首页，快速了解博客、笔记、项目与当前阶段目标。"
        />
        <section class="preview-section hero">
            <div class="section-kicker">"首页"</div>
            <div class="hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"正在学习 Rust，也在认真做一个长期更新的个人网站。"</p>
                    <h1>"把项目、笔记和正在形成的思考，做成一份持续公开生长的个人档案。"</h1>
                    <p class="lede">
                        "现在正式进入 Leptos SSR 路线。第一版已经接通博客主链路，这一轮开始把 notes、projects 和基础站点输出补齐。"
                    </p>
                    <div class="hero-actions">
                        <A href="/blog" attr:class="button primary">"阅读最新文章"</A>
                        <A href="/notes" attr:class="button ghost">"查看最近笔记"</A>
                    </div>
                </div>

                <aside class="hero-aside">
                    <div class="note-card warm">
                        <span class="meta-label">"当前主题"</span>
                        <h2>"Leptos + Rust 内容站"</h2>
                        <p>"先把 SSR 骨架、内容组织和页面气质做对，再逐步把第一版剩余页面和站点能力补齐。"</p>
                    </div>

                    <div class="note-card">
                        <span class="meta-label">"这一轮目标"</span>
                        <ul>
                            <li>"notes 与 projects 正式接入"</li>
                            <li>"博客详情补上下篇导航"</li>
                            <li>"补基础 RSS 与 sitemap 输出"</li>
                        </ul>
                    </div>
                </aside>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在整理首页内容..." /> }>
                {Suspend::new(async move {
                    let posts = latest_blog_posts().await;
                    let notes = latest_note_entries().await;
                    let project = featured_project().await;

                    match (posts, notes, project) {
                        (Ok(posts), Ok(notes), Ok(project)) => {
                            view! { <HomePanels posts=posts notes=notes project=project /> }.into_any()
                        }
                        (Err(error), _, _)
                        | (_, Err(error), _)
                        | (_, _, Err(error)) => {
                            view! { <PageError message=error.to_string() /> }.into_any()
                        }
                    }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn HomePanels(
    posts: Vec<BlogPostSummary>,
    notes: Vec<NoteSummary>,
    project: Option<ProjectSummary>,
) -> impl IntoView {
    view! {
        <>
            <div class="home-panels">
                <article class="panel feature-panel">
                    <div class="panel-head">
                        <span class="meta-label">"最新博客"</span>
                        <A href="/blog">"更多文章"</A>
                    </div>

                    <div class="stack-list">
                        {posts
                            .into_iter()
                            .map(|post| {
                                view! {
                                    <A href=format!("/blog/{}", post.slug) attr:class="stack-item">
                                        <strong>{post.title}</strong>
                                        <span>{post.summary}</span>
                                    </A>
                                }
                            })
                            .collect_view()}
                    </div>
                </article>

                <article class="panel split-panel">
                    <div class="panel-head">
                        <span class="meta-label">"最近笔记"</span>
                        <A href="/notes">"全部笔记"</A>
                    </div>
                    <div class="mini-list">
                        {notes
                            .into_iter()
                            .map(|note| {
                                view! {
                                    <A href="/notes" attr:class="mini-list-link">
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
                        <span class="meta-label">"代表项目"</span>
                        <A href="/projects">"项目页"</A>
                    </div>
                    {project
                        .map(|project| {
                            view! {
                                <div class="project-feature">
                                    <h3>{project.title}</h3>
                                    <p>{project.stack.join(" / ")}</p>
                                    <small>{project.summary}</small>
                                </div>
                            }
                                .into_any()
                        })
                        .unwrap_or_else(|| {
                            view! { <p>"项目内容正在整理中。"</p> }.into_any()
                        })}
                </article>
            </div>

            <div class="home-reference-grid">
                <article class="panel manifesto-panel">
                    <div class="panel-head">
                        <span class="meta-label">"第一版重点"</span>
                        <span>"先把结构跑稳"</span>
                    </div>
                    <p>"这一版不是为了把功能堆满，而是先把一个长期可维护的全栈 Rust 内容站骨架立起来。"</p>
                    <div class="manifesto-list">
                        <span>"首页负责说明路线和当前阶段。"</span>
                        <span>"博客列表强调可扫描性与节奏感。"</span>
                        <span>"笔记与项目页补齐后，站点内容层次才真正完整。"</span>
                    </div>
                </article>

                <article class="panel timeline-panel">
                    <div class="panel-head">
                        <span class="meta-label">"当前边界"</span>
                        <span>"v1 scope"</span>
                    </div>
                    <div class="timeline-list">
                        <span>"正式实现：`/`、`/blog`、`/blog/:slug`、`/notes`、`/projects`、`/about`"</span>
                        <span>"站点输出：基础 RSS 与 sitemap"</span>
                        <span>"暂不接入：数据库、评论、搜索、后台"</span>
                    </div>
                </article>
            </div>
        </>
    }
}

#[component]
fn BlogListPage() -> impl IntoView {
    view! {
        <Title text="博客 | Wen's Field Notes" />
        <Meta
            name="description"
            content="按时间与标签浏览博客文章，查看这个个人内容站里已经正式发布的文章。"
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"博客列表"</div>
                    <h2>"正式文章应该像作品被陈列，而不是像日志被堆叠。"</h2>
                </div>
                <p>"这一版先做按时间排序、标签展示和阅读连续性，不接搜索、不接数据库，先把结构和节奏做清楚。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在载入博客列表..." /> }>
                {Suspend::new(async move {
                    match list_blog_posts().await {
                        Ok(posts) => view! { <BlogListContent posts=posts /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    }
                })}
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
                    .map(|tag| view! { <span class="chip">{tag}</span> })
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
    let slug = move || params.read().get("slug").unwrap_or_default();

    view! {
        <Suspense fallback=move || view! { <PageLoading label="正在载入文章..." /> }>
            {Suspend::new(async move {
                match get_blog_post(slug()).await {
                    Ok(post) => view! { <BlogDetailContent post=post /> }.into_any(),
                    Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn BlogDetailContent(post: BlogPost) -> impl IntoView {
    let html = post.html.clone();
    let title_text = format!("{} | Wen's Field Notes", post.title);
    let description_text = post.summary.clone();

    view! {
        <Title text=title_text />
        <Meta name="description" content=description_text />
        <section class="preview-section article-preview">
            <div class="section-heading article-head">
                <div>
                    <div class="section-kicker">"博客详情"</div>
                    <h2>"文章页的重点不是装饰，而是让内容被舒服地读完。"</h2>
                </div>

                <div class="article-nav-inline">
                    <A href="/blog">"返回列表"</A>
                    <A href="/about">"关于这个项目"</A>
                </div>
            </div>

            <article class="article-card">
                <header class="article-header">
                    <p class="blog-meta">{format_meta_line(&post.date, &post.tags)}</p>
                    <h3>{post.title.clone()}</h3>
                    <div class="tag-row">
                        {post
                            .tags
                            .iter()
                            .map(|tag| view! { <span class="chip soft">{tag.clone()}</span> })
                            .collect_view()}
                    </div>
                </header>

                <div class="article-body" inner_html=html></div>

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
    view! {
        <Title text="笔记 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看这个站点中的学习记录、实验结论与过程型笔记。"
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"笔记"</div>
                    <h2>"笔记比博客更轻、更快，也更接近学习过程本身。"</h2>
                </div>
                <p>"这里收集的是短判断、实验结论和阶段记录，不追求每篇都写成完整长文。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在载入笔记..." /> }>
                {Suspend::new(async move {
                    match list_note_entries().await {
                        Ok(notes) => view! { <NotesListContent notes=notes /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    }
                })}
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
                        <article class="note-entry">
                            <p class="blog-meta">{format_meta_line(&note.date, &note.tags)}</p>
                            <h3>{note.title}</h3>
                            <p>{note.summary}</p>
                        </article>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="项目 | Wen's Field Notes" />
        <Meta
            name="description"
            content="浏览这个站点中的项目展示，了解当前在做什么、用什么做、进行到哪一步。"
        />
        <section class="preview-section">
            <div class="section-heading">
                <div>
                    <div class="section-kicker">"项目"</div>
                    <h2>"项目页不是文章索引，而是把正在做的东西清楚地陈列出来。"</h2>
                </div>
                <p>"这一版先用卡片把项目目标、当前状态、技术栈和外部入口说明白。"</p>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在载入项目..." /> }>
                {Suspend::new(async move {
                    match list_project_entries().await {
                        Ok(projects) => view! { <ProjectsListContent projects=projects /> }.into_any(),
                        Err(error) => view! { <PageError message=error.to_string() /> }.into_any(),
                    }
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn ProjectsListContent(projects: Vec<ProjectSummary>) -> impl IntoView {
    view! {
        <div class="projects-grid">
            {projects
                .into_iter()
                .map(|project| {
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
                                    .unwrap_or_else(|| view! { <></> }.into_any())}
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
                                    .unwrap_or_else(|| view! { <></> }.into_any())}
                            </div>
                        </article>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <Title text="关于 | Wen's Field Notes" />
        <Meta
            name="description"
            content="了解这个个人内容站为什么存在、当前在做什么，以及它的第一版边界。"
        />
        <section class="preview-section about-section">
            <div class="section-kicker">"关于"</div>
            <div class="about-layout">
                <div class="about-copy">
                    <h2>"我想把这个网站做成一份持续更新的公开工作现场，而不是一张静态名片。"</h2>
                    <p>"这个仓库里会同时放产品文档、学习文档和正式代码。Rust 学习不是和项目分开的副线，而是直接在真实实现里推进。"</p>
                    <p>"现在已经正式切到 Leptos SSR 路线。博客、笔记、项目页和基础站点输出会逐步补齐，但数据库、评论、搜索和后台仍然不在第一版范围里。"</p>
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
                        <p>"第一版先完成正式内容站和基础输出能力，不接数据库、评论、搜索、后台。"</p>
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
        <section class="preview-section">
            <div class="section-kicker">"404"</div>
            <h2>"这个页面还没有被接进第一版。"</h2>
            <p class="lede">"现在正式开放的有首页、博客、笔记、项目和关于页。"</p>
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

async fn latest_blog_posts() -> Result<Vec<BlogPostSummary>, ServerFnError> {
    list_blog_posts().await.map(|posts| posts.into_iter().take(3).collect())
}

async fn latest_note_entries() -> Result<Vec<NoteSummary>, ServerFnError> {
    list_note_entries()
        .await
        .map(|notes| notes.into_iter().take(3).collect())
}

async fn featured_project() -> Result<Option<ProjectSummary>, ServerFnError> {
    list_project_entries().await.map(|projects| projects.into_iter().next())
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

#[server(ListProjectEntries, "/api")]
pub async fn list_project_entries() -> Result<Vec<ProjectSummary>, ServerFnError> {
    content::list_project_entries()
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}
