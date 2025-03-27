package cipher

import "crypto/cipher"

type Stream interface{ XORKeyStream(dst, src []byte) }

func NewCTR(block cipher.Block, iv []byte) Stream
