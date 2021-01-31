# SparkfunQuadStepDriver

SparkfunQuadStepDriver is a Rust driver library that allow you to use the "Sparkfun QuadStep" board.

# What is Sparkfun QuadStep?
![alt text](https://media.digikey.com/Photos/Sparkfun%20Elec%20%20Photos/MFG_ROB-10507.jpg)

It's a stepper motor driver that allow to drive at most 4 stepper motor.</br>
There are two possibility to control the motor, using the 4 channels(6 pin for each stepper motor) or using the BUS.</br>
The BUS, allow the board to control all 4 stepper motors using only 6 pins.</br>
A really important note is that if you wanna use the BUS, you must weld all bus enable on the rear of the board.</br>
For more info, check the datasheet.</br>

# How to use this library?


