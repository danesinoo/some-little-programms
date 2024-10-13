#set text(
	font: "Galvji",
	size: 9pt,
)

#set heading(
	numbering: "1.1",
	bookmarked: true,
)

#set page(
	paper: "a4",
	numbering: (page, total) => box[
		#page / #total
	],
	margin: (
		top: 3cm,
		bottom: 4cm,
		left: 3.5cm,
		right: 3.5cm
	),
	fill: rgb(251,248,240),
	header: align(right)[
		#context(document.title)
	],
)

#set par(
	leading: 1em,
	first-line-indent: 0.5em,
	justify: true,
)

#set document(
	title: "Hello world",
	author: "Carlo Rosso",
	date: datetime.today(),
)

#show link: set text(blue)
#show ref: set text(blue)


= Hello world <hello-world>

== Introduction <introduction>

=== Subsection

==== Subsection

=== Subsection

```ts

console.log("Hello world");

```

@hello-world
@introduction

Wow

#lorem(1000)

#link("http://www.google.com", "Google")
