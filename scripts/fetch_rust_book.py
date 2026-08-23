import os
import re
import sys
import time
import urllib.request
import json

out_dir = r"c:\Users\phiac\Workspace\gemphi\phiano\data\rust_book"
corpus_file = r"c:\Users\phiac\Workspace\gemphi\phiano\data\rust_book_corpus.txt"

os.makedirs(out_dir, exist_ok=True)

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'
}

print("1. Fetching Rust Book chapter list from GitHub...")
req = urllib.request.Request('https://api.github.com/repos/rust-lang/book/contents/src', headers=headers)

try:
    with urllib.request.urlopen(req) as resp:
        files = json.loads(resp.read().decode('utf-8'))
except Exception as e:
    print(f"Error fetching directory index: {e}")
    sys.exit(1)

md_files = [f for f in files if f['name'].endswith('.md') and not f['name'].startswith('appendix')]
print(f"Found {len(md_files)} chapter files to download.")

all_lines = []

for idx, file_info in enumerate(md_files, 1):
    fname = file_info['name']
    download_url = file_info['download_url']
    local_path = os.path.join(out_dir, fname)
    
    if not os.path.exists(local_path):
        print(f"[{idx}/{len(md_files)}] Downloading {fname}...")
        file_req = urllib.request.Request(download_url, headers=headers)
        try:
            with urllib.request.urlopen(file_req) as f_resp:
                content = f_resp.read().decode('utf-8', errors='ignore')
                with open(local_path, 'w', encoding='utf-8') as f:
                    f.write(content)
        except Exception as e:
            print(f"Error downloading {fname}: {e}")
            continue
        time.sleep(0.1)
    else:
        with open(local_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
    # Process content into clean training text lines
    lines = content.splitlines()
    for line in lines:
        line = line.strip()
        if not line or line.startswith('#') or line.startswith('```') or line.startswith('<!--'):
            continue
        # Strip markdown links and inline formatting
        line = re.sub(r'\[([^\]]+)\]\([^\)]+\)', r'\1', line)
        line = re.sub(r'[`*_]', '', line)
        if len(line.split()) >= 3:
            all_lines.append(line)

print(f"\nExtracted {len(all_lines)} clean training sentences from the Rust Book!")
with open(corpus_file, 'w', encoding='utf-8') as f:
    f.write('\n'.join(all_lines))

print(f"Saved compiled Rust Book training corpus to {corpus_file}")
