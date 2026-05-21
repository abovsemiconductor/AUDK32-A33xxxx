# A. Description
#    A list of commands here configures SCU CLK and generates a certain frequency of PLL Clock.
#
# B. Preparation
#    Make an environment to measure clock frequency by clock output port.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUCLK
#    2. PCU/GPIO
#
# D. Default Port
#    1. PCU    : PC13 (Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# PCU (PC13)
# 1. Port                        : [ 2 13 a 1 ]
#    Port Group Number           : 2 (C group)
#    Pin Number                  : 13
#    Pin Mode                    : Alternative (Mux)
#    Pin Alt Number              : 1
#
# 2. Port                        : [ 2 13 o p h ]
#    Pin Number                  : 6
#    Pin Mode                    : o (Output)
#    Pin Output Mode             : p (Push-Pull)
#    Pin Level                   : h (High)
#
# SCUCLK
# 1. Clkout                      : [ m 0 e ]
#    Clock Source                : m (MCLK : Main Clock)
#    Clock Source Divide         : 0
#    Clock Output Enable         : e (Enable)
#
# 2. PLL                         : [ e h 0 7 47 0 5 s 10 2 ]
#    Enable                      : e (Enable)
#    Clock Source                : h (HSI : High Speed Internal Clock) ( 16MHz )
#    Clock Divide                : 0
#    PLL Pre-divide (R)          : 7
#    PLL Divide 1   (N1)         : 47
#    PLL Divide 2   (N2)         : 0
#    PLL Output Divide (P)       : 5
#    PLL VCO (Freq Doubler) (D)  : s (VCO)
#    PLL Current                 : 10
#    PLL Bias                    : 2 (none)
#
#              ( FIN * ( N1 + 1 ) * ( D + 1 ))
#    Fout = -------------------------------------
#            (( R + 1 ) * ( N2 + 1 ) * ( P + 1 ))
#
# ex) pll e h 0 7 47 0 5 s 10 2 ( HSI 16MHz )
#
#               (16000000 * ( 47 + 1 ) * ( 0 + 1 ))
#    Fout = -------------------------------------------,    Fout = 16MHz
#               (( 7 + 1 ) * ( 0 + 1 ) * ( 5 + 1 ))
#
# ex) pll e e 0 7 95 0 5 s 10 2 ( HSE : High Speed External Clock = 8MHz )
#
#               (8000000 * ( 95 + 1 ) * ( 0 + 1 ))
#    Fout = -------------------------------------------,    Fout = 16MHz
#               (( 7 + 1 ) * ( 0 + 1 ) * ( 5 + 1 ))
#
#
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

# SCUCLK
send "cm scuclk"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

send "clkout m 0 e"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

# PLL configuration 16MHz 
print "pll e h 0 7 47 0 5 s 10 2"
send "pll e h 0 7 47 0 5 s 10 2"
sleep 0.5
send "lm"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

sleep 5
print "pll e e 0 7 95 0 5 s 10 2"
send "pll e e 0 7 95 0 5 s 10 2"
send "lm"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

end:
