import os
import sys
import time
import urllib.request

url = "https://huggingface.co/unsloth/phi-4-GGUF/resolve/main/phi-4-Q4_K_M.gguf"
dest = r"c:\Users\phiac\Workspace\gemphi\phiano\models\phi-4-Q4_K_M.gguf"
dest_tmp = dest + ".tmp"

os.makedirs(os.path.dirname(dest), exist_ok=True)

if os.path.exists(dest):
    print(f"File already exists: {dest}")
    sys.exit(0)

initial_size = os.path.getsize(dest_tmp) if os.path.exists(dest_tmp) else 0

req = urllib.request.Request(url)
if initial_size > 0:
    req.add_header('Range', f'bytes={initial_size}-')

print(f"Downloading Phi-4 Q4_K_M (8.28 GB) to {dest}...")
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
                if now - last_report >= 3.0:
                    elapsed = now - start_time
                    speed_mb = ((downloaded - initial_size) / (1024 * 1024)) / max(elapsed, 0.001)
                    if total_size > 0:
                        percent = (downloaded / total_size) * 100
                        print(f"Progress: {downloaded/(1024**3):.2f} / {total_size/(1024**3):.2f} GB ({percent:.1f}%) - Speed: {speed_mb:.2f} MB/s", flush=True)
                    else:
                        print(f"Downloaded: {downloaded/(1024**3):.2f} GB - Speed: {speed_mb:.2f} MB/s", flush=True)
                    last_report = now

    os.rename(dest_tmp, dest)
    print(f"Successfully completed download! Model saved to {dest}")
except Exception as e:
    print(f"Error during download: {e}")
    sys.exit(1)
