n=input('enter a number: ')
try:
    n=int(n)
except:
    print('sorry cannot understand')
    exit ()
nums=[]
for p in range(2,n+1):
    nums.append(p)
    for num in nums[:len(nums)-1]: #da rivedere, può essere semplificato
        if p % num == 0:
            nums.remove(p)
            break
print(nums)
