# A. Description
#    A list of commands here configures timer1 and generates a capture interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer1
#    2. PCU/GPIO
#
# D. Default Port
#    1. Timer10 : PB0 (Output)
#                 PA6 (Capture Input/External Clock Input)
#    2. Timer11 : PB1 (Output)
#                 PA7 (Capture Input/External Clock Input)
#
# E. Port Connection
#    1. PB1 (Timer11 Output) ----> PA6 (Timer10 Capture Input)
#
# For more information, read a user's manual of the target device carefully.
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 p 16 999 ]
#    Source                    : p (PCLK : Peripherl Clock)
#    Source Divide             : 16
#    Timer1 Pre-Divide         : 999 (MCCR Clock) / (Pre-Divide + 1)
#
# 3. Config                    : [ 0 i c r h 0 0 ]
#    Operation                 : i (Interrupt)
#    Mode                      : c (Capture)
#    Capture Egde              : r (Rising Edge)
#    Output Port Polarity      : h (High)
#    A Data                    : 0
#    B Data                    : 0
#
# Timer11
# 1. Channel                   : 1 (Timer11)
#
# 2. Clock                     : [ 0 p 16 999 ]
#    Source                    : p (PCLK : Peripherl Clock)
#    Source Divide             : 16
#    Timer1 Pre-Divide         : 999 (MCCR Clock) / (Pre-Divide + 1)
#
# 3. Config                    : [ 1 i p h 1000 1000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 1000
#    B Data                    : 1000
#
# PCU (PB1)
# 1. Port Group                : 1 (PCU Port B)
#
# 2. Port                      : [ 1 1 a 1 ]
#    Pin Number                : 1
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (T11O)
#
# PCU (PA6)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 6 i i ]
#    Pin Number                : 6
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : i (Input)

# PCU (PB1)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 1 1 a 1"
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

# Timer11
send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "uninit 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 1 p 16 999"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 1 i p h 1000 1000"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

# Timer10
send "init 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 0 p 16 999"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i c r h 0 0"
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

send "log on"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

end:
