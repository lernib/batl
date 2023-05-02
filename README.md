# Battalion

Battalion is a CLI tool for managing codebase relationships. It uses a simple heirarchy of **repositories** and **workspaces** to link codebases together when needed, and keep them separate when not.

## Installation

```bash
cargo install --git https://github.com/carlosskii/batl
batl setup
```

## Usage

```bash
# Create a new repository
batl repository init prototypes/awesome-project

# Create a new workspace
batl workspace init awesome-project

# (while in the workspace directory)
batl link init --name main prototypes/awesome-project
```