import subprocess
import sys
import os

dir_path = os.path.dirname(os.path.realpath(__file__))
cargo_toml = dir_path + "/Cargo.toml"
command = ["cargo", "run", "--release",
           "--manifest-path", cargo_toml] + sys.argv[1:]

try:
    subprocess.run(command)
except:
    subprocess.run(["rm", "dir_toy_server.html"],
                   stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)

finally:
    subprocess.run(["rm", "dir_toy_server.html"],
                   stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)
