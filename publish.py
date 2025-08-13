import os
import sys
import shutil
import platform

PLATFORM = platform.system()
if PLATFORM == "Darwin": PLATFORM = "macOS"
ARCH = platform.machine()


def prepare_deps():
    if PLATFORM == "Linux":
        code = os.system("sudo apt update && sudo apt install build-essential libasound2-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev libxinerama-dev libwayland-dev libxkbcommon-dev")
        if code != 0: sys.exit(1)


def pack(cmd: str, win_cmd: str, name: str):
    if PLATFORM == "Windows":
        code = os.system(win_cmd)
    else:
        code = os.system(cmd)

    if code != 0 or not os.path.exists("dist"): sys.exit(1)

    os.mkdir("tmp")
    shutil.copytree("dist", os.path.join("tmp", "UniV"))
    shutil.make_archive(
        os.path.join("publish", f"{name}-{ARCH}-{PLATFORM}"), 
        "zip", "tmp"
    )
    shutil.rmtree("tmp")


if os.path.exists("publish"):
    shutil.rmtree("publish")

os.mkdir("publish")

prepare_deps()

pack(
    "rustc dev_util.rs -o dev_util && ./dev_util --release",
    "rustc dev_util.rs -o dev_util.exe && dev_util --release",
    "NotPong"
)