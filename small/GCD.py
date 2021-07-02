#calcolare il massimo comune divisore alcuni numeri
import sys

def Split(str, a):
    if len(a)== 1: return str.split(a[0])
    else: return Split(a[1].join(str.split(a[0])), a[1:])

def MCD(primo_numero, secondo_numero):
    if primo_numero % secondo_numero== 0: return secondo_numero
    else: return MCD(secondo_numero, primo_numero % secondo_numero)

stringa_numeri= input('Enter the numbers (let a space or a comma between the numbers): ')
numeri= set(Split(stringa_numeri, (' ',',')))

for i in numeri:
    numeri.remove(i)
    numeri.add(int(i))

if 0 in numeri:
    numeri.remove(0)

if len(numeri) == 0: sys.exit('The GCD is 0')

cnumeri= numeri.copy()

primo_numero=cnumeri.pop()
if len(numeri)== 0:
    print('The GCD is', primo_numero)
    quit()

while cnumeri != set():
    secondo_numero = cnumeri.pop()
    primo_numero= MCD(primo_numero, secondo_numero)

print('The Greatest Common Divisor is', primo_numero)
