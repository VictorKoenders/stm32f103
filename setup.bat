set OPENOCD_DIR=D:\Tools\OpenOCD

set PATH=%PATH%;%OPENOCD_DIR%\bin;%OPENOCD_DIR%
start openocd -s %OPENOCD_DIR%\share\openocd\scripts -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
start setup_itmdump.bat