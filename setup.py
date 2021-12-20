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
import subprocess

target = sys.argv[1].replace("/", "\\")
app = sys.argv[2]
auto_install = sys.argv[3] == "-y"

current_os = sys.platform

# Check if git is installed
try:
    subprocess.run(["git", "--help"], stdout=subprocess.DEVNULL)
except OSError as e:
    print("Git is not installed")
    exit(-1)

# Check if cargo is installed
try:
    subprocess.run(["cargo", "-h"], stdout=subprocess.DEVNULL)
except OSError as e:
    if not auto_install:
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
    if not current_os == "win32":
        # For Unix, use the script provided by the official rust website
        os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
    else:
        # For Windows, Installs the executable and launches it. Afterwards, deletes it
        os.system("curl -o installer.exe https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe")
        os.system("installer.exe -y")
        os.remove("installer.exe")

# Remove the kube folder if it already exists
if current_os == "win32":
    os.system("rmdir kube /S /Q")
else:
    subprocess.run(["rm", "-rf", "kube"], stdout=subprocess.DEVNULL)

# Clone the kube project from GitHub
os.system("git clone https://github.com/dolphin2410/kube")

# Write Target
f1 = open("kube/target.txt", "w+")
f1.write(target)
f1.close()

# Write App
f2 = open("kube/app.txt", "w+")
f2.write(app)
f2.close()

# Get the archive provided by the target.txt file
if current_os == "win32":
    os.system("copy " + target + " kube\\" + target)
else:
    subprocess.run(["cp", target, "kube/" + target], stdout=subprocess.DEVNULL)

# Setup Linux Dependencies
if not current_os == "win32":
    os.system("sudo apt-get install libgtk-3-dev -y")
    os.system("sudo apt-get install libsoup2.4 -y")
    os.system("sudo apt-get install webkit2gtk-4.0 -y")

# Build the project
os.system("cd kube && cargo build --release")

# Move the generated executable to the root directory
if current_os == "win32":
    os.system("copy kube\\target\\release\\kube.exe installer.exe /Y")
else:
    subprocess.run(["cp", "kube/target/release/kube", "installer"], stdout=subprocess.DEVNULL)
# Delete kube
if current_os == "win32":
    os.system("rmdir /S /Q kube")
else:
    subprocess.run(["rm", "-rf", "kube"], stdout=subprocess.DEVNULL)

# Process End
print("Complete!")
