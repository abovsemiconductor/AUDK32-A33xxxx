# A. Description
#    A list of commands here configures timer1 and generates a period interrupt.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. Timer1
#
# D. Default Port
#    1. Timer10 : PB0 (Output)
#                 PA6 (Capture Input/External Clock Input)
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
# 3. Config                    : [ 0 i p h 1000 1000 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 1000
#    B Data                    : 1000
#
# Timer10
send ""

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

send "clk 0 p 16 999"
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
