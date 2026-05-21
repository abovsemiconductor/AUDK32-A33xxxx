# A. Description
#    A list of commands here configures FRT module and generates an overflow interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. FRT
#
# D. Note
#    An Overflow interrupt occurs after 4 minute.
#
# Please, read the User Manual of the specific chip for more details.
#
# FRT0
# 1. Channel                   : 0 (FRT0)
#
# 2. Clock                     : [ 0 m h 0 1]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 16MHz)
#    Source Divide             : 0 (No Divider available so default 0)
#    FRT Pre-Divide            : 1
#
# 3. Config                    : [ 0 i f o ]
#    Operation                 : i (Interrupt)
#    Mode                      : f (Free-Run)
#
# FRT0
send ""

send "cm FRT"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "clk 0 m h 0 1"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "config 0 i f"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

end:
