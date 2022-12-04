import re
l=[[int(j)for j in re.split('[,-]',x)]for x in open('data/input')]
A=B=0
for a,x,b,y in l:A+=(a-b)*(x-y)<1;B+=(x-b)*(a-y)<1
print(A,B)