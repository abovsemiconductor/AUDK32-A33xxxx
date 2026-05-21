# A. Description
#    A list of commands here configures FRT module and generates a match interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. FRT
#
# Please, read the User Manual of the specific chip for more details.
#
# FRT0
# 1. Channel                   : 0 (FRT0)
#
# 2. Clock                     : [ 0 m h 0 256 ]
#    Source                    : m (MCCR : Misc Clock)
#    MCCR Source               : h (HSI : High Speed Internal Clock - 16MHz)
#    Source Divide             : 0 (No Divider available so default 0)
#    FRT Pre-Divide            : 256
#
# 3. Config                    : [ 0 i m 0xF424 ]
#    Operation                 : i (Interrupt)
#    Mode                      : m (Match)
#    Match Counter Value       : 0xF424 (62500)
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

send "clk 0 m h 0 256"
expect {
    "<FRT> # "
    break
    timeout 5 goto end
}

send "config 0 i m 0xF424"
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
