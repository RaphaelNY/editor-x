use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::as_24_bit_terminal_escaped;

#[derive(Props, PartialEq, Clone)]
pub struct EditorAreaProps {
    language: Signal<String>,
}

#[component]
pub fn EditorArea(props: EditorAreaProps) -> Element {
    let mut content = use_signal(String::new);
    let mut lines: Signal<Vec<String>> = use_signal(Vec::new);
    let language = props.language.clone();
    let mut highlighted_content = use_signal(String::new); // 这里使用 use_signal 进行管理

    // 加载语法集和主题
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme = Theme::default();

    // 解析 Markdown
    let parse_markdown = |markdown_content: &str| {
        let parser = Parser::new_ext(markdown_content, Options::all());
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    };

    // 高亮 Rust 代码
    let highlight_rust = |code: &str, syntax_set: &SyntaxSet, theme: &Theme| {
        let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
        let mut h = HighlightLines::new(syntax, theme);
        let highlighted = h.highlight(code, syntax_set);

        let mut html_output = String::new();
        for (style, line) in highlighted {
            let line_html = format!(
                "<span style=\"color: rgb({}, {}, {});\">{}</span>",
                style.foreground.r,
                style.foreground.g,
                style.foreground.b,
                as_24_bit_terminal_escaped(&[(style, line)], true)
            );
            html_output.push_str(&line_html);
        }
        html_output
    };

    // 更新高亮内容的函数
    let highlight_content = move |content_text: &str, lang: &str| {
        match lang {
            "rust" => highlight_rust(content_text, &syntax_set, &theme),
            "markdown" => parse_markdown(content_text),
            _ => String::new(),
        }
    };

    // 监听 `content` 更新
    use_effect(move || {
        let content_text = content();
        let lang = language();
        let highlighted = highlight_content(&content_text, &lang);
        highlighted_content.set(highlighted);
    }); // 只有 content 或 language 改变时更新高亮

    // 监听 `content` 的每行更新
    use_effect(move || {
        let content_text = content();
        let new_lines = content_text
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        lines.set(new_lines);
    }); // 只有 content 改变时更新行号

    let line_number_elements = lines()
        .iter()
        .enumerate()
        .map(|(i, _)| {
            rsx! {
                div {
                    style: "text-align: center;",
                    "{i + 1}"
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div {
            style: "flex-grow: 1; position: relative; width: 100vw; height: 100vh;", // 设置宽度为窗口宽度

            div {
                style: "position: absolute; left: 0; top: 0; width: 40px; background: #f0f0f0; padding: 10px; line-height: 1.9; font-family: monospace; overflow-y: auto;",
                { line_number_elements.into_iter() }
            }

            // 使用 textarea 进行文本输入
            textarea {
                style: "position: absolute; left: 50px; top: 0; width: calc(50vw - 50px); height: 100%; padding: 10px; font-size: 16px; line-height: 1.5; font-family: monospace; border: none; outline: none;",
                value: "{content()}", // 绑定 content 信号
                oninput: move |e| {
                    content.set(e.value().clone()); // 更新 content
                }
            }

            // 显示高亮的内容
            div {
                style: "position: absolute; left: 50vw; top: 0; width: 50vw; height: 100%; padding: 10px; font-size: 16px; line-height: 1.5; font-family: monospace; overflow-y: auto; white-space: pre-wrap; background-color: #f9f9f9;",
                dangerous_inner_html: "{highlighted_content()}" // 渲染高亮的内容
            }
        }
    }
}
