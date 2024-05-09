# Battalion

Battalion is a CLI tool for managing codebase relationships. It uses a simple heirarchy of **repositories** and **workspaces** to link codebases together when needed, and keep them separate when not.

## Installation

```bash
cargo install batl
batl setup
```

## Usage

```bash
# Create a new repository
batl repository init prototypes/awesome-project

# Create a new workspace
batl workspace init --ref prototypes/awesome-project

# Create a library
batl repository init prototypes/awesome-library

# cd into the workspace
cd $(batl workspace which prototypes/awesome-project)

# create a link
batl link init -n library prototypes/awesome-library

# Start building!
```
