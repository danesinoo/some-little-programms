use std::fs;
use std::collections::HashSet;

fn main() {
    // Open the file
    let file = fs::read_to_string("/Users/carlorosso/Documents/some-little-programms/clash.txt")
    .expect("Unable to read file");

    // inizializzo un vettore che poi conterra le posizioni delle righe
    let mut numero_righe: Vec<usize> = vec![0usize];
    let mut puntatore: usize = 0;

    // in questo modo ogni volta che viene trovato il carattere '\n' la posizione del carattere è salvata
    // in realà esiste un metodo più veloce per effettuare questo passagio, solo che volevo generare un array
    // non un vettore. Ho scoperto che le dimensioni del vettore devono essere ben definite prima della compilazione
    for char in file.chars() {
        puntatore += 1;
        if char == '\n' { numero_righe.push(puntatore) }
    }

    // questo è il vettore in cui sono salvate le singole righe. info_parita indica proprio la singola riga:
    // ogni riga corrisponde ad una partita
    let mut info_partita : Vec<&str> = Vec::new();


    // viene diviso l'intero testo importato in righe e queste sono inserite in info_partita
    for i in 0..numero_righe.len() - 1 {
        info_partita.push( &file[ numero_righe[i]..numero_righe[i+1] - 1 ] );
    }

    // questo vettore contiene tutte le partite. Non sono più dati posti a caso, quando entrano in questo
    // set le informazioni sono della classe richiesta ed è possibile estrarle
    let mut partite : Vec<Partita> = Vec::new();

    // grazie a questa classe è possibile ordinare per bene le partite
    struct Partita {
        mazzo: HashSet<String>,         // elenco delle carte, non è importante l'ordine
        esito: char,                    // V: vittoria; S: sconfitta
        risultato: (usize, usize)       // numero partite vinte, numero partite perse

// non uso il numero delle partite vinte o quello delle partite perse in questo programma, tuttavia
// una sua implementazione risulterebbe piuttosto semplice

    }

 // Debug del testo inserito
/*    for i in &info_partita {
        let v: Vec<&str> = i.split(" ").collect();
        println!("{:?}", v.len());
    }
*/
// in questo caso scontro è una stringa grezza con le informazioni sulla partita.
    for scontro in info_partita {
        let parole: Vec<&str> = scontro.split(" ").collect(); // si genera un vettore che contiene le parole singole
        let battaglia = Partita {
            mazzo: parole[..8].iter().map(|&s|s.into()).collect(), // le prime 8 parole sono inserite nel mazzo
            esito: parole[8].chars().next().unwrap(), // la non indica una vittoria o una sconfitta
            risultato: (parole[9].parse::<usize>().unwrap(), // la decima indica il numero di torri abbattute
            parole[10].parse::<usize>().unwrap()) // l'undicesima indica il numero di torri che mi hanno distrutto
        };
        partite.push(battaglia);
    }

    // qui inizializzo 4 set in coppie a due a due.
    // Il primo è un array che contiene tutte le possibili carte dei mazzi che ho battutto.
    // Idem per il terzo, but contiene i mazzi che mi hanno battuto.
    // Il secondo array contiene una coppia numero stringa: il numero indica il numero di volte che
    // c'era la carta rappresentata dalla stringa nei mazzi che ho battutto.
    // Idem per il quarto, but dei mazzi che mi hanno battuto.
    let generica_stringa = &String::from("hello");
    let mut carteh : [&String; 60] = [ generica_stringa ; 60 ];
    let mut frequenza_carteh: [(usize, &String); 60] = [ ( 0usize, generica_stringa ) ; 60 ];


    let mut cartes : [ &String; 60] = [ generica_stringa ; 60 ];
    let mut frequenza_cartes: [(usize, &String); 60] = [ ( 0usize, generica_stringa ) ; 60 ];

    // qui mano a mano si riempiono i vari array
    for partita in &partite {
        match &partita.esito {
            &'V' => {    for carta in &partita.mazzo { // se ho vinto la partita...
                        //    print!(" {} ", carta);
                            if carteh.contains(&carta) {
                                frequenza_carteh[ match carteh.iter().position(|r| r == &carta) {
                                    Some(value) => value,
                                    None => 59
                                }].0 += 1}
                            else {
                                carteh[ match carteh.iter().position(|r| r == &generica_stringa) {
                                    Some(value) => value,
                                    None => 59
                                }] = carta;
                                frequenza_carteh[ match carteh.iter().position(|r| r == &generica_stringa) {
                                    Some(value) => value,
                                    None => 59     // Notare che qui si genera un errore
                                } -1 ] = (1, carta);}
                        }
                    }
            _ => { // se ho perso la parita...
                for carta in &partita.mazzo {
            //        print!(" 1 {}  1 ", carta);
                    if cartes.contains(&carta) {
                        frequenza_cartes[ match cartes.iter().position(|r| r == &carta) {
                            Some(value) => value,
                            None => 59
                        }].0 += 1}
                    else {
                        cartes[ match cartes.iter().position(|r| r == &generica_stringa) {
                            Some(value) => value,
                            None => 59
                        }] = carta;
                        frequenza_cartes[ match cartes.iter().position(|r| r == &generica_stringa ) {
                            Some(value) => value,
                            None => 59
                        } -1 ] = (1, carta); }
                } // notare che individuo la posizione della tupla grazie al primo e al terzo array.
            } //non ho davvero bisogno di questi due array. Potrei ottimizzare il programma
        }
             // Debug
//         println!("{:?}, {}, {}, {}", partita.mazzo, partita.esito, partita.risultato.0, partita.risultato.0);
    }

    // In questa parte è visualizzato l'output, non mostro i dati in modo assoluto perchè è inutile:
    // verrebbero fuori solo numeri molto grandi difficili da confrontare e con cui ragionare. In questo
    // modo sono già elaborati!

    println!("Questo è l'elenco delle carte che più di frequente si trovano nei pazzi che ti battono:");
    for i in frequenza_cartes {
        if i.0 > 5 &&  carteh.contains(&i.1) && i.0 as f32 / frequenza_carteh[carteh.iter().position(|r| r == &i.1).unwrap()].0 as f32 > 1. {
            println!("la carta {} è presente in {} mazzi;", i.1,
            i.0 as f32 / frequenza_carteh[carteh.iter().position(|r| r == &i.1).unwrap()].0 as f32);}
        else if i.0 > 5 && !carteh.contains(&i.1)  { println!("la carta {} è presente in {} mazzi;", i.1, i.0) }
    }

    println!("Questo è l'elenco delle carte che più di frequente si trovano nei pazzi che batti:");
    for i in frequenza_carteh {
        if i.0 > 5 && cartes.contains(&i.1) && i.0 as f32 / frequenza_cartes[cartes.iter().position(|r| r == &i.1).unwrap()].0 as f32 > 1. {
            println!("la carta {} è presente in {} mazzi", i.1,
            i.0 as f32 / frequenza_cartes[cartes.iter().position(|r| r == &i.1).unwrap()].0 as f32);}
        else if i.0 > 5 && !cartes.contains(&i.1) { println!("la carta {} è presente in {} mazzi;", i.1, i.0) }
    }
}

// piccoli miglioramenti che mi vengono in mente. Potrei costruire di già i set di cui ho bisogno (quelli con le carte)
// inoltre il set con le carte potrebbe contenere una tupla che indica vari dati riguardo alla carta: per esempio
// carta di "stato": gelo, furia, terremoto, ...
// carta usa e getta: gli spiriti + palla di fuoco, di neve, fulmine, ... (effetto istantaneo)
// carta assalto: danno singolo, corpo a corpo
// carta mischia: danno ad area, corpo a corpo
// arcieri: danno singolo, a distanza
// mortaio: danno ad area, a distanza
// chi va sulle strutture e chi no

/* In questo modo si potrebbe analizzare i dati in modo più completo
    sarebbe fiko effettuare uno studio dei mazzi che sono generati per notare
    strategie ignorate/ legami tra le carte che noi ignoriamo
    Sarebbe interessante anche introdurre uno studio sui livelli delle carte
    Se c'è una netta differenza tra un livello piuttosto che un altro

    Ma prima di tutto vorrei creare un file che sia molto leggero in cui salvare gli array che genero
    in modo tale da non doverli ricalcolare ogni volta. Magari così li possono anche aprire con un altro
    linguaggio
*/
