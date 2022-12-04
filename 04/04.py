import re
l=[[int(j)for j in re.split('[,-]',x)]for x in open('data/input')]
A=B=0
for a,x,b,y in l:B+=a<=y and x>=b;A+=(a<=b and x>=y)or(a>=b and x<=y)
print(A,B)