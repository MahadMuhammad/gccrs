---
title: "Commands"
description: "Doks comes with commands for common tasks."
lead: "Doks comes with commands for common tasks."
date: 2020-10-13T15:21:01+02:00
lastmod: 2020-10-13T15:21:01+02:00
draft: false
images: []
menu:
  docs:
    parent: "prologue"
weight: 130
toc: true
---

{{< alert icon="💡" text="You can change the commands in the scripts section of `./package.json`." />}}

## create

Create new content for your site:
## I am Mahad


```javascript
#include <iostream>
using namespace std;

int main() {
    cout << "Hello, World!";
    return 0;
}
```

```rust
fn main() {
    println!("Hello, World!");
}
```

```python
print("Hello, World!")
```

```

{{< highlight html >}}
<section id="main">
  <div>
    <h1 id="title">{{ .Title }}</h1>
    {{ range .Pages }}
      {{ .Render "summary"}}
    {{ end }}
  </div>
</section>
{{< /highlight >}}

```html {path="layouts/partials/head.html"}
<head>
    <meta name="author" content="{{ with .Site.Params.author }}{{ . }}{{ end }}" />
    <meta name="description" content="{{if .IsHome }}{{ $.Site.Params.description }}{{ else }}{{ .Description }}{{ end }}" />
    <meta name="theme-color" content="#111111" />
</head>
```

{{< highlight javascript >}}
function addOne(number) {
    return number + 1;
}
{{< /highlight >}}

```bash
npm run create [path] [flags]
```

See also the Hugo docs: [hugo new](https://gohugo.io/commands/hugo_new/).

### Docs based tree

Create a docs based tree — with a single command:

```bash
npm run create -- --kind docs [section]
```

For example, create a docs based tree named guides:

```bash
npm run create -- --kind docs guides
```

## lint

Check scripts, styles, and markdown for errors:

```bash
npm run lint
```

### scripts

Check scripts for errors:

```bash
npm run lint:scripts [-- --fix]
```

### styles

Check styles for errors:

```bash
npm run lint:styles [-- --fix]
```

### markdown

Check markdown for errors:

```bash
npm run lint:markdown [-- --fix]
```

## clean

Delete temporary directories:

```bash
npm run clean
```

## start

Start local development server:

```bash
npm run start
```

## build

Build production website:

```bash
npm run build
```

### functions

Build Lambda functions:

```bash
npm run build:functions
```

### preview

Build production website including draft and future content:

```bash
npm run build:preview
```
