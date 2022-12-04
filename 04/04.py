import re
l=[[int(j)for j in re.split('[,-]',x)]for x in open('data/input')]
A=B=0
for a,b,x,y in l:A+=(a-x)*(b-y)<1;B+=(b-x)*(a-y)<1
print(A,B)