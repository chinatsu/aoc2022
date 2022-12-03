l=[[ord(i)-(38if i.isupper()else 96) for i in l]for l in open('data/input').readlines()];print(sum(max(set(b[:len(b)//2]).intersection(set(b[len(b)//2:])))for b in l),sum(max(set(c[0]).intersection(set(c[1])).intersection(set(c[2])))for c in[l[s:min(s+3,len(l))]for s in range(0,len(l),3)]),sep="\n")

