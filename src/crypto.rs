#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach0ctl: DMACH0CTL,
    #[doc = "0x04 - Channel 0 External Address"]
    pub dmach0extaddr: DMACH0EXTADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Channel 0 DMA Length"]
    pub dmach0len: DMACH0LEN,
    _reserved3: [u8; 8usize],
    #[doc = "0x18 - DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    pub dmastat: DMASTAT,
    #[doc = "0x1c - DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
    pub dmaswreset: DMASWRESET,
    #[doc = "0x20 - Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach1ctl: DMACH1CTL,
    #[doc = "0x24 - Channel 1 External Address"]
    pub dmach1extaddr: DMACH1EXTADDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - Channel 1 DMA Length"]
    pub dmach1len: DMACH1LEN,
    _reserved8: [u8; 72usize],
    #[doc = "0x78 - DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    pub dmabuscfg: DMABUSCFG,
    #[doc = "0x7c - DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
    pub dmaporterr: DMAPORTERR,
    _reserved10: [u8; 124usize],
    #[doc = "0xfc - DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    pub dmahwver: DMAHWVER,
    _reserved11: [u8; 768usize],
    #[doc = "0x400 - Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    pub keywritearea: KEYWRITEAREA,
    #[doc = "0x404 - Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    pub keywrittenarea: KEYWRITTENAREA,
    #[doc = "0x408 - Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    pub keysize: KEYSIZE,
    #[doc = "0x40c - Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    pub keyreadarea: KEYREADAREA,
    _reserved15: [u8; 240usize],
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey2: AESKEY2,
    _reserved16: [u8; 12usize],
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey3: AESKEY3,
    _reserved17: [u8; 44usize],
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aesiv: AESIV,
    _reserved18: [u8; 12usize],
    #[doc = "0x550 - AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
    pub aesctl: AESCTL,
    #[doc = "0x554 - AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen0: AESDATALEN0,
    #[doc = "0x558 - AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen1: AESDATALEN1,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: AESAUTHLEN,
    _reserved_22_aesdatain0: [u8; 4usize],
    _reserved_23_aesdatain1: [u8; 4usize],
    _reserved_24_aesdatain2: [u8; 4usize],
    _reserved_25_aesdatain3: [u8; 4usize],
    #[doc = "0x570 - AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    pub aestagout: AESTAGOUT,
    _reserved27: [u8; 144usize],
    #[doc = "0x604 - HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain1: HASHDATAIN1,
    #[doc = "0x608 - HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain2: HASHDATAIN2,
    #[doc = "0x60c - HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain3: HASHDATAIN3,
    #[doc = "0x610 - HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain4: HASHDATAIN4,
    #[doc = "0x614 - HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain5: HASHDATAIN5,
    #[doc = "0x618 - HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain6: HASHDATAIN6,
    #[doc = "0x61c - HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain7: HASHDATAIN7,
    #[doc = "0x620 - HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain8: HASHDATAIN8,
    #[doc = "0x624 - HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain9: HASHDATAIN9,
    #[doc = "0x628 - HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain10: HASHDATAIN10,
    #[doc = "0x62c - HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain11: HASHDATAIN11,
    #[doc = "0x630 - HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain12: HASHDATAIN12,
    #[doc = "0x634 - HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain13: HASHDATAIN13,
    #[doc = "0x638 - HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain14: HASHDATAIN14,
    #[doc = "0x63c - HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain15: HASHDATAIN15,
    #[doc = "0x640 - HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain16: HASHDATAIN16,
    #[doc = "0x644 - HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain17: HASHDATAIN17,
    #[doc = "0x648 - HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain18: HASHDATAIN18,
    #[doc = "0x64c - HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain19: HASHDATAIN19,
    #[doc = "0x650 - HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain20: HASHDATAIN20,
    #[doc = "0x654 - HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain21: HASHDATAIN21,
    #[doc = "0x658 - HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain22: HASHDATAIN22,
    #[doc = "0x65c - HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain23: HASHDATAIN23,
    #[doc = "0x660 - HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain24: HASHDATAIN24,
    #[doc = "0x664 - HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain25: HASHDATAIN25,
    #[doc = "0x668 - HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain26: HASHDATAIN26,
    #[doc = "0x66c - HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain27: HASHDATAIN27,
    #[doc = "0x670 - HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain28: HASHDATAIN28,
    #[doc = "0x674 - HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain29: HASHDATAIN29,
    #[doc = "0x678 - HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain30: HASHDATAIN30,
    #[doc = "0x67c - HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain31: HASHDATAIN31,
    #[doc = "0x680 - HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    pub hashiobufctrl: HASHIOBUFCTRL,
    #[doc = "0x684 - HASH Mode"]
    pub hashmode: HASHMODE,
    #[doc = "0x688 - HASH Input Length LSB"]
    pub hashinlenl: HASHINLENL,
    #[doc = "0x68c - HASH Input Length MSB"]
    pub hashinlenh: HASHINLENH,
    _reserved62: [u8; 48usize],
    #[doc = "0x6c0 - HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesta: HASHDIGESTA,
    #[doc = "0x6c4 - HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestb: HASHDIGESTB,
    #[doc = "0x6c8 - HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestc: HASHDIGESTC,
    #[doc = "0x6cc - HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestd: HASHDIGESTD,
    #[doc = "0x6d0 - HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigeste: HASHDIGESTE,
    #[doc = "0x6d4 - HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestf: HASHDIGESTF,
    #[doc = "0x6d8 - HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestg: HASHDIGESTG,
    #[doc = "0x6dc - HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesth: HASHDIGESTH,
    #[doc = "0x6e0 - HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesti: HASHDIGESTI,
    #[doc = "0x6e4 - HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestj: HASHDIGESTJ,
    #[doc = "0x6e8 - HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestk: HASHDIGESTK,
    #[doc = "0x6ec - HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestl: HASHDIGESTL,
    #[doc = "0x6f0 - HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestm: HASHDIGESTM,
    #[doc = "0x6f4 - HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestn: HASHDIGESTN,
    #[doc = "0x6f8 - HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesto: HASHDIGESTO,
    #[doc = "0x6fc - HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestp: HASHDIGESTP,
    #[doc = "0x700 - Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
    pub algsel: ALGSEL,
    #[doc = "0x704 - DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    pub dmaprotctl: DMAPROTCTL,
    _reserved80: [u8; 56usize],
    #[doc = "0x740 - Software Reset"]
    pub swreset: SWRESET,
    _reserved81: [u8; 60usize],
    #[doc = "0x780 - Control Interrupt Configuration"]
    pub irqtype: IRQTYPE,
    #[doc = "0x784 - Control Interrupt Enable"]
    pub irqen: IRQEN,
    #[doc = "0x788 - Control Interrupt Clear"]
    pub irqclr: IRQCLR,
    #[doc = "0x78c - Control Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x790 - Control Interrupt Status"]
    pub irqstat: IRQSTAT,
    _reserved86: [u8; 104usize],
    #[doc = "0x7fc - Hardware Version"]
    pub hwver: HWVER,
}
impl RegisterBlock {
    #[doc = "0x560 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain0(&self) -> &AESDATAIN0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1376usize) as *const AESDATAIN0) }
    }
    #[doc = "0x560 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain0_mut(&self) -> &mut AESDATAIN0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut AESDATAIN0) }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout0(&self) -> &AESDATAOUT0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1376usize) as *const AESDATAOUT0) }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout0_mut(&self) -> &mut AESDATAOUT0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut AESDATAOUT0) }
    }
    #[doc = "0x564 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain1(&self) -> &AESDATAIN1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1380usize) as *const AESDATAIN1) }
    }
    #[doc = "0x564 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain1_mut(&self) -> &mut AESDATAIN1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1380usize) as *mut AESDATAIN1) }
    }
    #[doc = "0x564 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout1(&self) -> &AESDATAOUT1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1380usize) as *const AESDATAOUT1) }
    }
    #[doc = "0x564 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout1_mut(&self) -> &mut AESDATAOUT1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1380usize) as *mut AESDATAOUT1) }
    }
    #[doc = "0x568 - AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain2(&self) -> &AESDATAIN2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1384usize) as *const AESDATAIN2) }
    }
    #[doc = "0x568 - AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain2_mut(&self) -> &mut AESDATAIN2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1384usize) as *mut AESDATAIN2) }
    }
    #[doc = "0x568 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout2(&self) -> &AESDATAOUT2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1384usize) as *const AESDATAOUT2) }
    }
    #[doc = "0x568 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout2_mut(&self) -> &mut AESDATAOUT2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1384usize) as *mut AESDATAOUT2) }
    }
    #[doc = "0x56c - AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain3(&self) -> &AESDATAIN3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1388usize) as *const AESDATAIN3) }
    }
    #[doc = "0x56c - AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub fn aesdatain3_mut(&self) -> &mut AESDATAIN3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1388usize) as *mut AESDATAIN3) }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout3(&self) -> &AESDATAOUT3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1388usize) as *const AESDATAOUT3) }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout3_mut(&self) -> &mut AESDATAOUT3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1388usize) as *mut AESDATAOUT3) }
    }
}
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0ctl](dmach0ctl) module"]
pub type DMACH0CTL = crate::Reg<u32, _DMACH0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0CTL;
#[doc = "`read()` method returns [dmach0ctl::R](dmach0ctl::R) reader structure"]
impl crate::Readable for DMACH0CTL {}
#[doc = "`write(|w| ..)` method takes [dmach0ctl::W](dmach0ctl::W) writer structure"]
impl crate::Writable for DMACH0CTL {}
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach0ctl;
#[doc = "Channel 0 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0extaddr](dmach0extaddr) module"]
pub type DMACH0EXTADDR = crate::Reg<u32, _DMACH0EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0EXTADDR;
#[doc = "`read()` method returns [dmach0extaddr::R](dmach0extaddr::R) reader structure"]
impl crate::Readable for DMACH0EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmach0extaddr::W](dmach0extaddr::W) writer structure"]
impl crate::Writable for DMACH0EXTADDR {}
#[doc = "Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "Channel 0 DMA Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0len](dmach0len) module"]
pub type DMACH0LEN = crate::Reg<u32, _DMACH0LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0LEN;
#[doc = "`read()` method returns [dmach0len::R](dmach0len::R) reader structure"]
impl crate::Readable for DMACH0LEN {}
#[doc = "`write(|w| ..)` method takes [dmach0len::W](dmach0len::W) writer structure"]
impl crate::Writable for DMACH0LEN {}
#[doc = "Channel 0 DMA Length"]
pub mod dmach0len;
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](dmastat) module"]
pub type DMASTAT = crate::Reg<u32, _DMASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTAT;
#[doc = "`read()` method returns [dmastat::R](dmastat::R) reader structure"]
impl crate::Readable for DMASTAT {}
#[doc = "`write(|w| ..)` method takes [dmastat::W](dmastat::W) writer structure"]
impl crate::Writable for DMASTAT {}
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmastat;
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaswreset](dmaswreset) module"]
pub type DMASWRESET = crate::Reg<u32, _DMASWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASWRESET;
#[doc = "`read()` method returns [dmaswreset::R](dmaswreset::R) reader structure"]
impl crate::Readable for DMASWRESET {}
#[doc = "`write(|w| ..)` method takes [dmaswreset::W](dmaswreset::W) writer structure"]
impl crate::Writable for DMASWRESET {}
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
pub mod dmaswreset;
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1ctl](dmach1ctl) module"]
pub type DMACH1CTL = crate::Reg<u32, _DMACH1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1CTL;
#[doc = "`read()` method returns [dmach1ctl::R](dmach1ctl::R) reader structure"]
impl crate::Readable for DMACH1CTL {}
#[doc = "`write(|w| ..)` method takes [dmach1ctl::W](dmach1ctl::W) writer structure"]
impl crate::Writable for DMACH1CTL {}
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach1ctl;
#[doc = "Channel 1 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1extaddr](dmach1extaddr) module"]
pub type DMACH1EXTADDR = crate::Reg<u32, _DMACH1EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1EXTADDR;
#[doc = "`read()` method returns [dmach1extaddr::R](dmach1extaddr::R) reader structure"]
impl crate::Readable for DMACH1EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmach1extaddr::W](dmach1extaddr::W) writer structure"]
impl crate::Writable for DMACH1EXTADDR {}
#[doc = "Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "Channel 1 DMA Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1len](dmach1len) module"]
pub type DMACH1LEN = crate::Reg<u32, _DMACH1LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1LEN;
#[doc = "`read()` method returns [dmach1len::R](dmach1len::R) reader structure"]
impl crate::Readable for DMACH1LEN {}
#[doc = "`write(|w| ..)` method takes [dmach1len::W](dmach1len::W) writer structure"]
impl crate::Writable for DMACH1LEN {}
#[doc = "Channel 1 DMA Length"]
pub mod dmach1len;
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabuscfg](dmabuscfg) module"]
pub type DMABUSCFG = crate::Reg<u32, _DMABUSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABUSCFG;
#[doc = "`read()` method returns [dmabuscfg::R](dmabuscfg::R) reader structure"]
impl crate::Readable for DMABUSCFG {}
#[doc = "`write(|w| ..)` method takes [dmabuscfg::W](dmabuscfg::W) writer structure"]
impl crate::Writable for DMABUSCFG {}
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmabuscfg;
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaporterr](dmaporterr) module"]
pub type DMAPORTERR = crate::Reg<u32, _DMAPORTERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAPORTERR;
#[doc = "`read()` method returns [dmaporterr::R](dmaporterr::R) reader structure"]
impl crate::Readable for DMAPORTERR {}
#[doc = "`write(|w| ..)` method takes [dmaporterr::W](dmaporterr::W) writer structure"]
impl crate::Writable for DMAPORTERR {}
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
pub mod dmaporterr;
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmahwver](dmahwver) module"]
pub type DMAHWVER = crate::Reg<u32, _DMAHWVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAHWVER;
#[doc = "`read()` method returns [dmahwver::R](dmahwver::R) reader structure"]
impl crate::Readable for DMAHWVER {}
#[doc = "`write(|w| ..)` method takes [dmahwver::W](dmahwver::W) writer structure"]
impl crate::Writable for DMAHWVER {}
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmahwver;
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywritearea](keywritearea) module"]
pub type KEYWRITEAREA = crate::Reg<u32, _KEYWRITEAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWRITEAREA;
#[doc = "`read()` method returns [keywritearea::R](keywritearea::R) reader structure"]
impl crate::Readable for KEYWRITEAREA {}
#[doc = "`write(|w| ..)` method takes [keywritearea::W](keywritearea::W) writer structure"]
impl crate::Writable for KEYWRITEAREA {}
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod keywritearea;
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywrittenarea](keywrittenarea) module"]
pub type KEYWRITTENAREA = crate::Reg<u32, _KEYWRITTENAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWRITTENAREA;
#[doc = "`read()` method returns [keywrittenarea::R](keywrittenarea::R) reader structure"]
impl crate::Readable for KEYWRITTENAREA {}
#[doc = "`write(|w| ..)` method takes [keywrittenarea::W](keywrittenarea::W) writer structure"]
impl crate::Writable for KEYWRITTENAREA {}
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod keywrittenarea;
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keysize](keysize) module"]
pub type KEYSIZE = crate::Reg<u32, _KEYSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYSIZE;
#[doc = "`read()` method returns [keysize::R](keysize::R) reader structure"]
impl crate::Readable for KEYSIZE {}
#[doc = "`write(|w| ..)` method takes [keysize::W](keysize::W) writer structure"]
impl crate::Writable for KEYSIZE {}
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod keysize;
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreadarea](keyreadarea) module"]
pub type KEYREADAREA = crate::Reg<u32, _KEYREADAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYREADAREA;
#[doc = "`read()` method returns [keyreadarea::R](keyreadarea::R) reader structure"]
impl crate::Readable for KEYREADAREA {}
#[doc = "`write(|w| ..)` method takes [keyreadarea::W](keyreadarea::W) writer structure"]
impl crate::Writable for KEYREADAREA {}
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod keyreadarea;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey2](aeskey2) module"]
pub type AESKEY2 = crate::Reg<u32, _AESKEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESKEY2;
#[doc = "`read()` method returns [aeskey2::R](aeskey2::R) reader structure"]
impl crate::Readable for AESKEY2 {}
#[doc = "`write(|w| ..)` method takes [aeskey2::W](aeskey2::W) writer structure"]
impl crate::Writable for AESKEY2 {}
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey2;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey3](aeskey3) module"]
pub type AESKEY3 = crate::Reg<u32, _AESKEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESKEY3;
#[doc = "`read()` method returns [aeskey3::R](aeskey3::R) reader structure"]
impl crate::Readable for AESKEY3 {}
#[doc = "`write(|w| ..)` method takes [aeskey3::W](aeskey3::W) writer structure"]
impl crate::Writable for AESKEY3 {}
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey3;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesiv](aesiv) module"]
pub type AESIV = crate::Reg<u32, _AESIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESIV;
#[doc = "`read()` method returns [aesiv::R](aesiv::R) reader structure"]
impl crate::Readable for AESIV {}
#[doc = "`write(|w| ..)` method takes [aesiv::W](aesiv::W) writer structure"]
impl crate::Writable for AESIV {}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aesiv;
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](aesctl) module"]
pub type AESCTL = crate::Reg<u32, _AESCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESCTL;
#[doc = "`read()` method returns [aesctl::R](aesctl::R) reader structure"]
impl crate::Readable for AESCTL {}
#[doc = "`write(|w| ..)` method takes [aesctl::W](aesctl::W) writer structure"]
impl crate::Writable for AESCTL {}
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
pub mod aesctl;
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen0](aesdatalen0) module"]
pub type AESDATALEN0 = crate::Reg<u32, _AESDATALEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATALEN0;
#[doc = "`read()` method returns [aesdatalen0::R](aesdatalen0::R) reader structure"]
impl crate::Readable for AESDATALEN0 {}
#[doc = "`write(|w| ..)` method takes [aesdatalen0::W](aesdatalen0::W) writer structure"]
impl crate::Writable for AESDATALEN0 {}
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen0;
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen1](aesdatalen1) module"]
pub type AESDATALEN1 = crate::Reg<u32, _AESDATALEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATALEN1;
#[doc = "`read()` method returns [aesdatalen1::R](aesdatalen1::R) reader structure"]
impl crate::Readable for AESDATALEN1 {}
#[doc = "`write(|w| ..)` method takes [aesdatalen1::W](aesdatalen1::W) writer structure"]
impl crate::Writable for AESDATALEN1 {}
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen1;
#[doc = "AES Authentication Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesauthlen](aesauthlen) module"]
pub type AESAUTHLEN = crate::Reg<u32, _AESAUTHLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAUTHLEN;
#[doc = "`read()` method returns [aesauthlen::R](aesauthlen::R) reader structure"]
impl crate::Readable for AESAUTHLEN {}
#[doc = "`write(|w| ..)` method takes [aesauthlen::W](aesauthlen::W) writer structure"]
impl crate::Writable for AESAUTHLEN {}
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout0](aesdataout0) module"]
pub type AESDATAOUT0 = crate::Reg<u32, _AESDATAOUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT0;
#[doc = "`read()` method returns [aesdataout0::R](aesdataout0::R) reader structure"]
impl crate::Readable for AESDATAOUT0 {}
#[doc = "`write(|w| ..)` method takes [aesdataout0::W](aesdataout0::W) writer structure"]
impl crate::Writable for AESDATAOUT0 {}
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain0](aesdatain0) module"]
pub type AESDATAIN0 = crate::Reg<u32, _AESDATAIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN0;
#[doc = "`read()` method returns [aesdatain0::R](aesdatain0::R) reader structure"]
impl crate::Readable for AESDATAIN0 {}
#[doc = "`write(|w| ..)` method takes [aesdatain0::W](aesdatain0::W) writer structure"]
impl crate::Writable for AESDATAIN0 {}
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain0;
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout1](aesdataout1) module"]
pub type AESDATAOUT1 = crate::Reg<u32, _AESDATAOUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT1;
#[doc = "`read()` method returns [aesdataout1::R](aesdataout1::R) reader structure"]
impl crate::Readable for AESDATAOUT1 {}
#[doc = "`write(|w| ..)` method takes [aesdataout1::W](aesdataout1::W) writer structure"]
impl crate::Writable for AESDATAOUT1 {}
#[doc = "Data Input/Output"]
pub mod aesdataout1;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain1](aesdatain1) module"]
pub type AESDATAIN1 = crate::Reg<u32, _AESDATAIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN1;
#[doc = "`read()` method returns [aesdatain1::R](aesdatain1::R) reader structure"]
impl crate::Readable for AESDATAIN1 {}
#[doc = "`write(|w| ..)` method takes [aesdatain1::W](aesdatain1::W) writer structure"]
impl crate::Writable for AESDATAIN1 {}
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain1;
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout2](aesdataout2) module"]
pub type AESDATAOUT2 = crate::Reg<u32, _AESDATAOUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT2;
#[doc = "`read()` method returns [aesdataout2::R](aesdataout2::R) reader structure"]
impl crate::Readable for AESDATAOUT2 {}
#[doc = "`write(|w| ..)` method takes [aesdataout2::W](aesdataout2::W) writer structure"]
impl crate::Writable for AESDATAOUT2 {}
#[doc = "Data Input/Output"]
pub mod aesdataout2;
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain2](aesdatain2) module"]
pub type AESDATAIN2 = crate::Reg<u32, _AESDATAIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN2;
#[doc = "`read()` method returns [aesdatain2::R](aesdatain2::R) reader structure"]
impl crate::Readable for AESDATAIN2 {}
#[doc = "`write(|w| ..)` method takes [aesdatain2::W](aesdatain2::W) writer structure"]
impl crate::Writable for AESDATAIN2 {}
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain2;
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout3](aesdataout3) module"]
pub type AESDATAOUT3 = crate::Reg<u32, _AESDATAOUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT3;
#[doc = "`read()` method returns [aesdataout3::R](aesdataout3::R) reader structure"]
impl crate::Readable for AESDATAOUT3 {}
#[doc = "`write(|w| ..)` method takes [aesdataout3::W](aesdataout3::W) writer structure"]
impl crate::Writable for AESDATAOUT3 {}
#[doc = "Data Input/Output"]
pub mod aesdataout3;
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain3](aesdatain3) module"]
pub type AESDATAIN3 = crate::Reg<u32, _AESDATAIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN3;
#[doc = "`read()` method returns [aesdatain3::R](aesdatain3::R) reader structure"]
impl crate::Readable for AESDATAIN3 {}
#[doc = "`write(|w| ..)` method takes [aesdatain3::W](aesdatain3::W) writer structure"]
impl crate::Writable for AESDATAIN3 {}
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain3;
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aestagout](aestagout) module"]
pub type AESTAGOUT = crate::Reg<u32, _AESTAGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESTAGOUT;
#[doc = "`read()` method returns [aestagout::R](aestagout::R) reader structure"]
impl crate::Readable for AESTAGOUT {}
#[doc = "`write(|w| ..)` method takes [aestagout::W](aestagout::W) writer structure"]
impl crate::Writable for AESTAGOUT {}
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aestagout;
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain1](hashdatain1) module"]
pub type HASHDATAIN1 = crate::Reg<u32, _HASHDATAIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN1;
#[doc = "`read()` method returns [hashdatain1::R](hashdatain1::R) reader structure"]
impl crate::Readable for HASHDATAIN1 {}
#[doc = "`write(|w| ..)` method takes [hashdatain1::W](hashdatain1::W) writer structure"]
impl crate::Writable for HASHDATAIN1 {}
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain1;
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain2](hashdatain2) module"]
pub type HASHDATAIN2 = crate::Reg<u32, _HASHDATAIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN2;
#[doc = "`read()` method returns [hashdatain2::R](hashdatain2::R) reader structure"]
impl crate::Readable for HASHDATAIN2 {}
#[doc = "`write(|w| ..)` method takes [hashdatain2::W](hashdatain2::W) writer structure"]
impl crate::Writable for HASHDATAIN2 {}
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain2;
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain3](hashdatain3) module"]
pub type HASHDATAIN3 = crate::Reg<u32, _HASHDATAIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN3;
#[doc = "`read()` method returns [hashdatain3::R](hashdatain3::R) reader structure"]
impl crate::Readable for HASHDATAIN3 {}
#[doc = "`write(|w| ..)` method takes [hashdatain3::W](hashdatain3::W) writer structure"]
impl crate::Writable for HASHDATAIN3 {}
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain3;
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain4](hashdatain4) module"]
pub type HASHDATAIN4 = crate::Reg<u32, _HASHDATAIN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN4;
#[doc = "`read()` method returns [hashdatain4::R](hashdatain4::R) reader structure"]
impl crate::Readable for HASHDATAIN4 {}
#[doc = "`write(|w| ..)` method takes [hashdatain4::W](hashdatain4::W) writer structure"]
impl crate::Writable for HASHDATAIN4 {}
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain4;
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain5](hashdatain5) module"]
pub type HASHDATAIN5 = crate::Reg<u32, _HASHDATAIN5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN5;
#[doc = "`read()` method returns [hashdatain5::R](hashdatain5::R) reader structure"]
impl crate::Readable for HASHDATAIN5 {}
#[doc = "`write(|w| ..)` method takes [hashdatain5::W](hashdatain5::W) writer structure"]
impl crate::Writable for HASHDATAIN5 {}
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain5;
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain6](hashdatain6) module"]
pub type HASHDATAIN6 = crate::Reg<u32, _HASHDATAIN6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN6;
#[doc = "`read()` method returns [hashdatain6::R](hashdatain6::R) reader structure"]
impl crate::Readable for HASHDATAIN6 {}
#[doc = "`write(|w| ..)` method takes [hashdatain6::W](hashdatain6::W) writer structure"]
impl crate::Writable for HASHDATAIN6 {}
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain6;
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain7](hashdatain7) module"]
pub type HASHDATAIN7 = crate::Reg<u32, _HASHDATAIN7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN7;
#[doc = "`read()` method returns [hashdatain7::R](hashdatain7::R) reader structure"]
impl crate::Readable for HASHDATAIN7 {}
#[doc = "`write(|w| ..)` method takes [hashdatain7::W](hashdatain7::W) writer structure"]
impl crate::Writable for HASHDATAIN7 {}
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain7;
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain8](hashdatain8) module"]
pub type HASHDATAIN8 = crate::Reg<u32, _HASHDATAIN8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN8;
#[doc = "`read()` method returns [hashdatain8::R](hashdatain8::R) reader structure"]
impl crate::Readable for HASHDATAIN8 {}
#[doc = "`write(|w| ..)` method takes [hashdatain8::W](hashdatain8::W) writer structure"]
impl crate::Writable for HASHDATAIN8 {}
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain8;
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain9](hashdatain9) module"]
pub type HASHDATAIN9 = crate::Reg<u32, _HASHDATAIN9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN9;
#[doc = "`read()` method returns [hashdatain9::R](hashdatain9::R) reader structure"]
impl crate::Readable for HASHDATAIN9 {}
#[doc = "`write(|w| ..)` method takes [hashdatain9::W](hashdatain9::W) writer structure"]
impl crate::Writable for HASHDATAIN9 {}
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain9;
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain10](hashdatain10) module"]
pub type HASHDATAIN10 = crate::Reg<u32, _HASHDATAIN10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN10;
#[doc = "`read()` method returns [hashdatain10::R](hashdatain10::R) reader structure"]
impl crate::Readable for HASHDATAIN10 {}
#[doc = "`write(|w| ..)` method takes [hashdatain10::W](hashdatain10::W) writer structure"]
impl crate::Writable for HASHDATAIN10 {}
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain10;
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain11](hashdatain11) module"]
pub type HASHDATAIN11 = crate::Reg<u32, _HASHDATAIN11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN11;
#[doc = "`read()` method returns [hashdatain11::R](hashdatain11::R) reader structure"]
impl crate::Readable for HASHDATAIN11 {}
#[doc = "`write(|w| ..)` method takes [hashdatain11::W](hashdatain11::W) writer structure"]
impl crate::Writable for HASHDATAIN11 {}
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain11;
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain12](hashdatain12) module"]
pub type HASHDATAIN12 = crate::Reg<u32, _HASHDATAIN12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN12;
#[doc = "`read()` method returns [hashdatain12::R](hashdatain12::R) reader structure"]
impl crate::Readable for HASHDATAIN12 {}
#[doc = "`write(|w| ..)` method takes [hashdatain12::W](hashdatain12::W) writer structure"]
impl crate::Writable for HASHDATAIN12 {}
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain12;
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain13](hashdatain13) module"]
pub type HASHDATAIN13 = crate::Reg<u32, _HASHDATAIN13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN13;
#[doc = "`read()` method returns [hashdatain13::R](hashdatain13::R) reader structure"]
impl crate::Readable for HASHDATAIN13 {}
#[doc = "`write(|w| ..)` method takes [hashdatain13::W](hashdatain13::W) writer structure"]
impl crate::Writable for HASHDATAIN13 {}
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain13;
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain14](hashdatain14) module"]
pub type HASHDATAIN14 = crate::Reg<u32, _HASHDATAIN14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN14;
#[doc = "`read()` method returns [hashdatain14::R](hashdatain14::R) reader structure"]
impl crate::Readable for HASHDATAIN14 {}
#[doc = "`write(|w| ..)` method takes [hashdatain14::W](hashdatain14::W) writer structure"]
impl crate::Writable for HASHDATAIN14 {}
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain14;
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain15](hashdatain15) module"]
pub type HASHDATAIN15 = crate::Reg<u32, _HASHDATAIN15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN15;
#[doc = "`read()` method returns [hashdatain15::R](hashdatain15::R) reader structure"]
impl crate::Readable for HASHDATAIN15 {}
#[doc = "`write(|w| ..)` method takes [hashdatain15::W](hashdatain15::W) writer structure"]
impl crate::Writable for HASHDATAIN15 {}
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain15;
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain16](hashdatain16) module"]
pub type HASHDATAIN16 = crate::Reg<u32, _HASHDATAIN16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN16;
#[doc = "`read()` method returns [hashdatain16::R](hashdatain16::R) reader structure"]
impl crate::Readable for HASHDATAIN16 {}
#[doc = "`write(|w| ..)` method takes [hashdatain16::W](hashdatain16::W) writer structure"]
impl crate::Writable for HASHDATAIN16 {}
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain16;
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain17](hashdatain17) module"]
pub type HASHDATAIN17 = crate::Reg<u32, _HASHDATAIN17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN17;
#[doc = "`read()` method returns [hashdatain17::R](hashdatain17::R) reader structure"]
impl crate::Readable for HASHDATAIN17 {}
#[doc = "`write(|w| ..)` method takes [hashdatain17::W](hashdatain17::W) writer structure"]
impl crate::Writable for HASHDATAIN17 {}
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain17;
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain18](hashdatain18) module"]
pub type HASHDATAIN18 = crate::Reg<u32, _HASHDATAIN18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN18;
#[doc = "`read()` method returns [hashdatain18::R](hashdatain18::R) reader structure"]
impl crate::Readable for HASHDATAIN18 {}
#[doc = "`write(|w| ..)` method takes [hashdatain18::W](hashdatain18::W) writer structure"]
impl crate::Writable for HASHDATAIN18 {}
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain18;
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain19](hashdatain19) module"]
pub type HASHDATAIN19 = crate::Reg<u32, _HASHDATAIN19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN19;
#[doc = "`read()` method returns [hashdatain19::R](hashdatain19::R) reader structure"]
impl crate::Readable for HASHDATAIN19 {}
#[doc = "`write(|w| ..)` method takes [hashdatain19::W](hashdatain19::W) writer structure"]
impl crate::Writable for HASHDATAIN19 {}
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain19;
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain20](hashdatain20) module"]
pub type HASHDATAIN20 = crate::Reg<u32, _HASHDATAIN20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN20;
#[doc = "`read()` method returns [hashdatain20::R](hashdatain20::R) reader structure"]
impl crate::Readable for HASHDATAIN20 {}
#[doc = "`write(|w| ..)` method takes [hashdatain20::W](hashdatain20::W) writer structure"]
impl crate::Writable for HASHDATAIN20 {}
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain20;
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain21](hashdatain21) module"]
pub type HASHDATAIN21 = crate::Reg<u32, _HASHDATAIN21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN21;
#[doc = "`read()` method returns [hashdatain21::R](hashdatain21::R) reader structure"]
impl crate::Readable for HASHDATAIN21 {}
#[doc = "`write(|w| ..)` method takes [hashdatain21::W](hashdatain21::W) writer structure"]
impl crate::Writable for HASHDATAIN21 {}
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain21;
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain22](hashdatain22) module"]
pub type HASHDATAIN22 = crate::Reg<u32, _HASHDATAIN22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN22;
#[doc = "`read()` method returns [hashdatain22::R](hashdatain22::R) reader structure"]
impl crate::Readable for HASHDATAIN22 {}
#[doc = "`write(|w| ..)` method takes [hashdatain22::W](hashdatain22::W) writer structure"]
impl crate::Writable for HASHDATAIN22 {}
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain22;
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain23](hashdatain23) module"]
pub type HASHDATAIN23 = crate::Reg<u32, _HASHDATAIN23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN23;
#[doc = "`read()` method returns [hashdatain23::R](hashdatain23::R) reader structure"]
impl crate::Readable for HASHDATAIN23 {}
#[doc = "`write(|w| ..)` method takes [hashdatain23::W](hashdatain23::W) writer structure"]
impl crate::Writable for HASHDATAIN23 {}
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain23;
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain24](hashdatain24) module"]
pub type HASHDATAIN24 = crate::Reg<u32, _HASHDATAIN24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN24;
#[doc = "`read()` method returns [hashdatain24::R](hashdatain24::R) reader structure"]
impl crate::Readable for HASHDATAIN24 {}
#[doc = "`write(|w| ..)` method takes [hashdatain24::W](hashdatain24::W) writer structure"]
impl crate::Writable for HASHDATAIN24 {}
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain24;
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain25](hashdatain25) module"]
pub type HASHDATAIN25 = crate::Reg<u32, _HASHDATAIN25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN25;
#[doc = "`read()` method returns [hashdatain25::R](hashdatain25::R) reader structure"]
impl crate::Readable for HASHDATAIN25 {}
#[doc = "`write(|w| ..)` method takes [hashdatain25::W](hashdatain25::W) writer structure"]
impl crate::Writable for HASHDATAIN25 {}
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain25;
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain26](hashdatain26) module"]
pub type HASHDATAIN26 = crate::Reg<u32, _HASHDATAIN26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN26;
#[doc = "`read()` method returns [hashdatain26::R](hashdatain26::R) reader structure"]
impl crate::Readable for HASHDATAIN26 {}
#[doc = "`write(|w| ..)` method takes [hashdatain26::W](hashdatain26::W) writer structure"]
impl crate::Writable for HASHDATAIN26 {}
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain26;
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain27](hashdatain27) module"]
pub type HASHDATAIN27 = crate::Reg<u32, _HASHDATAIN27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN27;
#[doc = "`read()` method returns [hashdatain27::R](hashdatain27::R) reader structure"]
impl crate::Readable for HASHDATAIN27 {}
#[doc = "`write(|w| ..)` method takes [hashdatain27::W](hashdatain27::W) writer structure"]
impl crate::Writable for HASHDATAIN27 {}
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain27;
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain28](hashdatain28) module"]
pub type HASHDATAIN28 = crate::Reg<u32, _HASHDATAIN28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN28;
#[doc = "`read()` method returns [hashdatain28::R](hashdatain28::R) reader structure"]
impl crate::Readable for HASHDATAIN28 {}
#[doc = "`write(|w| ..)` method takes [hashdatain28::W](hashdatain28::W) writer structure"]
impl crate::Writable for HASHDATAIN28 {}
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain28;
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain29](hashdatain29) module"]
pub type HASHDATAIN29 = crate::Reg<u32, _HASHDATAIN29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN29;
#[doc = "`read()` method returns [hashdatain29::R](hashdatain29::R) reader structure"]
impl crate::Readable for HASHDATAIN29 {}
#[doc = "`write(|w| ..)` method takes [hashdatain29::W](hashdatain29::W) writer structure"]
impl crate::Writable for HASHDATAIN29 {}
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain29;
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain30](hashdatain30) module"]
pub type HASHDATAIN30 = crate::Reg<u32, _HASHDATAIN30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN30;
#[doc = "`read()` method returns [hashdatain30::R](hashdatain30::R) reader structure"]
impl crate::Readable for HASHDATAIN30 {}
#[doc = "`write(|w| ..)` method takes [hashdatain30::W](hashdatain30::W) writer structure"]
impl crate::Writable for HASHDATAIN30 {}
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain30;
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdatain31](hashdatain31) module"]
pub type HASHDATAIN31 = crate::Reg<u32, _HASHDATAIN31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDATAIN31;
#[doc = "`read()` method returns [hashdatain31::R](hashdatain31::R) reader structure"]
impl crate::Readable for HASHDATAIN31 {}
#[doc = "`write(|w| ..)` method takes [hashdatain31::W](hashdatain31::W) writer structure"]
impl crate::Writable for HASHDATAIN31 {}
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain31;
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashiobufctrl](hashiobufctrl) module"]
pub type HASHIOBUFCTRL = crate::Reg<u32, _HASHIOBUFCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHIOBUFCTRL;
#[doc = "`read()` method returns [hashiobufctrl::R](hashiobufctrl::R) reader structure"]
impl crate::Readable for HASHIOBUFCTRL {}
#[doc = "`write(|w| ..)` method takes [hashiobufctrl::W](hashiobufctrl::W) writer structure"]
impl crate::Writable for HASHIOBUFCTRL {}
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hashiobufctrl;
#[doc = "HASH Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashmode](hashmode) module"]
pub type HASHMODE = crate::Reg<u32, _HASHMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHMODE;
#[doc = "`read()` method returns [hashmode::R](hashmode::R) reader structure"]
impl crate::Readable for HASHMODE {}
#[doc = "`write(|w| ..)` method takes [hashmode::W](hashmode::W) writer structure"]
impl crate::Writable for HASHMODE {}
#[doc = "HASH Mode"]
pub mod hashmode;
#[doc = "HASH Input Length LSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashinlenl](hashinlenl) module"]
pub type HASHINLENL = crate::Reg<u32, _HASHINLENL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHINLENL;
#[doc = "`read()` method returns [hashinlenl::R](hashinlenl::R) reader structure"]
impl crate::Readable for HASHINLENL {}
#[doc = "`write(|w| ..)` method takes [hashinlenl::W](hashinlenl::W) writer structure"]
impl crate::Writable for HASHINLENL {}
#[doc = "HASH Input Length LSB"]
pub mod hashinlenl;
#[doc = "HASH Input Length MSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashinlenh](hashinlenh) module"]
pub type HASHINLENH = crate::Reg<u32, _HASHINLENH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHINLENH;
#[doc = "`read()` method returns [hashinlenh::R](hashinlenh::R) reader structure"]
impl crate::Readable for HASHINLENH {}
#[doc = "`write(|w| ..)` method takes [hashinlenh::W](hashinlenh::W) writer structure"]
impl crate::Writable for HASHINLENH {}
#[doc = "HASH Input Length MSB"]
pub mod hashinlenh;
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigesta](hashdigesta) module"]
pub type HASHDIGESTA = crate::Reg<u32, _HASHDIGESTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTA;
#[doc = "`read()` method returns [hashdigesta::R](hashdigesta::R) reader structure"]
impl crate::Readable for HASHDIGESTA {}
#[doc = "`write(|w| ..)` method takes [hashdigesta::W](hashdigesta::W) writer structure"]
impl crate::Writable for HASHDIGESTA {}
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesta;
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestb](hashdigestb) module"]
pub type HASHDIGESTB = crate::Reg<u32, _HASHDIGESTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTB;
#[doc = "`read()` method returns [hashdigestb::R](hashdigestb::R) reader structure"]
impl crate::Readable for HASHDIGESTB {}
#[doc = "`write(|w| ..)` method takes [hashdigestb::W](hashdigestb::W) writer structure"]
impl crate::Writable for HASHDIGESTB {}
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestb;
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestc](hashdigestc) module"]
pub type HASHDIGESTC = crate::Reg<u32, _HASHDIGESTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTC;
#[doc = "`read()` method returns [hashdigestc::R](hashdigestc::R) reader structure"]
impl crate::Readable for HASHDIGESTC {}
#[doc = "`write(|w| ..)` method takes [hashdigestc::W](hashdigestc::W) writer structure"]
impl crate::Writable for HASHDIGESTC {}
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestc;
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestd](hashdigestd) module"]
pub type HASHDIGESTD = crate::Reg<u32, _HASHDIGESTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTD;
#[doc = "`read()` method returns [hashdigestd::R](hashdigestd::R) reader structure"]
impl crate::Readable for HASHDIGESTD {}
#[doc = "`write(|w| ..)` method takes [hashdigestd::W](hashdigestd::W) writer structure"]
impl crate::Writable for HASHDIGESTD {}
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestd;
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigeste](hashdigeste) module"]
pub type HASHDIGESTE = crate::Reg<u32, _HASHDIGESTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTE;
#[doc = "`read()` method returns [hashdigeste::R](hashdigeste::R) reader structure"]
impl crate::Readable for HASHDIGESTE {}
#[doc = "`write(|w| ..)` method takes [hashdigeste::W](hashdigeste::W) writer structure"]
impl crate::Writable for HASHDIGESTE {}
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigeste;
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestf](hashdigestf) module"]
pub type HASHDIGESTF = crate::Reg<u32, _HASHDIGESTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTF;
#[doc = "`read()` method returns [hashdigestf::R](hashdigestf::R) reader structure"]
impl crate::Readable for HASHDIGESTF {}
#[doc = "`write(|w| ..)` method takes [hashdigestf::W](hashdigestf::W) writer structure"]
impl crate::Writable for HASHDIGESTF {}
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestf;
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestg](hashdigestg) module"]
pub type HASHDIGESTG = crate::Reg<u32, _HASHDIGESTG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTG;
#[doc = "`read()` method returns [hashdigestg::R](hashdigestg::R) reader structure"]
impl crate::Readable for HASHDIGESTG {}
#[doc = "`write(|w| ..)` method takes [hashdigestg::W](hashdigestg::W) writer structure"]
impl crate::Writable for HASHDIGESTG {}
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestg;
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigesth](hashdigesth) module"]
pub type HASHDIGESTH = crate::Reg<u32, _HASHDIGESTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTH;
#[doc = "`read()` method returns [hashdigesth::R](hashdigesth::R) reader structure"]
impl crate::Readable for HASHDIGESTH {}
#[doc = "`write(|w| ..)` method takes [hashdigesth::W](hashdigesth::W) writer structure"]
impl crate::Writable for HASHDIGESTH {}
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesth;
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigesti](hashdigesti) module"]
pub type HASHDIGESTI = crate::Reg<u32, _HASHDIGESTI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTI;
#[doc = "`read()` method returns [hashdigesti::R](hashdigesti::R) reader structure"]
impl crate::Readable for HASHDIGESTI {}
#[doc = "`write(|w| ..)` method takes [hashdigesti::W](hashdigesti::W) writer structure"]
impl crate::Writable for HASHDIGESTI {}
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesti;
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestj](hashdigestj) module"]
pub type HASHDIGESTJ = crate::Reg<u32, _HASHDIGESTJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTJ;
#[doc = "`read()` method returns [hashdigestj::R](hashdigestj::R) reader structure"]
impl crate::Readable for HASHDIGESTJ {}
#[doc = "`write(|w| ..)` method takes [hashdigestj::W](hashdigestj::W) writer structure"]
impl crate::Writable for HASHDIGESTJ {}
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestj;
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestk](hashdigestk) module"]
pub type HASHDIGESTK = crate::Reg<u32, _HASHDIGESTK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTK;
#[doc = "`read()` method returns [hashdigestk::R](hashdigestk::R) reader structure"]
impl crate::Readable for HASHDIGESTK {}
#[doc = "`write(|w| ..)` method takes [hashdigestk::W](hashdigestk::W) writer structure"]
impl crate::Writable for HASHDIGESTK {}
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestk;
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestl](hashdigestl) module"]
pub type HASHDIGESTL = crate::Reg<u32, _HASHDIGESTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTL;
#[doc = "`read()` method returns [hashdigestl::R](hashdigestl::R) reader structure"]
impl crate::Readable for HASHDIGESTL {}
#[doc = "`write(|w| ..)` method takes [hashdigestl::W](hashdigestl::W) writer structure"]
impl crate::Writable for HASHDIGESTL {}
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestl;
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestm](hashdigestm) module"]
pub type HASHDIGESTM = crate::Reg<u32, _HASHDIGESTM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTM;
#[doc = "`read()` method returns [hashdigestm::R](hashdigestm::R) reader structure"]
impl crate::Readable for HASHDIGESTM {}
#[doc = "`write(|w| ..)` method takes [hashdigestm::W](hashdigestm::W) writer structure"]
impl crate::Writable for HASHDIGESTM {}
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestm;
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestn](hashdigestn) module"]
pub type HASHDIGESTN = crate::Reg<u32, _HASHDIGESTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTN;
#[doc = "`read()` method returns [hashdigestn::R](hashdigestn::R) reader structure"]
impl crate::Readable for HASHDIGESTN {}
#[doc = "`write(|w| ..)` method takes [hashdigestn::W](hashdigestn::W) writer structure"]
impl crate::Writable for HASHDIGESTN {}
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestn;
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigesto](hashdigesto) module"]
pub type HASHDIGESTO = crate::Reg<u32, _HASHDIGESTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTO;
#[doc = "`read()` method returns [hashdigesto::R](hashdigesto::R) reader structure"]
impl crate::Readable for HASHDIGESTO {}
#[doc = "`write(|w| ..)` method takes [hashdigesto::W](hashdigesto::W) writer structure"]
impl crate::Writable for HASHDIGESTO {}
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesto;
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashdigestp](hashdigestp) module"]
pub type HASHDIGESTP = crate::Reg<u32, _HASHDIGESTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHDIGESTP;
#[doc = "`read()` method returns [hashdigestp::R](hashdigestp::R) reader structure"]
impl crate::Readable for HASHDIGESTP {}
#[doc = "`write(|w| ..)` method takes [hashdigestp::W](hashdigestp::W) writer structure"]
impl crate::Writable for HASHDIGESTP {}
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestp;
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [algsel](algsel) module"]
pub type ALGSEL = crate::Reg<u32, _ALGSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALGSEL;
#[doc = "`read()` method returns [algsel::R](algsel::R) reader structure"]
impl crate::Readable for ALGSEL {}
#[doc = "`write(|w| ..)` method takes [algsel::W](algsel::W) writer structure"]
impl crate::Writable for ALGSEL {}
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaprotctl](dmaprotctl) module"]
pub type DMAPROTCTL = crate::Reg<u32, _DMAPROTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAPROTCTL;
#[doc = "`read()` method returns [dmaprotctl::R](dmaprotctl::R) reader structure"]
impl crate::Readable for DMAPROTCTL {}
#[doc = "`write(|w| ..)` method takes [dmaprotctl::W](dmaprotctl::W) writer structure"]
impl crate::Writable for DMAPROTCTL {}
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod dmaprotctl;
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](swreset) module"]
pub type SWRESET = crate::Reg<u32, _SWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRESET;
#[doc = "`read()` method returns [swreset::R](swreset::R) reader structure"]
impl crate::Readable for SWRESET {}
#[doc = "`write(|w| ..)` method takes [swreset::W](swreset::W) writer structure"]
impl crate::Writable for SWRESET {}
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "Control Interrupt Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqtype](irqtype) module"]
pub type IRQTYPE = crate::Reg<u32, _IRQTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQTYPE;
#[doc = "`read()` method returns [irqtype::R](irqtype::R) reader structure"]
impl crate::Readable for IRQTYPE {}
#[doc = "`write(|w| ..)` method takes [irqtype::W](irqtype::W) writer structure"]
impl crate::Writable for IRQTYPE {}
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "Control Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqen](irqen) module"]
pub type IRQEN = crate::Reg<u32, _IRQEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQEN;
#[doc = "`read()` method returns [irqen::R](irqen::R) reader structure"]
impl crate::Readable for IRQEN {}
#[doc = "`write(|w| ..)` method takes [irqen::W](irqen::W) writer structure"]
impl crate::Writable for IRQEN {}
#[doc = "Control Interrupt Enable"]
pub mod irqen;
#[doc = "Control Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](irqclr) module"]
pub type IRQCLR = crate::Reg<u32, _IRQCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQCLR;
#[doc = "`read()` method returns [irqclr::R](irqclr::R) reader structure"]
impl crate::Readable for IRQCLR {}
#[doc = "`write(|w| ..)` method takes [irqclr::W](irqclr::W) writer structure"]
impl crate::Writable for IRQCLR {}
#[doc = "Control Interrupt Clear"]
pub mod irqclr;
#[doc = "Control Interrupt Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](irqset) module"]
pub type IRQSET = crate::Reg<u32, _IRQSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSET;
#[doc = "`read()` method returns [irqset::R](irqset::R) reader structure"]
impl crate::Readable for IRQSET {}
#[doc = "`write(|w| ..)` method takes [irqset::W](irqset::W) writer structure"]
impl crate::Writable for IRQSET {}
#[doc = "Control Interrupt Set"]
pub mod irqset;
#[doc = "Control Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstat](irqstat) module"]
pub type IRQSTAT = crate::Reg<u32, _IRQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTAT;
#[doc = "`read()` method returns [irqstat::R](irqstat::R) reader structure"]
impl crate::Readable for IRQSTAT {}
#[doc = "`write(|w| ..)` method takes [irqstat::W](irqstat::W) writer structure"]
impl crate::Writable for IRQSTAT {}
#[doc = "Control Interrupt Status"]
pub mod irqstat;
#[doc = "Hardware Version\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver](hwver) module"]
pub type HWVER = crate::Reg<u32, _HWVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVER;
#[doc = "`read()` method returns [hwver::R](hwver::R) reader structure"]
impl crate::Readable for HWVER {}
#[doc = "`write(|w| ..)` method takes [hwver::W](hwver::W) writer structure"]
impl crate::Writable for HWVER {}
#[doc = "Hardware Version"]
pub mod hwver;
