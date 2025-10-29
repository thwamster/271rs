import sys
import binascii
from ed25519 import *

# examples of inputs: see sign.input
# should produce no output: python sign.py < sign.input

# warning: currently 37 seconds/line on a fast machine

# fields on each input line: sk, pk, m, sm
# each field hex
# each field colon-terminated
# sk includes pk at end
# sm includes m at end

while 1:
  line = sys.stdin.readline()
  if not line: break
  x = line.split(":")
  sk = binascii.unhexlify(x[0][0:64])
  m = binascii.unhexlify(x[2])
  pk = publickey(sk)
  s = signature(m, sk, pk)
  assert checkvalid(s, m, pk)
  assert bytes(x[0].encode("latin-1")) == binascii.hexlify(sk + pk)
  assert bytes(x[1].encode("latin-1")) == binascii.hexlify(pk)
  assert bytes(x[3].encode("latin-1")) == binascii.hexlify(s + m)
