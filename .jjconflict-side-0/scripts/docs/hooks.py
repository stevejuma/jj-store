"""Portable Mermaid images and CSP-compatible Material output."""
import hashlib
import re
from pathlib import Path
from bs4 import BeautifulSoup
from mkdocs.utils import get_relative_url


def on_page_markdown(markdown, *, page, config, files):
    # HTML image paths are output-relative; Markdown links are source-relative.
    def image(match):
        digest = hashlib.sha256(match[1].encode()).hexdigest()[:20]
        path = get_relative_url(f"assets/generated/{digest}.svg", page.url)
        dark_path = get_relative_url(f"assets/generated/{digest}.dark.svg", page.url)
        return f'<figure class="architecture-diagram"><img src="{path}" data-light-src="{path}" data-dark-src="{dark_path}" alt="Architecture diagram; explanation follows."></figure>'
    return re.sub(r"```mermaid\n([\s\S]*?)\n```", image, markdown)


def on_post_page(output, *, page, config):
    soup = BeautifulSoup(output, "html.parser")
    for search in soup.select('[data-md-component="search"][role="dialog"]'):
        search['aria-label'] = 'Search documentation'
    directory = Path(config["site_dir"]) / "assets" / "bootstrap"
    directory.mkdir(parents=True, exist_ok=True)
    for script in soup.find_all("script"):
        if script.get("src") or script.get("type") in ("application/json", "application/ld+json"):
            continue
        source = script.string or script.get_text()
        if not source.strip():
            continue
        name = hashlib.sha256(source.encode()).hexdigest()[:20] + ".js"
        (directory / name).write_text(source)
        script.clear()
        script["src"] = get_relative_url("assets/bootstrap/" + name, page.url)
    for link in soup.find_all("a", href=True):
        if link["href"].startswith(("https://", "http://")):
            link["target"] = "_blank"
            link["rel"] = "noopener noreferrer"
    return str(soup)


def on_post_build(*, config):
    # MkDocs' error template does not run the page hooks.
    from types import SimpleNamespace
    path = Path(config['site_dir']) / '404.html'
    if path.exists():
        path.write_text(on_post_page(path.read_text(), page=SimpleNamespace(url='404.html'), config=config))
    # Debug source maps are not part of the reader's runtime closure.
    for mapping in Path(config['site_dir']).rglob('*.map'):
        mapping.unlink()
