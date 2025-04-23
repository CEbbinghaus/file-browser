use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use either::Either;
use hypertext::{html_elements, Attribute, GlobalAttributes, RenderIterator, Renderable};
use std::{ffi::OsString, fs, path::PathBuf};
use ui::page;

mod packages;
mod ui;

#[allow(dead_code)]
impl<T: GlobalAttributes> HtmxAttributes for T {}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
trait HtmxAttributes: GlobalAttributes {
    const hx_boost: Attribute = Attribute;
}

#[derive(Debug, Clone)]
struct FileSystemEntry {
    name: OsString,
    path: PathBuf,
    is_dir: bool,
}

impl FileSystemEntry {
    fn from_path(path: &str) -> Option<Self> {
        let path = match std::path::absolute(path.strip_suffix('/').unwrap_or(path)) {
            Ok(path) => path,
            Err(err) => {
                println!("Invalid path: {path} - {err}");
                return None;
            }
        };

        if !path.exists() {
            return None;
        }

        let name = path.file_name()?.to_os_string();
        let is_dir = path.is_dir();

        Some(Self { name, path, is_dir })
    }

    fn list_files(&self) -> Option<Vec<Self>> {
        if !self.is_dir {
            return None;
        }

        let entries = match fs::read_dir(&self.path) {
            Ok(entries) => entries,
            Err(_) => return None,
        };

        Some(
            entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;

                    Some(Self {
                        name: entry.file_name(),
                        path: entry.path(),
                        is_dir: entry.file_type().ok()?.is_dir(),
                    })
                })
                .collect(),
        )
    }

    fn read_content(&self) -> Option<String> {
        if self.is_dir {
            return None;
        }

        fs::read_to_string(&self.path).ok()
    }
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/files/", get(files))
        .route("/files/{*path}", get(files))
        .route("/settings/", get(settings));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    (
        StatusCode::TEMPORARY_REDIRECT,
        [(header::LOCATION, "/files/")],
        "Redirecting...",
    )
}

async fn settings() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        page(hypertext::rsx! {
            <h1>Settings</h1>
        })
        .render()
        .into_inner(),
    )
}

async fn files(path: Option<Path<String>>) -> impl IntoResponse {
    let path = path.map(|path| path.to_owned()).unwrap_or(".".into());
    let Some(result) = list_files(&path) else {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html")],
            "Not Found".into(),
        );
    };

    let content = page(hypertext::rsx_move! {
        <div>
            {
                TestRenderable(&path, &result)
            }
        </div>
    });

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        content.render().into_inner(),
    )
}

fn list_files(path: &str) -> Option<Either<Vec<FileSystemEntry>, FileSystemEntry>> {
    let fs_entry = FileSystemEntry::from_path(path)?;

    if !fs_entry.is_dir {
        return Some(Either::Right(fs_entry));
    }

    Some(Either::Left(fs_entry.list_files()?))
}

struct TestRenderable<'a>(
    &'a String,
    &'a Either<Vec<FileSystemEntry>, FileSystemEntry>,
);

impl Renderable for TestRenderable<'_> {
    fn render_to(self, output: &mut String) {
        match self.1 {
            Either::Left(entries) => {
                render_list(self.0, entries).render_to(output);
            }
            Either::Right(entry) => {
                render_item(entry).render_to(output);
            }
        }
    }
}

struct MonacoRenderable(String);

impl Renderable for MonacoRenderable {
    fn render_to(self, output: &mut String) {
        output.push_str(
            "<script>
                htmx.onLoad(function (target) {
                    let container = target.querySelector(\"p.container\");
                    let value = container.innerText;
                    container.innerText = \"\";
                    window.editor = monaco.editor.create(container, {
                        value,
                        automaticLayout: true,
                        useShadowDOM: true,
                    });
                });
            </script>
            ",
        );

        hypertext::rsx! {
            <p class="container whitespace-pre-wrap h-full pt-4">{self.0}</p>
        }
        .render_to(output);
    }
}

fn render_item<'a>(item: &'a FileSystemEntry) -> impl Renderable + 'a {
    hypertext::rsx! {
        <h1 class="font-bold text-xl"><i class="fa-regular fa-file pr-4"></i> {item.name.to_string_lossy()}</h1>
        {
            MonacoRenderable(match item.read_content() {
            Some(content) => content,
            None => "Binary...".to_string(),
        })
        }
        <p class="container"></p>
    }
}

fn render_list<'a>(path: &'a String, items: &'a Vec<FileSystemEntry>) -> impl Renderable + 'a {
    let paths: Vec<_> = items
        .iter()
        .map(|entry| {
            format!(
                "/files/{path}/{}",
                entry.name.to_string_lossy().trim_start_matches('/')
            )
        })
        .collect();
    hypertext::rsx! {
        <h1 class="font-bold text-xl"><i class="fa-regular fa-folder pr-4"></i> {path.clone()}</h1>
        <ul hx-boost="true">
            {
                items.iter().zip(paths).map(| (entry, path) | hypertext::rsx_move! {
                    <li class="item">
                        <span class="icon w-4 h-4 pr-2">

                        {
                            if entry.is_dir {
                                hypertext::rsx! {
                                    <i class="fa-regular fa-folder"></i>
                                }
                            } else {
                                hypertext::rsx! {
                                    <i class="fa-regular fa-file"></i>
                                }
                            }
                        }
                        </span>
                        <a href={path}>{ entry.name.to_string_lossy() }</a>
                    </li>
                }).render_all()
            }
        </ul>
    }
}
