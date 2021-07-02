tab=list(set([' 1',' 2',' 3',' 4',' 5',' 6',' 7',' 8',' 9','10','11','12','13','14','15','  ']))

while tab!=['  ', '15', '14', '13', '12', '11', '10', ' 9', ' 8', ' 7', ' 6', ' 5', ' 4', ' 3', ' 2', ' 1',]:

    print(tab[:4]) or print(tab[4:8])
    print(tab[8:12]) or print(tab[12:])
    com=input('Enter a comand: ')
    pos=tab.index('  ')

    if com=='w' and pos<12:
        tab[pos],tab[pos+4]=tab[pos+4],'  '
    if com=='s' and pos>3:
        tab[pos],tab[pos-4]=tab[pos-4],'  '

    if com=='a' and (pos+1)%4!=0:
        tab[pos]=tab[pos+1]
        tab[pos+1]='  '

    if com=='d' and pos%4!=0:
        tab[pos]=tab[pos-1]
        tab[pos-1]='  '

    if com=='done': break

print('YOU WON!')
