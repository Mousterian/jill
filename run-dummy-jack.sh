# jack seems to be suspiciously quiet on raspbian
# need to switch on verbose to see what it's up to
jackd -v -r -ddummy -r44100 -p1024 &