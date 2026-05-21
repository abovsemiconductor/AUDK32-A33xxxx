# A. Description
#    A list of commands here configures SPI module and receives data via SPI.
#
# B. Preparation
#    Connecting ports with an external SPI device should be correct.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SPI
#    2. PCU/GPIO
#
# D. Default Port
#    1. SPI0    : PB10 (Slave Select)
#               : PB11 (Serial Clock)
#               : PB12 (Master Out Slave In)
#               : PB13 (Master In Slave Out)
#
# For more information, read a user's manual of the target device carefully.
#
# SPI0
# 1. Channel                   : 0 (SPI0)
#
# 2. Config                    : [ 0 i s 8 0 m h 5 -delay 1 1 1 ]
#    Operation                 : i (Interrupt)
#    Mode                      : s (Slave)
#    Data Length               : 8 (8bit)
#    Polarity/Phase            : 0 (Polarity : Low, Phase : Low)
#    Bit Order                 : m (MSB)
#    Slave Select Pin Polarity : h (High)
#    Baudrate                  : 5 (PCLK / (Baudrate + 1))
#    Delay Start               : 1
#    Delay Stop                : 1
#    Delay Burst               : 1
#
# 3. Rx                        : [ 0 8 ]
#    Receive Data Length       : 8
#
# PCU (PBx)
# 1. Port Group                : 1 (PCU Port B)
#
# 2. Port                      : [ 1 10 a 1 ] [ 1 11 a 1 ] [ 1 12 a 1 ] [ 1 13 a 1 ]
#    Pin Number                : 10 / 11 / 12 / 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (SS0/SCK0/MOSI0/MISO0)
# 
# 3. Port                      : [ 1 11 i i ] [ 1 12 i i ]
#    Pin Number                : 11 / 12
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : i (Input)

# PCU (PBx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 10 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 11 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 12 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 13 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 11 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 12 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# SPI0
send "cm spi"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "config 0 i s 8 0 m h 5 -delay 1 1 1"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

send "rx 0 8"
expect {
    "<SPI> # "
    break
    timeout 5 goto end
}

end:
