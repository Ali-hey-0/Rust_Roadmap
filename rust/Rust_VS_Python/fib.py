from time import perf_counter


def fib(n):
    n1 = 1
    n2 = 1
    
    for _ in range(3, n+1):
        n1, n2 = n2, n1 + n2
    
    return n2
    
    
if __name__ == "__main__":
    n = int(input("N: "))
    st = perf_counter()
    r = fib(n)
    end = perf_counter()
    result = end - st 
    print(r)
    print("{:.6f} seconds".format(result))
    
# for 500 0.000516 seconds