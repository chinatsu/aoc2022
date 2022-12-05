import re
A=B=0
for x in open('data/input'):a,b,x,y=map(int,re.split('[,-]',x));A+=(a-x)*(b-y)<1;B+=x<=b>=a<=y
print(A,B)