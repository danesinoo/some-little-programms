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

- __Integers__: per gli interi:

| number of bits    | interi    | naturali  |
| ---               | ---       | ---       |
| hardware specific | int       | uint      |
| 8                 | int8      | uint8     |
| 16                | int16     | uint16    |
| 32                | int32     | uint32    |
| 64                | int64     | uint64    |

- __Floating Point__: numeri con la virgola:

| number of bits    | float     |
| ---               | ---       |
| hardware specific | int       |
| 8                 | int8      |
| 16                | int16     |
| 32                | int32     |
| 64                | int64     |

- *Booleans*

- *Maps*

- *Array*

```go
// base structure to initialize an array
// var array_example = [<size>]<data_type>{element_1, ..., element_i} // i < size

some_name := [5]string{"Carlo", "Giovanni"}

some_name[2] = "Giacomo"
```

- *Slice*: corrisponde a ``std::<vector>`` in cpp, ovvero la sua dimensione è
  dinamica (l'array ce l'ha statica (nella stack))

```go
// base structure to initializa a slice
// var slice_example = []<data_tipe>{element1, ..., element_i}

some_name := []string{"Carlo", "Giovanni"}
some_name.append("Giacomo")
```


### Type conversion



## Take some input

```go
var some_input
fmt.Scan(&some_input)       // non possiamo passare la variabile per valore
```

Per passare una variabile ad una funzione e permettere alla funzione di
modificare il valore che stiamo passando abbiamo bisogno di passare un puntatore
alla variabile. Per questo motivo usiamo l'operatore di dereferenziazione ``&``.
In questo modo permettiamo alla funzione di avere _side-effect_ sulla variabile
che passiamo.

## Loops

```go
for {
    // this iterates forever

    i := 0
    i += 10

    // break    // used to exit a loop

    continue

    // this code is unreachable
    i += 20
}
```

Go non ha la _keyword_ ``while``, i loop esistono solo mediante la _keyword_
``for``. Per iterare nelle liste:

```go
for index, element := range the_slice {
    fmt.Println("the_slice[", index, "]: ", element)

    // equivalent
    // fmt.Printf("the_slice[%v]: %v", index, the_slice[index]) // %v stands for "value"
}
```
<!-- ``string``: package per lavorare con le string -->

Per ignorare una variabile si utilizza il simobolo ``_``, per esempio se non
vogliamo l'indice nel loop precedente il codice diventa:

```go
for _, element := range the_slice {
    fmt.Println("the_slice[", index, "]: ", element)

    // equivalent
    // fmt.Printf("the_slice[%v]: %v", index, the_slice[index]) // %v stands for "value"
}
```

Il simbolo ``_`` avvisa il compilatore che non abbiamo intenzione di usare la
suddetta variabile.

## Condition

```go
if condition { 
    /* block */ 
} else if another_condition { 
    /* another block */ 
} else { 
    /* last block */ 
}
```

## Functions

```go
package main

import "fmt"

func main() {
    some_int := 10
    fun(some_arg)
}

// template:
// func function_name(variable_name_1 type_1, ..., variable_name_i type_i)
// (returning_type_1, ..., returning_type_n) {
// /* block of code */
// }

func fun(var_name int) {
    fmt.Println(var_name)
}

// if type_i == type_j for all j in [1, i]
// even this works:
// func function_name(variable_name_1, ..., variable_name_i type)
// you can group up (factorize) the variable type
