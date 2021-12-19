"""
   Copyright 2021 Dolphin2410<dolphin2410@outlook.kr>

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
"""

import sys
import os
import shutil
from enum import Enum
import subprocess
import stat

target = sys.argv[1]
app = sys.argv[2]

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
    # Other os than Windows, Linux or Macos
    print("Unsupported OS: " + current_platform)
    exit(-1)

# Check if git is installed
if shutil.which("git") is None:
    print("Git is not installed")
    exit(-1)

# Check if cargo is installed
if shutil.which("cargo") is None:
    while True:
        read = input("Rust is not installed. Install? [Y/n]")
        if read == "\n" or read.lower() == "y":
            # Break the loop and continue the installation process
            break
        elif read.lower() == "n":
            # Exit if rust isn't installed
            print("Rust is not installed")
            exit(-1)

    # Install Rust
    if current_os == OS.LINUX or current_os == OS.MACOS:
        # For Linux, use the script provided by the official rust website
        os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
    else:
        # For Windows, Installs the executable and launches it. Afterwards, deletes it
        os.system("curl -o installer.exe https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe")
        os.system("installer.exe -y")
        os.remove("installer.exe")

# Remove the kube folder if it already exists
os.system("rm -rf kube")

# Clone the kube project from GitHub
os.system("git clone https://github.com/dolphin2410/kube")

# Write Target
f = open("kube/target.txt", "w+")
f.write(target)

# Write App
f = open("kube/app.txt", "w+")
f.write(app)

# Get the archive provided by the target.txt file
shutil.copy(target, "kube/" + target)

# Setup Linux Dependencies
if current_os == OS.LINUX:
    os.system("sudo apt-get install libgtk-3-dev -y")
    os.system("sudo apt-get install libsoup2.4 -y")
    os.system("sudo apt-get install webkit2gtk-4.0 -y")

# Build the project
os.system("cd kube && cargo build --release")

# Move the generated executable to the root directory
if current_os == OS.WINDOWS:
    shutil.copy("kube/target/release/kube.exe", "installer.exe")
else:
    shutil.copy("kube/target/release/kube", "installer")

if not os.access("kube", os.W_OK):
    os.chmod("kube", stat.S_IWUSR)

# Remove the Kube folder
shutil.rmtree("kube")

# Process End
print("DONE!")
