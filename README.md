# Kube
***Binary Application Installer***

## Notes
Currently under development

## Introduction
Kube is made to install a java application image made by `jlink`. The front-end is made up with [web-view](https://github.com/Boscop/web-view), and the rest with pure rust.

## Usage
You can easily create the installer with `setup.py`. 

### Requirements
- The `python` interpreter
- Windows, Mac or any Linux distribution

### Steps
1. Download the `setup.py` located in the root of this project.
> You can do this by running the following command

```bash
curl -o setup.py https://raw.githubusercontent.com/dolphin2410/kube/main/setup.py
```

2. Create `target.txt`
> This should contain the name of the archive. Here's an example

```bash
path/to/archive.zip
```

3. Execute the `setup.py`
> You can do this by running the following command
    
```bash
python setup.py
```