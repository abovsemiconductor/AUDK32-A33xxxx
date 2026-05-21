# A. Description
#    A list of commands here configures UART and receives data.
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
# 4. Rx                        : [ 1 8 ]
#    Receive Data Length       : 8
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 12 a 1 ] [ 3 13 a 1 ]
#    Pin Number                : 12 / 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (TX/RX)
#
# 3. Port                      : [ 3 12 i i ]
#    Pin Number                : 12
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


# UART1
send "cm uart"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

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

send "rx 1 8"
expect {
    "<UART> # "
    break
    timeout 5 goto end
}

end:
