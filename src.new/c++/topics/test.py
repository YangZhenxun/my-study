import subprocess

ll = ""
k = input()
ll=k
for l in range(int(k)):
    z = input()
    ll+="\n"+z

main = subprocess.run(
    "build/windows/x64/release/topics.exe",
    capture_output=True,
    #stdin=subprocess.PIPE,
    input=bytes(ll, encoding="utf-8"),
)
x = main = subprocess.run(
    "build/windows/x64/release/x.exe",
    capture_output=True,
    #stdin=subprocess.PIPE,
    input=bytes(ll, encoding="utf-8")
)

if main.stdout == x.stdout:
    print("100%")
else:
    print(main.stdout, x.stdout)