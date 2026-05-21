# A. Description
#    A list of commands here configures UART, and transmits and receives data via UART.
#
# B. Preparation
#    Connecting ports with a suitable instrument should be correct
#
# C. Prerequisite Example (abov_example_config.h)
#    1. UART
#    2. PCU/GPIO
#
# D. Default Port
#    1. UART1   : PD13 (Transmit)
#               : PD12 (Receive)
#    2. UART2   : PC11 (Transmit)
#               : PC10 (Receive)
#
# E. Port Connection
#    1. PD13 (UART1 Transmit) <----> PC10 (UART2 Receive)
#    2. PD12 (UART1 Receive)  <----> PC11 (UART2 Transmit)
#
# For more information, read a user's manual of the target device carefully.
#
# UART1
# 1. Channel                   : 1 (UART1)
#
# 2. CLK                       : [ 1 p ]
#    Source                    : p (PCLK : Peripheral Clock)
#
# 3. Config                    : [ 1 i 8 o 1 38400 ]
#    Operation                 : i (Interrupt)
#    Data Bit                  : 8 (8bit)
#    Parity                    : o (odd)
#    Stop Bit                  : 1 (1bit)
#    Baudrate                  : 38400 bps
#
# 4. Data                      : [ 1 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0 ]
#    Data Length               : 8
#    Data                      : 0xa5 ... (Hexa and space (delimitor))
#
# 5. Tx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# UART2
# 1. Channel                   : 2 (UART2)
#
# 2. CLK                       : [ 2 p ]
#    Source                    : p (PCLK : Peripheral Clock)
#
# 3. Config                    : [ 2 i 8 o 1 38400 ]
#    Operation                 : i (Interrupt)
#    Data Bit                  : 8 (8bit)
#    Parity                    : o (odd)
#    Stop Bit                  : 1 (1bit)
#    Baudrate                  : 38400 bps
#
# 4. Rx                        : [ 2 8 ]
#    Receive Data Length       : 8
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 12 a 1 ] [ 3 13 a 1 ]
#    Pin Number                : 12 / 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (RX/TX)
#
# 3. Port                      : [ 3 12 i i ]
#    Pin Number                : 12
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : i (Input)
#
# PCU (PCx)
# 1. Port Group                : 2 (PCU Port C)
#
# 2. Port                      : [ 2 10 a 1 ] [ 2 11 a 1 ]
#    Pin Number                : 10 / 11
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (RX/TX)
#
# 3. Port                      : [ 2 10 i i ]
#    Pin Number                : 10
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : i (Input)

# PCU (PDx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 12 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 12 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 13 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 13 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PCx)
send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 10 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 10 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 11 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 11 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# UART2
send "cm uart"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "uninit 2"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "init 2"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "clk 2 p"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "config 2 i 8 o 1 38400"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "rx 2 8"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

# UART1

send "uninit 1"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "clk 1 p"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "config 1 i 8 o 1 38400"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "data 1 8 0xa5 0x5a 0xa5 0x5a 0x05 0x07 0x09 0xa0"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

send "tx 1 8"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

end:
