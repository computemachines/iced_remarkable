#! /bin/bash

function trap_ctrlc ()
{
    echo "Caught Ctrl-C ..."
#    ssh rm systemctl start xochitl
}

trap "trap_ctrlc" SIGINT

#ssh rm killall counter && echo "killed existing process"
#ssh rm systemctl stop xochitl
ssh rm ./counter && echo "Closed normally"
#echo "Starting xochitl ..."
#ssh rm systemctl start xochitl