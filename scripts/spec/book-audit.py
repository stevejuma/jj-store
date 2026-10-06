"""Check local files and anchors in the generated specification book."""

from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit
import sys


class Page(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = set()
        self.links = []

    def handle_starttag(self, tag, attributes):
        attributes = dict(attributes)
        if "id" in attributes:
            self.ids.add(attributes["id"])
        if tag == "a" and "href" in attributes:
            self.links.append(attributes["href"])
        if tag in {"img", "script"} and "src" in attributes:
            self.links.append(attributes["src"])
        if tag == "link" and "href" in attributes:
            self.links.append(attributes["href"])


def audit(directory):
    root = Path(directory).resolve()
    pages = {}
    for path in root.rglob("*.html"):
        page = Page()
        page.feed(path.read_text())
        pages[path] = page
    if not pages:
        raise ValueError("generated book has no HTML pages")
    errors = set()
    links = 0
    for path, page in pages.items():
        if path.name == "print.html":
            continue
        for href in page.links:
            url = urlsplit(href)
            if url.scheme or url.netloc:
                continue
            links += 1
            target = (path.parent / unquote(url.path)).resolve() if url.path else path
            problem = None
            if not target.is_relative_to(root):
                problem = "outside the generated book"
            elif not target.exists():
                problem = "missing local file"
            elif url.fragment and target in pages and unquote(url.fragment) not in pages[target].ids:
                problem = "missing anchor"
            if problem:
                errors.add(f"{path.relative_to(root)}: {href}: {problem}")
    if errors:
        raise ValueError("\n".join(sorted(errors)))
    print(f"Book links and anchors valid ({len(pages)} HTML files; {links} local references)")


if __name__ == "__main__":
    try:
        audit(sys.argv[1] if len(sys.argv) > 1 else "spec/book")
    except ValueError as error:
        print(error, file=sys.stderr)
        sys.exit(1)
