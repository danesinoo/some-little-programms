fname=input('Enter the file name: ')
rfile = open(fname + '.txt')

def Split(str, a):
    if len(a)== 1: return str.split(a[0])
    else: return Split(a[1].join(str.split(a[0])), a[1:])

#in generale l'idea è di far ripetere lo split per ogni stringa in a.

bib = open (fname + '.bib', 'w')

for line in rfile:
    line= Split(line, (';','{','}','\n'))
    for i in range(line.count('')): line.remove('')
    for i in range(len(line)): line[i]= line[i].strip(' \.'.strip('.'))
    if 'bibitem' in line: line.remove('bibitem')
    if 'textit' in line: line.remove('textit')

    #if it's found an url in the last position the element is inserted in the
    #@onlile section. Data are inserted as follow:
    #name of the host, name if the page end url (of course it's reported the 
    #keyword for the reference as well)

    if line[-1].startswith('https://'):

        bib.write('@online{' + line[0] + ',\n')
        bib.write('    author    = "' + line[1] + '",\n')
        bib.write('    title     = "' + line[2] + '",\n')
        bib.write('    url       = "' + line[-1] + '"\n}\n\n')

    #everything else is inserted in @book category. Data are inserted as follow:
    #author, title, publisher, year of published (of course it's reported the
    #keyword for the reference as well)

    else:
        bib.write('@book{' + line[0] + ',\n')
        bib.write('  author =       "' + line[1] + '",\n')
        bib.write('  title =        "' + line[2] + '",\n')
        bib.write('  publisher=     "' + line[3] + '",\n')
        bib.write('  year =         "' + line[-1] + '"\n}\n\n')
bib.close()
