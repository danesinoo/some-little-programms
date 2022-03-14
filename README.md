# some-little-programms
bib.py generates a .bib file from a .txt file. 
It's useful to make a reference file for LaTex:
if you already have a bibiography manual written (\bibitem{}{} like) it's enough to copy it 
in a text file, then this programm will convert it to .bib. I needed to convert only @book and 
@online, but it's easy personalizable. Just to warn, it's written from me for me, so I think it
will take a while to use it in the best way.

It fairly convert texts like:

\bibitem{1}{Halliday D., Resnick R., Walker J.}{\textit{Fondamenti di fisica, Onde, Campo elettrico e magnetico} Zanichelli} 2017}
\bibitem{2}{Halliday D., Resnick R., Walker J.;\textit{Fondamenti di fisica, Induzione e onde elettromagnetiche. Relatività e quanti; Zanichelli; 2018}
\bibitem{3}{Meola Mario; \textit{Ecografia clinica e color doppler in nefrologia}; Eureka; 2008}
\bibitem{4}{ Giannella Giovanni; \textit{Effetto doppler e applicazioni astrofisiche}; Università di Bologna, Corso di Studio in Astronomia; \url{https://amslaurea.unibo.it/7149/1/Giannella_Giovanni_tesi.pdf};2014}

or even:
\bibitem{nasa}{\textit{2020 Mission Perceverance rover; NASA science}: \url{https://mars.nasa.gov/mars2020/timeline/landing/entry-descent-landing/}} 
\bibitem{atmosfera}{Musella M. A. (febbraio 2021); La matematica di Perceverance, Accademia delle Stelle; \url{https://accademiadellestelle.org/la-matematica-di-perseverance/}} 
\bibitem{tempo}{Marinari M. (9 febbraio 2021); Close-up engineering; \url{https://aerospacecue.it/rover-nasa-perseverance-atterraggio-marte-7-minutes-terror/24040/}} 
\bibitem{costo}{Redazione Ansa (29 luglio 2020); Ansa; \url{https://www.ansa.it/canale_scienza_tecnica/notizie/spazio_astronomia/2020/07/29/marte-piu-solidi-gli-indizi-di-vita-e-mars-2020-e-pronta-al-lancio_64a4092b-06a3-439a-89fa-63af70941ce2.html}} 
\bibitem{platino}{Costo platino; il 07/05/2021; \url{https://www.coininvest.com/it/grafici/prezzo-platino/oncia/1giorno/}} 
\bibitem{campoM}{Wikipedia; \textit{Marte (astronomia)}; \url{https://it.wikipedia.org/wiki/Marte_(astronomia)}} 
\bibitem{campoT}{Wikipedia; Campo geomagnetico; \url{https://it.wikipedia.org/wiki/Campo_geomagnetico}}




On the other hand the second programm is about poker:
This programm play 1 milion mathces at poker Texas hold 'em in 1'30'', with my computer,
which isn't really powerful. 
You insert your cards, say how many players are in the game and then it starts the count.
It's easily manageable, so you can insert the cards in the center of the table while you're 
playing 


I also code clash.rs through which I tried and enjoyed rust. The .txt file is the input of the programm. The programm needs to be compiled. clash.rs gives you back the cards you may have some trouble against and the ones which you may not. You need to write by hand the matches and the result. There need to be 8 words, 1 letter, which may be V or S and two numbers based on the crowns you earned and the ones the opponent did. The code doesn't work if the input isn't written properly, for this reason I wrote a second programm which detect mistakes, though It happened I deleted it. I don't use the crown variable, I wasn't sure about the algorithm to make it useful. You can choose however you like how to call the cards, just remember it needs to be only one word. An advice may be using come character like _ ? . , to be able to write more than one word to determine the card. The output will use the chosen word. :3
