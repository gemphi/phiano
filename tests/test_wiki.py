import urllib.request, json
url = "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&explaintext=true&titles=Rust_programming_language&format=json&redirects=1"
r = urllib.request.urlopen(url, timeout=10)
d = json.loads(r.read())
pages = d['query']['pages']
k = list(pages.keys())[0]
print(f"PageID: {k}, Title: {pages[k]['title']}")
extract = pages[k].get('extract', 'NONE')
print(f"Extract length: {len(extract)}")
print(f"Extract: {extract[:200]}")
