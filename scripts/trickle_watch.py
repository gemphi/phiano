"""Keeps the phi4 trickle (gen_prose.py) running: restarts it whenever it
exits without finishing, a minute later. The .done file makes every restart
resume where the last one stopped.

    python trickle_watch.py --model phi4 --vocab emma --n 16 --pace 30 --threads 4 --out <dir>

Every argument goes to gen_prose.py unchanged; the log goes to <out>/trickle.log.
"""
import os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
args = sys.argv[1:]
out = args[args.index("--out") + 1] if "--out" in args else "."
log, err = os.path.join(out, "trickle.log"), os.path.join(out, "trickle.err")

while True:
    with open(log, "a", encoding="utf-8") as lo, open(err, "a", encoding="utf-8") as er:
        lo.write(f"watch {time.strftime('%Y-%m-%d %H:%M')}: starting gen_prose.py\n"); lo.flush()
        code = subprocess.call([sys.executable, "-u", os.path.join(HERE, "gen_prose.py")] + args, stdout=lo, stderr=er)
    with open(log, encoding="utf-8") as lo:
        finished = any(line.startswith("done: ") for line in lo.readlines()[-3:])
    if finished:
        break
    with open(log, "a", encoding="utf-8") as lo:
        lo.write(f"watch {time.strftime('%H:%M')}: gen_prose.py exited ({code}) unfinished; restarting in a minute\n")
    time.sleep(60)
