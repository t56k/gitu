# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2024-03-07

### 🚀 Features

- Move 'reset' keybind to capital 'X' to mimic magit
- Proper y/n prompt when discarding things

### 🐛 Bug Fixes

- Annotated tags would not display

## [0.4.0] - 2024-03-06

### 🚀 Features

- Add `style.line_highlight.[un]changed` config options

### 🐛 Bug Fixes

- Terminal would corrupt text when quitting gitu after opening editor
- Terminal would corrupt text when gitu crashed

## [0.3.0] - 2024-03-05

### 🚀 Features

- Read not just EDITOR env var, but GIT_EDITOR & VISUAL too
- Add error popup and more graceful error handling
- Improve CHANGELOG.md format
- Replace --exit-immediately cli flag with new --print

### 🐛 Bug Fixes

- Show author date (not commit date) on commits like 'git log'

### 🎨 Styling

- Selection_line & selection_area now extend fully to left

<!-- generated by git-cliff -->