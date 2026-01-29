# Contributing to Neuxdotdev Documentation

Thank you for your interest in contributing! 
This guide explains the steps, rules, and standards used to consistently and qualitatively improve this documentation.

---

##  Scope of Contributions

Contributions can include:

- Improvements to **grammar** and **spelling**
- Adding or updating **documentation content**
- Improving **heading structure**
- Organizing **code examples**
- Adding **new features** to the documentation
- Fixing **broken links**

---

##  How to Contribute

### 1. Fork Repository

Click the **Fork** button on this repo to your GitHub account.

### 2. Clone Your Fork

```bash
$ git clone https://github.com/<username>/neuxdotdev.github.io.git
$ cd neuxdotdev.github.io
```

### 3. Create a New Branch

Name the branch based on the contribution type:

```bash
$ git checkout -b fix/<short-description>
$ git checkout -b feature/<short-description>
```

Example:

```bash
$ git checkout -b fix/typo-homepage
```

### 4. Apply Changes

* Follow the writing standards in the documentation
* Don't change other formats without discussion
* Make sure the local preview works

### 5. Commit with a Clear Message

```bash
$ git add .
$ git commit -m "fix: short description of changes"
```

Use the following convention:
`fix:` for fixes, `feat:` for additions, `docs:` for documentation.

### 6. Push to Branch

```bash
$ git push origin <branch-name>
```

### 7. Open Pull Request

Open the PR from your fork/branch to `main` in the main repo.

---

##  Writing Standards

### Markdown Format

* Use `##` for level 2 headings
* Use `###` if subheadings are needed
* Don't use HTML unless necessary
* Use `-` or `*` for bullet points

### Language

* Use standard Indonesian
* Avoid uncommon abbreviations
* All internal links must be relative

### Code

* Use syntax highlighting according to your language
* Example:

```bash
$ npm install
```

or

```js
console.log('hello')
```

---

##  Quality Check

Before submitting a PR, do the following:

* Spell check
* Local preview (if using a generator)
* Check internal links

---

##  Discussion

When in doubt:

* Add a comment to the PR
* Tag the reviewer
* Create an issue first first

Look for tags like `help wanted` or `good first issue` if you want to start small.