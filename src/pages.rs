pub const HTML_SHELL: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>__TITLE__</title>
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <style>
      :root {
        color-scheme: dark;
        --bg: #050505;
        --card: #0f0f10;
        --border: #222;
        --text: #f5f5f5;
        --muted: #9ca3af;
        --accent: #e5e5e5;
      }
      * { box-sizing: border-box; margin: 0; padding: 0; }
      body {
        min-height: 100vh;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, "SF Pro Text", sans-serif;
        background: radial-gradient(circle at top, #111 0, #050505 55%);
        color: var(--text);
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 2rem 1.5rem;
      }
      .card {
        width: 100%;
        max-width: 720px;
        border-radius: 1.25rem;
        border: 1px solid var(--border);
        background: radial-gradient(circle at top left, #151515 0, var(--card) 50%, #050505 100%);
        padding: 1.75rem 1.75rem 1.5rem;
      }
      .eyebrow {
        font-size: 0.7rem;
        letter-spacing: 0.22em;
        text-transform: uppercase;
        color: var(--muted);
        margin-bottom: 0.75rem;
      }
      h1 { font-size: 1.6rem; line-height: 1.2; margin-bottom: 0.75rem; }
      p { font-size: 0.9rem; line-height: 1.6; color: var(--muted); }
      .grid { display: grid; grid-template-columns: 1.1fr 1fr; gap: 1.5rem; }
      @media (max-width: 640px) { .grid { grid-template-columns: 1fr; } }
      .pill-row { display: flex; flex-wrap: wrap; gap: 0.4rem; margin: 1.25rem 0 0.75rem; }
      .pill {
        font-size: 0.7rem;
        text-transform: uppercase;
        letter-spacing: 0.16em;
        padding: 0.25rem 0.6rem;
        border-radius: 999px;
        border: 1px solid var(--border);
        color: var(--muted);
      }
      .pill strong { color: var(--accent); font-weight: 600; }
      .links { display: flex; flex-direction: column; gap: 0.35rem; margin-top: 0.5rem; font-size: 0.8rem; }
      .links a { color: var(--accent); text-decoration: none; display: inline-flex; align-items: center; gap: 0.3rem; }
      .links a span { font-size: 0.75rem; color: var(--muted); }
      .links a:hover { text-decoration: underline; }
      .meta {
        margin-top: 1.5rem;
        padding-top: 0.75rem;
        border-top: 1px solid var(--border);
        display: flex;
        justify-content: space-between;
        gap: 0.75rem;
        font-size: 0.75rem;
        color: var(--muted);
      }
      .badge {
        padding: 0.1rem 0.55rem;
        border-radius: 999px;
        border: 1px solid var(--border);
        font-size: 0.7rem;
        text-transform: uppercase;
        letter-spacing: 0.14em;
      }
      .back { font-size: 0.8rem; margin-bottom: 1rem; }
      .back a { color: var(--muted); text-decoration: none; }
      .back a:hover { color: var(--accent); text-decoration: underline; }
      ul { padding-left: 1rem; margin-top: 0.6rem; font-size: 0.85rem; color: var(--muted); }
      code {
        background: #0b0b0c;
        border: 1px solid var(--border);
        padding: 0.1rem 0.35rem;
        border-radius: 0.35rem;
        color: var(--accent);
      }
    </style>
  </head>
  <body>
    <div class="card">__BODY__</div>
  </body>
</html>"#;

pub fn html_shell(title: &str, body: &str) -> String {
    HTML_SHELL
        .replace("__TITLE__", title)
        .replace("__BODY__", body)
}
