import time
import eratosthenes

def is_prime(n):    
    if n < 2:
        return False
    elif n==2:
        return True
    elif n%2==0:
        return False
    else:
        i =3
        while i*i<=n:
            if n%i ==0:
                return False
            i += 2
        return True

if __name__ == '__main__':
    start = time.time()
    #for i in range(0,10000000):
    #for i in range(0,100):
   #    if is_prime(i):
    #        print(i)
    print(eratosthenes.get_prime_numbers(10))
    elapsed_time = time.time() - start
    print ("elapsed_time:{:.3f}".format(elapsed_time) + "[sec]")