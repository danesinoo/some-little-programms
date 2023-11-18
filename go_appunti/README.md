# Appunti di studio per golang

Go:

- è veloce
- staticamente tipizzato
- un linguaggio compilato
- general purpose language
- built-in testing
- object-oriented language (in a good way)

vscode plugins:
- go
- gopls
- go-outlines
- go-import

## Setup the environment

In una cartella si esegue: ``go mod init <module-name>``.
Questo genera il file ``go.mod``: go.mod descrive i _package_ che usiamo
all'interno del programma. Per creare il programma che stampa "Hello World", si
scrive:

```go
package main

import "fmt"    // to import the Print function

func main() {
    fmt.Print("Hello World")
    // fmt.Println("Hello World")
}
```

I programmi e le librerie di go sono organizzati in package.  
``import "fmt"`` include il package "fmt". Un *package* è un container di 
qualche codice.  
Per eseguire il programma si digita nel terminale ``go run main.go``, dalla
directory (PWD) che contiene il file ``main.go``.

## Variabili

Per inizializzare una variabile si scrive:

```go
var variable_name = "<some value>"

// from here on out, variable_name can be used as:
variable_name = "some string"

// or
another_initialization := "<some other value>"
```

Di seguito è inizializzata una costante:
```go
const SOME_COSTANT_VARIABLE = 50
```

Si può dichiarare una variabile senza inizializzarla. Attenzione, go non
permette di usare una variabile prima della sua inizializzazione. Per esempio:

```go
var someString string       // il tipo deve essere esplicito

// var someVariable         // this is an ERROR! Il tipo non è esplicito 
                            // e il compilatore non lo indovina

var someInteger int

// fmt.Println(someName)    // this is an ERROR!

someName = "Tom"
fmt.Println(someString, " ", someInteger)
```

## Data Types

- __Strings__: per serie di caratteri

- __Integers__ per gli interi:

| number of bits | interi | naturali |
| - | - | - |
| 8 | int8 | uint8 |
| 16 | int16 | uint16 |
| 32 | int32 | uint32 |
| 64 | int64 | uint64 |


- *Booleans*

- *Maps*

- *Arrays*

- *Slice*: corrisponde a std::<vector> in cpp





