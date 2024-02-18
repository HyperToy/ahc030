N = int(input())

sum = 0
for i in range(0, N):
    score = int(input().split(" ")[2])
    sum += score

print("case: {}".format(N))
print("sum : {0:,}".format(sum))
print("avg : {0:,}".format(int(sum / N)))
