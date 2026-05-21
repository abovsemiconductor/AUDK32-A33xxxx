# A. Description
#    A list of commands here configures PCU module and generates non-maskable input interrupt via the signal from output port.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. PCU/GPIO
#
# D. Default Port
#    1. PCU     : PA0 (Input)
#               : PF0 (Output)
#
# E. Port Connection
#    1. PF0 (Output) ----> PA0 (Input)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PA0)
# 1. Port Group                   : 0 (PCU Port A)
#
# 2. Port                         : [ 0 0 i i -intr n e b ] 
#    Pin Number                   : 0
#    Pin Mode                     : i (Input)
#    Pin Input Mode               : i (Normal Input)
#    Pin Operation Mode           : n (Non Maskable Interrupt)
#    Pin Interrupt Mode           : e (Edge)
#    Pin Interrupt Trigger Edge   : b (Both Edge)
#
# PCU (PF0)
# 1. Port Group                   : 5 (PCU Port F)
#
# 2. Port                         : [ 5 0 o p h ]
#    Pin Number                   : 0
#    Pin Mode                     : o (Output)
#    Pin Output Mode              : p (Push-Pull)
#    Pin Level                    : h (High)
#
# 3. Output                       : [ 5 0 l / h ]
#    Pin Number                   : 0
#    Pin Level                    : l (Low) / h (High)
#
# PCU (PF0)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 5 0 o p h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

# PCU (PA0)
send "port 0 0 i i -intr n e b"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

set a 0
set b 10

loop:
inc a
send "output 5 0 l"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1

send "output 5 0 h"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}
sleep 1

if a < b goto loop

end:

