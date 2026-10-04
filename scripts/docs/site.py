"""Validate the exact generated file/link closure and write its bounded manifest."""
import re
import json
import sys
from pathlib import Path
from urllib.parse import urljoin, urlsplit, unquote
from bs4 import BeautifulSoup

ROOT = Path(sys.argv[1] if len(sys.argv)>1 else 'docs/build')
EXTENSIONS = {'.html', '.js', '.css', '.json', '.svg', '.png', '.ico', '.woff2', '.woff', '.ttf', '.xml', '.gz'}
files = sorted(p.relative_to(ROOT).as_posix() for p in ROOT.rglob('*') if p.is_file() and p.name != 'asset-manifest.json')
if ROOT.is_symlink(): raise ValueError('Documentation root symlink')
if not files or len(files)>512:
    raise ValueError('Documentation file count exceeds 512')
for entry in ROOT.rglob('*'):
    if entry.is_symlink(): raise ValueError('Documentation symlink: '+str(entry))
total=0
ids={}
for name in files:
    p=ROOT/name
    if p.is_symlink() or p.suffix not in EXTENSIONS or len(p.relative_to(ROOT).parts)>8 or any(not re.fullmatch(r'[a-zA-Z0-9_.-]+', part) or part in ('..','.') for part in p.relative_to(ROOT).parts):
        raise ValueError('Unsafe documentation asset: '+name)
    size=p.stat().st_size
    if size>8*1024*1024: raise ValueError('Oversized documentation asset: '+name)
    total+=size
    if p.suffix=='.html':
        soup=BeautifulSoup(p.read_text(),'html.parser')
        ids[name]={str(e['id']) for e in soup.find_all(id=True)}
        for script in soup.find_all('script'):
            if not script.get('src') and script.get('type') not in ('application/json','application/ld+json') and script.get_text().strip():
                raise ValueError('Executable inline script: '+name)
if total>32*1024*1024: raise ValueError('Documentation bundle exceeds 32 MiB')
for name in files:
    if not name.endswith('.css'): continue
    css=(ROOT/name).read_text()
    for match in re.finditer(r"url\(\s*['\"]?([^'\")\s]+)", css):
        url=match[1]
        if url.startswith('data:'): continue
        parsed=urlsplit(url)
        if parsed.scheme or parsed.netloc: raise ValueError('External CSS dependency: '+url)
        path=unquote(urlsplit(urljoin('/'+name,url)).path).lstrip('/')
        if path not in files: raise ValueError(f'{name}: missing CSS target {url}')
    if re.search(r"@import\s+['\"]", css): raise ValueError('CSS imports must use validated url(): '+name)
for name in ids:
    soup=BeautifulSoup((ROOT/name).read_text(),'html.parser')
    for element in soup.find_all(['a','script','img','link']):
        url=element.get('href') or element.get('src')
        if not url: continue
        parsed=urlsplit(url)
        if parsed.scheme or parsed.netloc:
            if element.name!='a': raise ValueError('External runtime dependency: '+url)
            if parsed.scheme not in ('http','https','mailto'): raise ValueError('Unsafe external link: '+url)
            continue
        path=unquote(urlsplit(urljoin('/'+name,url)).path).lstrip('/')
        if path.endswith('/') or not path: path+='index.html'
        if path not in files: raise ValueError(f'{name}: missing target {url} -> {path}')
        if parsed.fragment and path in ids and unquote(parsed.fragment) not in ids[path]:
            raise ValueError(f'{name}: missing anchor {url}')
(ROOT/'asset-manifest.json').write_text(json.dumps({'version':1,'files':files},indent=2)+'\n')
print(f'Documentation closure: {len(files)} files, {total} bytes; links and anchors valid')
