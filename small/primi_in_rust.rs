fn main() {
	let mut primes = vec![2, 3, 5, 7, 9, 11];

// conosciamo la quantità di numeri primi, per cui si potrebbe utilizzare un array:
// let mut primes = [2; numero_di_primi];
// let mut pointer = 1;

	let numbmax = 1_000_000; // limite superiore che diamo al programma
	for i in 12..(numbmax+1){ // dal momento che ho già specificato i primi numeri primi parto da 12
		primes.push(i);

//        primes[pointer] = i;
//		  pointer += 1;
		for prime in 0..&primes.len()-1 { // si divide per tutti i primi più piccoli
			if i % primes[prime] == 0 {
				primes.pop(); // se viene trovato un divisore del numero, il numero è tolto dal vettore

// 				  pointer -= 1 // se fosse un array, non si toglie l'elemento, lo si riscrive

				break; // non ha più senso continuare a cercare divisori
			};
		};
	};
	print!("{:?}", primes)
}
