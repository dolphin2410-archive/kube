import sys
import os
import shutil

# OS Type Enum
class OS(Enum):
    WINDOWS = 0
    LINUX = 1
    MACOS = 2

current_os = None

current_platform = sys.platform

# Determine current OS
if current_platform.startswith("win32"):
    current_os = OS.WINDOWS
elif current_platform.startswith("linux"):
    current_os = OS.LINUX
elif current_platform.startswith("darwin"):
    current_os = OS.MACOS
else:
    print("Unsupported OS: " + current_platform)
    exit(-1)

if shutil.which("git") is None:
    print("Git is not installed")
    exit(-1)

if shutil.which("cargo") is None:
    read = ""
    while true:
        read = input("Rust is not installed. Install? [Y/n]")
        if read == "\n" or read.lower() == "y":
            break
        elif read.lower() == "n":
            print("Rust is not installed")
            exit(-1)

    # Install Rust
    if current_os == OS.LINUX or current_os == OS.MACOS:
        os.popen("curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y")
    else:
        os.popen("curl -o installer.exe https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe")
        os.popen("installer.exe -y")
        os.remove("installer.exe")

os.popen("git clone https://github.com/dolphin2410/kube")
shutil.copy("target.txt", "kube/target.txt")
shutil.copy("app.txt", "kube/app.txt")
f = open("target.txt", "r")
zip_archive = f.read()
shutil.copy(zip_archive, "kube/" + zip_archive)

os.popen("cd kube && cargo build --release")
if current_os == OS.WINDOWS:
    shutil.copy("kube/target/releases/kube.exe", "installer.exe")
else:
    shutil.copy("kube/target/releases/kube", "installer")

shutil.rmtree("kube")
print("DONE!")

