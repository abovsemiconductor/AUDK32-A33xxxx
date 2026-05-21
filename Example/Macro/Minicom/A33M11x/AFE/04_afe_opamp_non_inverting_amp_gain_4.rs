# A. Description
#    A list of commands here configures AFE and OP-Amp mode.
#
# B. Preparation
#    Supply power to input port by a suitable instrument or make a environment to generate 1V.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. AFE 
#    3. PCU/GPIO
#
# D. Default Port
#    1. AFE0    : PA0 (AO0)
#               : PA1 (AIN0)
#               : PA2 (AIP0)
#
# E. Port Connection
#    1. Supply Power + ----> PA2 (AIP0)
#       GND -----> R1 -----> PA1 (AIN0)
#       R1 ------> R2 -----> PA0 (AO0)
#
# F. Circuit Configuration
#                                  R2      
#                          ---- /\/\/\/ ----
#                          |               |
#                    R1    |     | \       |
#       GND ----- /\/\/\/ -------|- \      |
#                                |   --------- Vout
#                Vin ------------|+ /
#                                | /
#
#               < Non-inverting Amplifier >
#
#  Non-inverting Amplifier Gain Formula : (1 + R2/R1)
#
#  R1 Resistor : 10K Ohm
#  R2 Resistor : 30K Ohm
#  Gain : 1 + (30K/10K) = 4
#  Vout = 4*Vin = 4V
#
# For more information, read a user's manual of the target device carefully.
#
# AFE0
# 1. Config                    : [ 0 i o ]
#    Operation                 : i (Any operation can be possible.)
#    Mode                      : o (OP-Amp)
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 0 a 7 ] [ 0 1 a 7 ] [ 0 2 a 7 ] [ 0 4 a 1 ]
#    Pin Number                : 0 / 1 / 2 / 4
#    Pin Mode                  : a (Alternative)
#    Alternative               : 7 (AO0) / 7 (AIN0) / 7 (AIP0) / 1 (T0IO)
#

# PCU (PAx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 0 a 7"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 1 a 7"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 2 a 7"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 4 a 7"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# AFE0
send "cm afe"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

send "config 0 i o"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

send "log on 0"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<AFE> # "
    break
    timeout 5 goto end
}

end:
