pub mod content;

use chrono::NaiveDate;
use content::{
    BlogPost, BlogPostSummary, NoteEntry, NoteSummary, ProjectEntry, ProjectSummary, SearchResult,
    TagArchive, TagArchiveItem,
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
                        <Route path=path!("/blog") view=BlogListPage />
                        <Route path=path!("/blog/:slug") view=BlogDetailPage />
                        <Route path=path!("/notes") view=NotesPage />
                        <Route path=path!("/notes/:slug") view=NoteDetailPage />
                        <Route path=path!("/projects") view=ProjectsPage />
                        <Route path=path!("/projects/:slug") view=ProjectDetailPage />
                        <Route path=path!("/tags/:tag") view=TagArchivePage />
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
    let home_content = Resource::new_blocking(
        || (),
        |_| async move {
            let posts = latest_blog_posts().await;
            let notes = latest_note_entries().await;
            let project = featured_project().await;

            (posts, notes, project)
        },
    );

    view! {
        <Title text="首页 | Wen's Field Notes" />
        <Meta
            name="description"
            content="查看这个全栈 Rust 个人内容站的首页，快速了解博客、笔记、项目、搜索与当前阶段目标。"
        />
        <PageHeadExtras
            title="首页 | Wen's Field Notes".to_string()
            description="查看这个全栈 Rust 个人内容站的首页，快速了解博客、笔记、项目、搜索与当前阶段目标。".to_string()
            canonical_path="/".to_string()
        />
        <section class="preview-section hero">
            <div class="section-kicker">"首页"</div>
            <div class="hero-grid">
                <div class="hero-copy">
                    <p class="eyebrow">"正在学习 Rust，也在认真做一个长期更新的个人网站。"</p>
                    <h1>"把项目、笔记和正在形成的思考，做成一份持续公开生长的个人档案。"</h1>
                    <p class="lede">
                        "第二版开始补齐内容详情、标签归档、站内搜索和页面级 SEO，让这个站点从可用升级成更完整的公开内容档案。"
                    </p>
                    <div class="hero-actions">
                        <A href="/blog" attr:class="button primary">"阅读最新文章"</A>
                        <A href="/notes" attr:class="button ghost">"查看最近笔记"</A>
                        <A href="/search" attr:class="button ghost">"站内搜索"</A>
                    </div>
                </div>

                <aside class="hero-aside">
                    <div class="note-card warm">
                        <span class="meta-label">"当前主题"</span>
                        <h2>"Leptos + Rust 内容站"</h2>
                        <p>"先把 SSR 骨架、内容组织和页面气质做对，再逐步把第二版剩余链路补齐。"</p>
                    </div>

                    <div class="note-card">
                        <span class="meta-label">"这一轮目标"</span>
                        <ul>
                            <li>"projects 详情页、标签归档和搜索页正式接入"</li>
                            <li>"页面间跳转更自然，内容入口更明确"</li>
                            <li>"SEO 与公开访问质量继续提升"</li>
                        </ul>
                    </div>
                </aside>
            </div>

            <Suspense fallback=move || view! { <PageLoading label="正在整理首页内容..." /> }>
                {move || {
                    home_content.get().map(|(posts, notes, project)| match (posts, notes, project) {
                        (Ok(posts), Ok(notes), Ok(project)) => {
                            view! { <HomePanels posts=posts notes=notes project=project /> }.into_any()
                        }
                        (Err(error), _, _)
                        | (_, Err(error), _)
                        | (_, _, Err(error)) => {
                            view! { <PageError message=error.to_string() /> }.into_any()
                        }
                    })
                }}
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
                        <span class="meta-label">"代表项目"</span>
                        <A href="/projects">"项目页"</A>
                    </div>
                    {project
                        .map(|project| {
                            view! {
                                <A href=format!("/projects/{}", project.slug) attr:class="project-feature">
                                    <h3>{project.title}</h3>
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
                        <span class="meta-label">"第二版重点"</span>
                        <span>"把内容链路补完整"</span>
                    </div>
                    <p>"第二版不是把站点做成复杂平台，而是把已经存在的内容真正组织成一套可浏览、可发现、可长期公开访问的结构。"</p>
                    <div class="manifesto-list">
                        <span>"详情页让内容从列表项变成可独立阅读的档案。"</span>
                        <span>"标签与搜索让内容不只依赖首页被发现。"</span>
                        <span>"页面结构与 SEO 打磨让公开访问体验更稳定。"</span>
                    </div>
                </article>

                <article class="panel timeline-panel">
                    <div class="panel-head">
                        <span class="meta-label">"当前边界"</span>
                        <span>"v2 scope"</span>
                    </div>
                    <div class="timeline-list">
                        <span>"内容链路：`/blog/:slug`、`/notes/:slug`、`/projects/:slug`"</span>
                        <span>"发现能力：`/tags/:tag`、`/search`、扩展 sitemap"</span>
                        <span>"仍不接入：数据库、评论、后台、用户系统"</span>
                    </div>
                </article>
            </div>
        </>
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
                            <p class="blog-meta">{format_meta_line(&note.date, &note.tags)}</p>
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

                <div class="article-body project-article-body" inner_html=html></div>
            </article>
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
    let search_results = Resource::new(
        move || query.get(),
        |current_query| async move {
            search_content(current_query.clone())
                .await
                .map(|results| (current_query, results))
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

            <SearchForm query=query.get_untracked() />

            <Suspense fallback=move || view! { <PageLoading label="正在搜索内容..." /> }>
                {move || {
                    search_results.get().map(|result| match result {
                        Ok((current_query, results)) => {
                            view! { <SearchResultsContent query=current_query results=results /> }
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
fn SearchForm(query: String) -> impl IntoView {
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
        </form>
    }
}

#[component]
fn SearchResultsContent(query: String, results: Vec<SearchResult>) -> impl IntoView {
    let normalized_query = query.trim().to_string();

    if normalized_query.is_empty() {
        return view! {
            <div class="loading-card search-empty-card">
                <span class="meta-label">"等待输入"</span>
                <p>"输入关键词后，这里会展示博客、笔记和项目中的匹配结果。"</p>
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
                <span class="meta-label">{format!("搜索词：{}", normalized_query)}</span>
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
            content="了解这个个人内容站为什么存在、当前在做什么，以及它的第二版方向。"
        />
        <PageHeadExtras
            title="关于 | Wen's Field Notes".to_string()
            description="了解这个个人内容站为什么存在、当前在做什么，以及它的第二版方向。".to_string()
            canonical_path="/about".to_string()
        />
        <section class="preview-section about-section">
            <div class="section-kicker">"关于"</div>
            <div class="about-layout">
                <div class="about-copy">
                    <h2>"我想把这个网站做成一份持续更新的公开工作现场，而不是一张静态名片。"</h2>
                    <p>"这个仓库里会同时放产品文档、学习文档和正式代码。Rust 学习不是和项目分开的副线，而是直接在真实实现里推进。"</p>
                    <p>"第二版正在把内容详情、标签归档、站内搜索和页面级 SEO 逐步补齐，但数据库、评论、后台和用户系统仍然不在当前范围里。"</p>
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
                        <p>"第二版重点是完整浏览链路和公开访问质量，不接数据库、评论、搜索后台。"</p>
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
            <p class="lede">"现在正式开放的有首页、博客、笔记、项目、搜索、标签归档和关于页。"</p>
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

async fn latest_blog_posts() -> Result<Vec<BlogPostSummary>, ServerFnError> {
    list_blog_posts()
        .await
        .map(|posts| posts.into_iter().take(3).collect())
}

async fn latest_note_entries() -> Result<Vec<NoteSummary>, ServerFnError> {
    list_note_entries()
        .await
        .map(|notes| notes.into_iter().take(3).collect())
}

async fn featured_project() -> Result<Option<ProjectSummary>, ServerFnError> {
    list_project_entries()
        .await
        .map(|projects| projects.into_iter().next())
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

#[server(SearchContent, "/api")]
pub async fn search_content(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    content::search_content(&query)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}
