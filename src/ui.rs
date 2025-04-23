use hypertext::{html_elements, GlobalAttributes, Renderable};

use crate::packages::{FONTAWESOME_PACKAGE, HTMX_PACKAGE, MONACO_EDITOR_PACKAGE, MONACO_LOADER_PACKAGE, MONACO_STYLESHEET, TAILWIND_PACKAGE};

struct MonacoRequire(&'static str);
impl Renderable for MonacoRequire {
	fn render_to(self, output: &mut String) {
		output.push_str(&format!(
			"<script>\n\tvar require = {{ paths: {{ vs: \"{}\" }} }};\n</script>",
			self.0
		));
	}
}

pub(crate) fn page<'a>(content: impl Renderable + 'a) -> impl Renderable + 'a {
	hypertext::rsx! {
		<html>
			<head>
				<title>File Explorer</title>
				
				// <script>
				// 	var require = { paths: { vs: "../node_modules/monaco-editor/min/vs" } };
				// </script>
				{
					MonacoRequire("https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.26.1/min/vs")
				}
				<script src=MONACO_LOADER_PACKAGE ></script>
				<script src=MONACO_EDITOR_PACKAGE ></script>
				<script src=HTMX_PACKAGE ></script>
				<script src=TAILWIND_PACKAGE ></script>

				<link rel="stylesheet" href=FONTAWESOME_PACKAGE />
				<link rel="stylesheet" href=MONACO_STYLESHEET />
			</head>
			<body>
				<aside id="sidebar" class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform sm:translate-x-0">
					<div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
						<ul class="space-y-2 font-medium">
							<li>
								<a href="/files/" class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">
									<i class="fa-regular fa-file"></i>
									<span class="ms-3">Files</span>
								</a>
							</li>
							<li>
								<a href="/settings/" class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">
									<i class="fa-solid fa-gear"></i>
									<span class="flex-1 ms-3 whitespace-nowrap">Settings</span>
								</a>
							</li>
						</ul>
					</div>
				</aside>
				<div id="content" class="p-4 sm:ml-64">
					{ content }
				</div>
			</body>
		</html>
	}
}
