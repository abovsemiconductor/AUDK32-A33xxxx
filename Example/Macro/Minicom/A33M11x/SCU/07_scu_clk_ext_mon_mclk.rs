# A. Description
#    A list of commands here configures SCU CLK and generates MCLK Clock Monitor Failure interrupt.
#
# B. Preparation
#    Set breakpoint by SW debugger.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. SCUCLK
#    2. PCU/GPIO
#
# D. Default Port
#    1. PCU    : PC9 (Clock Output)
#
# For more information, read a user's manual of the target device carefully.
#
# SCUCLK
# 1. MONITOR             : [ m e e m 3 ]
#    Clock Source        : m (MCLK : Main Clock)
#    Clock Recovery      : e (Enable)
#    Monitor Enable      : e (Enable)
#    Interrupt Mode      : m (Maskable)
#    Priority            : 3
#
# 2. Clkout              : [ m 0 e ]
#    Clock Source        : m (MCLK : Main Clock)
#    Clock Source Divide : 0
#    Clock Output Enable : e (Enable)
#
# 3. CLKEN               : [ h d ]
#    Clock Source        : h (HSI : High Speed Internal Clock)
#    Clock Enable        : d (Disable)
#
# PCU (PC9)
# 1. Port                : [ 2 9 a 3 ]
#    Port Group Number   : 2 (C group)
#    Pin Number          : 9
#    Pin Mode            : a (Alternative)
#    Pin Alt Number      : 3 (Clock Output)

# PCU (PC9)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 2 9 a 3"
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

send "monitor m e e m 3"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

sleep 5
send "clken h d"
expect {
    "<SCUCLK> # "
    break
    timeout 5 goto end
}

end:
