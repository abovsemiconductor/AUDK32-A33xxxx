# A. Description
#    A list of commands here configures PWM and generates channel 0 ~ 7 match interrupts.
#
# B. Preparation
#    N/A
#
# C. Prerequisite Example (abov_example_config.h)
#    1. PWM
#    2. PCU/GPIO
#
# D. Default Port
#    1. PWM : PD0 (PWMA0) PE0 (PWMB0)
#           : PD1 (PWMA1) PE1 (PWMB1)
#           : PD2 (PWMA2) PE2 (PWMB2)
#           : PD3 (PWMA3) PE3 (PWMB3)
#           : PD4 (PWMA4) PE4 (PWMB4)
#           : PD5 (PWMA5) PE5 (PWMB5)
#           : PD6 (PWMA6) PE6 (PWMB6)
#           : PD7 (PWMA7) PE7 (PWMB7)
#
# For more information, read a user's manual of the target device carefully.
#
# PWM (0 ~ 7)
# 1. Channel                   : 0 / 1 / 2 / 3 / 4 / 5 / 6 / 7
#
# 2. Clock                     : [ 0 p e 249 16 ]
#    Channel                   : 0 / 1 / 2 / 3 / 4 / 5 / 6 / 7
#    Source                    : p (PCLK : Peripherl Clock)
#    Pre-Scaler Enable         : e (Enable)
#    Pre-Scaler Value          : 249 (Value + 1)
#    Clock Divider             : 16
#
# 3. Config                    : [ 0 i n d 4000 2000 ]
#    Channel                   : 0 / 1 / 2 / 3 / 4 / 5 / 6 / 7
#    Operation                 : i (Interrupt)
#    Port A Invert             : n (Not Inverted)
#    Sync with other channels  : d (Disable)
#    Period                    : 4000
#    Duty                      : 2000
#
# PCU (PDx)
# 1. Port Group                : 3 (PCU Port D)
#
# 2. Port                      : [ 3 0 a 1 ] [ 3 1 a 1 ] [ 3 2 a 1 ] [ 3 3 a 1 ] [ 3 4 a 1 ] [ 3 5 a 1 ] [ 3 6 a 1 ] [ 3 7 a 1 ]
#    Pin Number                : 0 / 1 / 2 / 3 / 4 / 5 / 6 / 7
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (PWM)
# 
# PCU (PEx)
# 1. Port Group                : 4 (PCU Port E)
#
# 2. Port                      : [ 4 0 a 1 ] [ 4 1 a 1 ] [ 4 2 a 1 ] [ 4 3 a 1 ] [ 4 4 a 1 ] [ 4 5 a 1 ] [ 4 6 a 1 ] [ 4 7 a 1 ]
#    Pin Number                : 0 / 1 / 2 / 3 / 4 / 5 / 6 / 7
#    Pin Mode                  : a (Alternative)
#    Alternative               : 1 (PWM)

# PCU (PDx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 0 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 1 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 2 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 3 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 4 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 5 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 6 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 3 7 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PCU (PEx)
send "port 4 0 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 1 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 2 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 3 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 4 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 5 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 6 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 4 7 a 1"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# PWM 
send "cm pwm"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 0 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 0 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 1"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 1 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 1 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 2"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 2 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 2 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 3"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 3 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 3 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 4"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 4 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 4 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 5"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 5 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 5 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 6"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 6 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 6 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "init 7"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "clk 7 p e 249 16"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "config 7 i n d 4000 2000"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "log on"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5 
send "start 1"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 2"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 3"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 4"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 5"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 6"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

sleep 0.5
send "start 7"
expect {
    "<PWM> # "
    break
    timeout 5 goto end
}

end:
