import subprocess 
import sys

print (sys.argv)

command = ["cargo","run"] + sys.argv[1:]

try:
    subprocess.run(command)
except:
    subprocess.run(["rm","dir.html"], stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)
    
finally:
    subprocess.run(["rm","dir.html"], stderr=subprocess.DEVNULL, stdout=subprocess.DEVNULL)