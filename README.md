# some-little-programms
bib.py generate a .bib file from a .txt file. 
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
