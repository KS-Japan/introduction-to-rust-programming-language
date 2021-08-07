import time
import eratosthenes

if __name__ == '__main__':
    start = time.time()
    print(eratosthenes.get_prime_numbers(10000000))
    elapsed_time = time.time() - start
    print ("elapsed_time:{:.3f}".format(elapsed_time) + "[sec]")