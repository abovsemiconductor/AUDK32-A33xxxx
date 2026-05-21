# A. Description
#    A list of commands here configures WDT and generates a reset.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. WDT
#
# For more information, read a user's manual of the target device carefully.
#
# WDT0
# 1. Channel                   : 0 (WDT 0)
#
# 2. Clock                     : [ 0 m h 0 256 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 16MHz)
#    Source Divide             : 0 (Not Available)
#    WDT Pre-Divide            : 256 (MCCR Clock) / 256
#
# 3. Config                    : [ 0 i m r 312500 0 ]
#    Operation                 : i (Interrupt)
#    Interrupt Mode            : m (Maskable)
#    Mode                      : r (Reset)
#    Inital Data               : 312500
#    Match Data                : 0
#
# WDT0
send ""

send "cm wdt"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 0 256"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

send "config 0 i m r 312500 0"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<WDT> # "
    break
    timeout 5 goto end
}

end:
