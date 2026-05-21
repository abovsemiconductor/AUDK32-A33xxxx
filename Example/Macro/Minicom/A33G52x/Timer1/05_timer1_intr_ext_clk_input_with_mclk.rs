# A. Description
#    A list of commands here configures timer1 and generates a periodic interrupt based on external clock source.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer1
#    2. SCUCLK
#    3. PCU/GPIO
#
# D. Default Port
#    1. Timer10 : PB0 (Output)
#                 PA6 (Capture Input/External Clock Input)
#    2. PCU     : PC13 (Clock Output)
#
# E. Port Connection
#    1. PC13 (Clock Output) ----> PA6 (Timer10 External Clock Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 e f 100 ]
#    Source                    : e (External Clock Input) 
#    External Clock Input Edge : f (Falling) 
#    Timer1 Pre-Divide         : 1000
#
# 3. Config                    : [ 0 i p h 1000 1000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Periodic)
#    Output Port Polarity      : h (High)
#    A Data                    : 1000
#    B Data                    : 1000
#
# SCUCLK
# 1. Clock Out                 : [ clkout m 8 e ]  
#    Clock Source              : m (MCLK - HSI 16MHz)
#    Clock Source Divide       : 8 (Clock Source / (2 * (Divide))
#    Clock Output Enable       : e (Enable)
#
# PCU (PC13)
# 1. Port Group                : 2 (PCU Port C)
#
# 2. Port                      : [ 2 13 a 1 ]
#    Pin Number                : 13
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (Clock Output)
#
# 3. Port                      : [ 2 13 o p h ]
#    Pin Number                : 6
#    Pin Mode                  : o (Output)
#    Pin Output Mode           : p (Push-Pull)
#    Pin Level                 : h (High)
#
# PCU (PA6)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 6 i i ]
#    Pin Number                : 6
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : i (Input)

# PCU (PC13)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 13 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 13 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

# PCU (PA6)
send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 6 i i"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

# SCUCLK
send "cm scuclk"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

send "clkout m 8 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

# Timer10
send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 0 e f 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 1000 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

end:
