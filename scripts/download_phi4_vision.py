import os
import sys
import time
import urllib.request

files_to_download = [
    {
        "url": "https://huggingface.co/ShayanCyan/phi4-multimodal-quantisized-gguf/resolve/main/phi4-mm-vision-q8.gguf",
        "dest": r"c:\Users\phiac\Workspace\gemphi\phiano\models\phi4-mm-vision-q8.gguf",
        "desc": "Phi-4 SigLIP Vision Encoder (572 MB)"
    },
    {
        "url": "https://huggingface.co/ShayanCyan/phi4-multimodal-quantisized-gguf/resolve/main/phi4-mm-Q4_K_M.gguf",
        "dest": r"c:\Users\phiac\Workspace\gemphi\phiano\models\phi4-mm-Q4_K_M.gguf",
        "desc": "Phi-4 Multimodal Language Model (2.32 GB)"
    }
]

for item in files_to_download:
    url = item["url"]
    dest = item["dest"]
    desc = item["desc"]
    dest_tmp = dest + ".tmp"

    os.makedirs(os.path.dirname(dest), exist_ok=True)

    if os.path.exists(dest):
        print(f"Skipping {desc}: File already exists at {dest}")
        continue

    initial_size = os.path.getsize(dest_tmp) if os.path.exists(dest_tmp) else 0

    req = urllib.request.Request(url)
    if initial_size > 0:
        req.add_header('Range', f'bytes={initial_size}-')

    print(f"\n---> Starting download of {desc}...")
    start_time = time.time()

    try:
        with urllib.request.urlopen(req) as response:
            content_len = response.headers.get('Content-Length')
            total_size = (int(content_len) + initial_size) if content_len else 0
            mode = 'ab' if initial_size > 0 else 'wb'
            downloaded = initial_size
            
            with open(dest_tmp, mode) as f:
                block_size = 2 * 1024 * 1024  # 2MB chunks
                last_report = time.time()
                
                while True:
                    buffer = response.read(block_size)
                    if not buffer:
                        break
                    downloaded += len(buffer)
                    f.write(buffer)
                    
                    now = time.time()
                    if now - last_report >= 2.5:
                        elapsed = now - start_time
                        speed_mb = ((downloaded - initial_size) / (1024 * 1024)) / max(elapsed, 0.001)
                        if total_size > 0:
                            percent = (downloaded / total_size) * 100
                            print(f"Progress [{desc}]: {downloaded/(1024**3):.2f} / {total_size/(1024**3):.2f} GB ({percent:.1f}%) - Speed: {speed_mb:.2f} MB/s", flush=True)
                        else:
                            print(f"Downloaded: {downloaded/(1024**3):.2f} GB - Speed: {speed_mb:.2f} MB/s", flush=True)
                        last_report = now

        os.rename(dest_tmp, dest)
        print(f"Successfully downloaded {desc} to {dest}")
    except Exception as e:
        print(f"Error downloading {desc}: {e}")
        sys.exit(1)

print("\n=== All Phi-4 Multimodal Vision files successfully downloaded! ===")
