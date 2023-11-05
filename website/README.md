# Website

This program is meant to convert .md files into .html.  
It is strongly inspired by [Jekyll](https://jekyllrb.com/), I mostly copied it,
of course it is not as complete as Jekyll.  
This project has got the Rust documentation, so you can clone the repo and use
the command ```cargo doc --open``` to get a general idea of the program.

## General Idea

This program converts a directory of markdown files to html files. The structure 
of the directory needs to be the following:

```markdown
target
├── folder1
│   ├── file1.md
│   ├── file2.md
│   └── ...
├── layout
│   ├── some_layout.md
│   └── ...
├── assets
│   ├── some_asset.md
│   └── ...
└── ...
```
